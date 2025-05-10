use crate::template::WRAPPER_RS_TEMPLATE;
use pyxel_wrapper_ts_types::{TsFunction, TsModule};
use std::collections::HashMap;
use tera::{Context, Function, Result as TeraResult, Tera, Value};

pub fn generate_wrapper_rust(modules: &[TsModule]) -> String {
    let mut tera = Tera::default();
    tera.add_raw_template("wrapper", WRAPPER_RS_TEMPLATE)
        .expect("Failed to parse template");
    tera.register_function("arg_type_rust", ArgTypeRustFunction);

    let mut context = Context::new();
    let mut enriched_modules = modules.to_vec();
    for module in &mut enriched_modules {
        for func in &mut module.functions {
            enrich_function_metadata(func);
            if func.name == "images" {
                func.return_type = "*mut ImageList".to_string();
                let custom_return = format!(
                    "Box::into_raw(Box::new(crate::{}::{}()))",
                    module.name, func.name
                );
                func.meta
                    .insert("custom_return".to_string(), custom_return.into());
            }
        }
        for class in &mut module.classes {
            for method in &mut class.methods {
                enrich_function_metadata(method);
                if class.name == "ImageList" && method.name == "get" {
                    method.return_type = "*mut Image".to_string();
                }
            }
        }
    }
    context.insert("modules", &enriched_modules);

    tera.render("wrapper", &context)
        .expect("Template render error")
}

#[derive(Debug)]
struct ArgTypeRustFunction;

impl Function for ArgTypeRustFunction {
    fn call(&self, args: &HashMap<String, Value>) -> TeraResult<Value> {
        let typ = args
            .get("typ")
            .or_else(|| args.get("_0"))
            .unwrap()
            .as_str()
            .unwrap();
        Ok(Value::String(arg_type_rust(typ).to_string()))
    }

    fn is_safe(&self) -> bool {
        true
    }
}

fn arg_type_rust(typ: &str) -> &str {
    match typ {
        "Color" | "Rgb24" => "i32",
        "i8" | "i16" | "i32" | "u8" | "u16" | "u32" => "i32",
        "f32" | "f64" => "f32",
        "bool" => "bool",
        "String" | "str" => "*const u8",
        _ => "i32",
    }
}

fn enrich_function_metadata(func: &mut TsFunction) {
    func.meta.insert(
        "arg_cast_lines".to_string(),
        arg_cast_rust_lines(&func.args, &func.return_type).into(),
    );

    if func.return_type.ends_with("List") {
        func.meta.insert("is_vec".to_string(), true.into());
    }
}

fn arg_cast_rust_lines(args: &[(String, String)], return_type: &str) -> String {
    let mut lines = vec![];

    let fallback = if return_type == "Self" {
        "return std::ptr::null_mut()".to_string()
    } else {
        format!("return (),")
    };

    for (name, typ) in args {
        match typ.as_str() {
            "String" | "str" => lines.push(format!(
                r#"let c_str = unsafe {{ CStr::from_ptr({} as *const i8) }};
let {} = match c_str.to_str() {{ Ok(s) => s, Err(_) => {} }};"#,
                name, name, fallback
            )),
            "Option<bool>" => lines.push(format!(
                "let {} = match {} {{ 0 => Some(false), 1 => Some(true), _ => None }};",
                name, name
            )),
            "u8" | "u16" | "u32" | "i8" | "i16" | "Color" | "Rgb24" => {
                lines.push(format!(
                    "let {} = match {}.try_into() {{ Ok(v) => v, Err(_) => {} }};",
                    name, name, fallback
                ));
            }
            _ => {}
        }
    }

    lines.join("\n")
}
