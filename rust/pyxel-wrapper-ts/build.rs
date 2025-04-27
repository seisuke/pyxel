use std::fs;
use std::path::Path;
use syn::{parse_file, Attribute, Item};

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=build.rs");

    // 出力先ディレクトリ
    let pkg_dir = Path::new("pkg");
    let _ = fs::create_dir_all(pkg_dir);

    // 出力ファイルパス
    let dts_path = pkg_dir.join("pyxel_wrapper_ts.d.ts");
    let exported_functions_path = pkg_dir.join("EXPORTED_FUNCTIONS.txt");
    let generated_rs_path = Path::new("src/generated.rs");

    // 入力Rustソース
    let content = fs::read_to_string("src/lib.rs").expect("Failed to read src/lib.rs");
    let syntax = parse_file(&content).expect("Failed to parse Rust source");

    let tsfunctions = parse_tsfunctions(&syntax);

    // d.ts 出力
    let dts_content = generate_dts(&tsfunctions);
    fs::write(&dts_path, dts_content).expect("Failed to write d.ts");

    // EXPORTED_FUNCTIONS.txt 出力
    let exported_list = generate_exported_functions_list(&tsfunctions);
    fs::write(&exported_functions_path, exported_list).expect("Failed to write EXPORTED_FUNCTIONS.txt");

    // Rust generated.rs 出力
    let generated_content = generate_wrapper_rust(&tsfunctions);
    fs::write(&generated_rs_path, generated_content).expect("Failed to write generated.rs");

    println!("cargo:warning=Generated .d.ts at {:?}", dts_path);
    println!("cargo:warning=Generated EXPORTED_FUNCTIONS.txt at {:?}", exported_functions_path);
    println!("cargo:warning=Generated generated.rs at {:?}", generated_rs_path);
}

#[derive(Debug)]
struct TsFunction {
    name: String,
    args: Vec<(String, String)>,
    return_type: String,
}

fn parse_tsfunctions(syntax: &syn::File) -> Vec<TsFunction> {
    let mut functions = Vec::new();

    for item in &syntax.items {
        if let Item::Fn(func) = item {
            if has_tsfunction_attribute(&func.attrs) {
                let name = func.sig.ident.to_string();

                let mut args = Vec::new();
                for input in &func.sig.inputs {
                    if let syn::FnArg::Typed(arg) = input {
                        if let syn::Pat::Ident(ident) = &*arg.pat {
                            let arg_name = ident.ident.to_string();
                            let ty = rust_to_ts_type(&arg.ty);
                            args.push((arg_name, ty));
                        }
                    }
                }

                let return_type = match &func.sig.output {
                    syn::ReturnType::Default => "void".to_string(),
                    syn::ReturnType::Type(_, ty) => rust_to_ts_type(ty),
                };

                functions.push(TsFunction {
                    name,
                    args,
                    return_type,
                });
            }
        }
    }

    functions
}

fn has_tsfunction_attribute(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| attr.path().is_ident("tsfunction"))
}

fn rust_to_ts_type(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(typepath) => {
            if let Some(ident) = typepath.path.get_ident() {
                match ident.to_string().as_str() {
                    "i32" | "i64" | "f32" | "f64" => "number".to_string(),
                    "bool" => "boolean".to_string(),
                    "String" => "string".to_string(),
                    _ => "any".to_string(),
                }
            } else {
                "any".to_string()
            }
        }
        _ => "any".to_string(),
    }
}

fn generate_dts(functions: &[TsFunction]) -> String {
    let mut lines = Vec::new();
    for f in functions {
        let args = f
            .args
            .iter()
            .map(|(name, ty)| format!("{}: {}", name, ty))
            .collect::<Vec<_>>()
            .join(", ");
        lines.push(format!("export function {}({}): {};", f.name, args, f.return_type));
    }
    lines.join("\n")
}

fn generate_exported_functions_list(functions: &[TsFunction]) -> String {
    let mut exported = Vec::new();
    for f in functions {
        exported.push(format!("\"_{}\"", f.name)); // ここをクォートする
    }
    format!("[{}]", exported.join(","))
}

fn generate_wrapper_rust(functions: &[TsFunction]) -> String {
    let mut lines = Vec::new();
    lines.push("// Auto-generated wrapper functions".to_string());
    for f in functions {
        let args = f
            .args
            .iter()
            .map(|(name, _)| name.clone())
            .collect::<Vec<_>>()
            .join(", ");
        let params = f
            .args
            .iter()
            .map(|(name, ty)| format!("{}: {}", name, ts_to_rust_type(ty)))
            .collect::<Vec<_>>()
            .join(", ");

        lines.push(format!(
            "#[no_mangle]
pub extern \"C\" fn {name}({params}) {{
    crate::{name}({args})
}}",
            name = f.name,
            params = params,
            args = args
        ));
    }
    lines.join("\n\n")
}

fn ts_to_rust_type(ts_type: &str) -> &str {
    match ts_type {
        "number" => "i32",
        "boolean" => "bool",
        "string" => "*const u8", // 簡易対応（本来はstring handling必要）
        _ => "i32",
    }
}
