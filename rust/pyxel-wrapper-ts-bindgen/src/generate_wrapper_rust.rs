use pyxel_wrapper_ts_types::{TsFunction, TsModule};

pub fn generate_wrapper_rust(modules: &[TsModule]) -> String {
    let mut code = String::new();
    code.push_str("// Auto-generated wrapper functions\n\n");

    for module in modules {
        for function in &module.functions {
            let fn_code = generate_function(&module.name, function);
            code.push_str(&fn_code);
            code.push_str("\n");
        }
        for class in &module.classes {
            for method in &class.methods {
                let fn_code = generate_class_method(&module.name, &class.name, method);
                code.push_str(&fn_code);
                code.push_str("\n");
            }
        }
    }

    code
}

fn generate_function(module_name: &str, function: &TsFunction) -> String {
    let fn_name = &function.name;
    let args = generate_rust_args(&function.args);
    let args_call = generate_rust_arg_names(&function.args);

    format!(
        "#[no_mangle]
pub extern \"C\" fn {fn_name}({args}) {{
    crate::{module_name}::{fn_name}({args_call})
}}
"
    )
}

fn generate_class_method(module_name: &str, class_name: &str, method: &TsFunction) -> String {
    let method_name = &method.name;
    let args = generate_rust_args(&method.args);
    let args_call = generate_rust_arg_names(&method.args);

    format!(
        "#[no_mangle]
pub extern \"C\" fn {class}_{method_name}({args}) {{
    crate::{module_name}::{class}::{method_name}({args_call})
}}
",
        class = class_name,
    )
}

fn generate_rust_args(args: &[(String, String)]) -> String {
    args.iter()
        .map(|(name, _typ)| format!("{name}: i32")) // 仮にすべてi32
        .collect::<Vec<_>>()
        .join(", ")
}

fn generate_rust_arg_names(args: &[(String, String)]) -> String {
    args.iter()
        .map(|(name, _typ)| name.clone())
        .collect::<Vec<_>>()
        .join(", ")
}
