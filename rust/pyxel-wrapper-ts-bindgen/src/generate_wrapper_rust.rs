use crate::template::WRAPPER_RS_TEMPLATE;
use pyxel_wrapper_ts_types::TsModule;
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
        for function in &mut module.functions {
            function.meta.insert(
                "arg_cast_lines".to_string(),
                arg_cast_rust_lines(&function.args).into(),
            );
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
        "String" | "str" => "*const u8", // pointers for strings
        _ => "i32",                      // default  fallback
    }
}

fn arg_cast_rust_lines(args: &[(String, String)]) -> String {
    let mut lines = vec![];

    for (name, typ) in args {
        match typ.as_str() {
            "String" | "str" => {
                lines.push(format!(
                    r#"let c_str = unsafe {{ CStr::from_ptr({} as *const i8) }};
let {} = match c_str.to_str() {{ Ok(s) => s, Err(_) => return, }};"#,
                    name, name
                ));
            }
            "Option<bool>" => {
                lines.push(format!(
                    "let {} = match {} {{ 0 => Some(false), 1 => Some(true), _ => None }};",
                    name, name
                ));
            }
            "u8" | "u16" | "u32" | "i8" | "i16" => {
                lines.push(format!(
                    "let {} = match {}.try_into() {{ Ok(v) => v, Err(_) => return, }};",
                    name, name
                ));
            }
            "Color" | "Rgb24" => {
                lines.push(format!(
                    "let {} = match {}.try_into() {{ Ok(v) => v, Err(_) => return, }};",
                    name, name
                ));
            }
            _ => {}
        }
    }

    lines.join("\n")
}
