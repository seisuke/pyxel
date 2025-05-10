use crate::template::PYXEL_TS_TEMPLATE;
use anyhow::Result;
use pyxel_wrapper_ts_types::{TsFunction, TsModule};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tera::{Context, Tera, Value};

use crate::generate_wrapper_rust::generate_wrapper_rust;

pub fn write_and_format<P: AsRef<std::path::Path>>(path: P, contents: &str) {
    fs::write(&path, contents).expect("Failed to write file");

    let status = Command::new("rustfmt")
        .arg(path.as_ref())
        .status()
        .expect("Failed to run rustfmt");

    if !status.success() {
        eprintln!("rustfmt failed on {:?}", path.as_ref());
    }
}

pub fn generate() -> Result<()> {
    let project_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let pkg_dir = project_dir.join("pkg");
    fs::create_dir_all(&pkg_dir)?; // 念のため pkg ディレクトリを作成
    let src_dir = project_dir.join("src");
    let json_path = project_dir
        .parent()
        .unwrap()
        .join("pyxel-wrapper-ts/pkg/tsbind_types.json");
    let rust_path = src_dir.join("generated.rs");
    let ts_path = pkg_dir.join("pyxel.ts");
    let exported_path = pkg_dir.join("EXPORTED_FUNCTIONS.txt");
    let modules: Vec<TsModule> = if json_path.exists() {
        let data = fs::read_to_string(&json_path)?;
        serde_json::from_str(&data)?
    } else {
        vec![]
    };
    let exported_names = collect_exported_function_names(&modules);

    println!(
        "Generating bindings from:\n{}",
        serde_json::to_string_pretty(&modules)?
    );

    let rendered = generate_wrapper_rust(&modules);
    write_and_format(&rust_path, &rendered);
    fs::write(&ts_path, generate_pyxel_ts(&modules))?;
    fs::write(&exported_path, serde_json::to_string(&exported_names)?)?;

    println!(
        "✅ Generated: {:?},{:?},{:?}",
        rust_path, ts_path, exported_path
    );

    Ok(())
}

fn resolve_ts_type(rust_type: &str, self_type: Option<&str>) -> String {
    match rust_type {
        "Color" | "Rgb24" => rust_type.to_string(),
        "i8" | "i16" | "i32" | "u8" | "u16" | "u32" | "f32" | "f64" => "number".to_string(),
        "usize" => "number".to_string(),
        "bool" => "boolean".to_string(),
        "String" | "str" => "string".to_string(),
        "void" => "void".to_string(),
        "Self" => self_type
            .map(|s| s.to_string())
            .unwrap_or_else(|| "any".to_string()),
        _ => rust_type.to_string(),
    }
}

fn generate_ts_decl_args(args: &[(String, String)], self_type: Option<&str>) -> String {
    args.iter()
        .map(|(name, ty)| {
            if ty.starts_with("Option<") && ty.ends_with('>') {
                let inner = ty.trim_start_matches("Option<").trim_end_matches('>');
                format!("{}?: {}", name, resolve_ts_type(inner, self_type))
            } else {
                format!("{}: {}", name, resolve_ts_type(ty, self_type))
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn generate_wrapper_call(func: &TsFunction) -> String {
    let call_args = func
        .args
        .iter()
        .map(|(n, _)| n.clone())
        .collect::<Vec<_>>()
        .join(", ");
    if !func.body.is_empty() {
        let mut result = func.body.clone();
        if func.meta.get("requires_cwrap").is_some() {
            result.push_str(&format!("\n_fns.{}({});", func.name, call_args));
        } else {
            result.push_str(&format!(
                "\nreturn (instance as any)._{}({});",
                func.name, call_args
            ));
        }
        result
    } else {
        format!("return (instance as any)._{}({});", func.name, call_args)
    }
}

fn generate_proxy_definefn(func: &TsFunction) -> Option<String> {
    if !func.meta.contains_key("requires_cwrap") {
        return None;
    }
    let return_type = if func.return_type == "void" {
        "null".to_string()
    } else {
        format!("\"{}\"", func.return_type)
    };
    let arg_types: Vec<String> = func
        .args
        .iter()
        .map(|(_, typ)| {
            if typ.contains("Option<bool>") || typ.contains("bool") {
                "'number'".to_string()
            } else if typ == "str" {
                "'string'".to_string()
            } else {
                "'number'".to_string()
            }
        })
        .collect();
    Some(format!(
        "if (prop === \"{}\") {{\n  return target.defineFn(\"{}\", {}, [{}]);\n}}",
        func.name,
        func.name,
        return_type,
        arg_types.join(", ")
    ))
}

fn resolve_ts_filter(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    match value {
        Value::String(rust_type) => Ok(Value::String(resolve_ts_type(rust_type, None))),
        _ => Err("Expected string".into()),
    }
}

pub fn generate_pyxel_ts(modules: &[TsModule]) -> String {
    let mut tera = Tera::default();
    tera.register_filter("resolve_ts", resolve_ts_filter);
    tera.add_raw_template("pyxel_ts", PYXEL_TS_TEMPLATE)
        .unwrap();

    let mut modules = modules.to_owned();
    for module in &mut modules {
        for func in &mut module.functions {
            if func.return_type.ends_with("List") {
                func.meta.insert("getter_array".into(), true.into());
                continue;
            }

            let ts_decl_args = generate_ts_decl_args(&func.args, None);
            func.meta.insert("ts_decl_args".into(), ts_decl_args.into());

            let wrapper_call = generate_wrapper_call(func);
            func.meta.insert("wrapper_call".into(), wrapper_call.into());

            if let Some(proxy) = generate_proxy_definefn(func) {
                func.meta.insert("proxy_definefn".into(), proxy.into());
            }
        }

        for class in &mut module.classes {
            for method in &mut class.methods {
                method.return_type = resolve_ts_type(&method.return_type, Some(&class.name));

                if !method.meta.contains_key("ts_decl_args") {
                    let ts_decl_args = generate_ts_decl_args(&method.args, Some(&class.name));
                    method
                        .meta
                        .insert("ts_decl_args".into(), ts_decl_args.into());
                }
            }
        }
    }

    let mut context = Context::new();
    context.insert("modules", &modules);
    tera.render("pyxel_ts", &context)
        .expect("Failed to render pyxel.ts")
}

fn collect_exported_function_names(modules: &[TsModule]) -> Vec<String> {
    let mut names = Vec::new();

    for m in modules {
        for func in &m.functions {
            names.push(format!("_{}", func.name));
        }

        for class in &m.classes {
            for method in &class.methods {
                let symbol_name = if method.name == "new" {
                    format!("_{}_new", class.name)
                } else {
                    format!("_{}_{}", class.name, method.name)
                };
                names.push(symbol_name);
            }
        }
    }

    names
}
