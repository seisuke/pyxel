use crate::template::{DTS_TEMPLATE, PYXEL_TS_TEMPLATE};
use anyhow::Result;
use pyxel_wrapper_ts_types::TsModule;
use std::env;
use std::fs;
use std::path::PathBuf;
use tera::{Context, Tera};

use crate::generate_wrapper_rust::generate_wrapper_rust;

pub fn generate() -> Result<()> {
    let project_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let pkg_dir = project_dir.join("pkg");
    fs::create_dir_all(&pkg_dir)?; // 念のため pkg ディレクトリを作成
    let src_dir = project_dir.join("src");
    let json_path = project_dir
        .parent()
        .unwrap()
        .join("pyxel-wrapper-ts/pkg/tsbind_types.json");
    let dts_path = pkg_dir.join("pyxel_wrapper_ts.d.ts");
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

    fs::write(&dts_path, generate_dts(&modules))?;
    fs::write(&rust_path, generate_wrapper_rust(&modules))?;
    fs::write(&ts_path, generate_pyxel_ts(&modules))?;
    fs::write(&exported_path, serde_json::to_string(&exported_names)?)?;

    println!(
        "✅ Generated: {:?},{:?},{:?},{:?}",
        dts_path, rust_path, ts_path, exported_path
    );

    Ok(())
}

fn resolve_ts_type(rust_type: &str, self_type: Option<&str>) -> String {
    if rust_type.starts_with("Option<") && rust_type.ends_with('>') {
        let inner = &rust_type[7..rust_type.len() - 1];
        let resolved = resolve_ts_type(inner, self_type);
        format!("{} | undefined", resolved)
    } else {
        match rust_type {
            "Color | Rgb24" => "number".to_string(),
            "i8" | "i16" | "i32" | "u8" | "u16" | "u32" | "f32" | "f64" => "number".to_string(),
            "bool" => "boolean".to_string(),
            "String" | "str" => "string".to_string(),
            "void" => "void".to_string(),
            "Self" => self_type
                .map(|s| s.to_string())
                .unwrap_or_else(|| "any".to_string()),
            _ => "any".to_string(),
        }
    }
}

pub fn generate_dts(modules: &[TsModule]) -> String {
    let mut tera = Tera::default();
    tera.add_raw_template("dts", DTS_TEMPLATE).unwrap();

    let mut modules = modules.to_owned();
    for module in &mut modules {
        for func in &mut module.functions {
            func.args
                .iter_mut()
                .for_each(|(_, typ)| *typ = resolve_ts_type(typ, None));
            func.return_type = resolve_ts_type(&func.return_type, None);
        }
        for class in &mut module.classes {
            for method in &mut class.methods {
                method
                    .args
                    .iter_mut()
                    .for_each(|(_, typ)| *typ = resolve_ts_type(typ, Some(&class.name)));
                method.return_type = resolve_ts_type(&method.return_type, Some(&class.name));
            }
        }
    }

    let mut context = Context::new();
    context.insert("modules", &modules);

    tera.render("dts", &context).expect("Failed to render dts")
}

pub fn generate_pyxel_ts(modules: &[TsModule]) -> String {
    let mut tera = Tera::default();
    tera.add_raw_template("pyxel_ts", PYXEL_TS_TEMPLATE)
        .unwrap();

    let mut modules = modules.to_owned();
    for module in &mut modules {
        for func in &mut module.functions {
            func.args
                .iter_mut()
                .for_each(|(_, typ)| *typ = resolve_ts_type(typ, None));
            func.return_type = resolve_ts_type(&func.return_type, None);
        }

        for class in &mut module.classes {
            for method in &mut class.methods {
                method.args.iter_mut().for_each(|arg| {
                    arg.1 = resolve_ts_type(&arg.1, Some(&class.name));
                });
                method.return_type = resolve_ts_type(&method.return_type, Some(&class.name));
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
