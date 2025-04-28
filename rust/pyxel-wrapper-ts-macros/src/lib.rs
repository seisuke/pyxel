extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use once_cell::sync::Lazy;
use pyxel_wrapper_ts_types::{TsClass, TsFunction, TsModule};
use std::sync::Mutex;

static MODULES: Lazy<Mutex<Vec<TsModule>>> = Lazy::new(|| Mutex::new(Vec::new()));
static CURRENT_MODULE: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));
static CURRENT_CLASS: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));

fn save_tsbind_types() {
    use std::env;
    use std::fs;
    use std::path::Path;

    let modules = MODULES.lock().unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = Path::new(&manifest_dir).join("pkg/tsbind_types.json");

    let json = serde_json::to_string_pretty(&*modules).expect("Failed to serialize modules");
    let _ = fs::write(&path, &json);
}

fn get_or_create_current_module(modules: &mut Vec<TsModule>) -> &mut TsModule {
    let module_name = {
        let current = CURRENT_MODULE.lock().unwrap();
        current.clone().expect("No current module set")
    };

    if let Some(i) = modules.iter().position(|m| m.name == module_name) {
        &mut modules[i]
    } else {
        modules.push(TsModule {
            name: module_name.clone(),
            functions: Vec::new(),
            classes: Vec::new(),
        });
        modules.last_mut().unwrap()
    }
}

fn parse_tsmodule_name(attr: TokenStream) -> Option<String> {
    if attr.is_empty() {
        return None;
    }

    let expr: syn::Expr =
        syn::parse2(attr.into()).expect("Invalid syntax for #[tsmodule(name = \"...\")]");

    if let syn::Expr::Assign(assign) = expr {
        if let syn::Expr::Path(path) = *assign.left {
            if path.path.is_ident("name") {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit_str),
                    ..
                }) = *assign.right
                {
                    return Some(lit_str.value());
                }
            }
        }
    }

    panic!("Expected #[tsmodule(name = \"...\")]");
}

#[proc_macro_attribute]
pub fn tsmodule(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as syn::ItemMod);
    let mod_name = parse_tsmodule_name(attr).unwrap_or_else(|| ast.ident.to_string());

    {
        let mut modules = MODULES.lock().unwrap();
        if modules.iter().all(|m| m.name != mod_name) {
            modules.push(TsModule {
                name: mod_name.clone(),
                functions: Vec::new(),
                classes: Vec::new(),
            });
        }
    }

    {
        let mut current_module = CURRENT_MODULE.lock().unwrap();
        *current_module = Some(mod_name);
    }

    save_tsbind_types();

    TokenStream::from(quote! { #ast })
}

#[proc_macro_attribute]
pub fn tsclass(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as syn::ItemStruct);
    let class_name = ast.ident.to_string();

    {
        let mut modules = MODULES.lock().unwrap();
        let module = get_or_create_current_module(&mut modules);

        if !module.classes.iter().any(|c| c.name == class_name) {
            module.classes.push(TsClass {
                name: class_name.clone(),
                methods: Vec::new(),
            });
        }
    }

    {
        let mut current_class = CURRENT_CLASS.lock().unwrap();
        *current_class = Some(class_name);
    }

    save_tsbind_types();

    TokenStream::from(quote! { #ast })
}

#[proc_macro_attribute]
pub fn tsimpl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as syn::ItemImpl);

    if let syn::Type::Path(type_path) = ast.self_ty.as_ref() {
        if let Some(ident) = type_path.path.get_ident() {
            let mut current_class = CURRENT_CLASS.lock().unwrap();
            *current_class = Some(ident.to_string());
        }
    }

    TokenStream::from(quote! { #ast })
}

#[proc_macro_attribute]
pub fn tsfunction(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as syn::ItemFn);

    let func_name = ast.sig.ident.to_string();
    let mut args = Vec::new();
    for input in &ast.sig.inputs {
        if let syn::FnArg::Typed(pat) = input {
            if let syn::Pat::Ident(ident) = &*pat.pat {
                let arg_name = ident.ident.to_string();
                let arg_type = "number".to_string(); // 仮
                args.push((arg_name, arg_type));
            }
        }
    }

    let return_type = match &ast.sig.output {
        syn::ReturnType::Default => "void".to_string(),
        syn::ReturnType::Type(_, _) => "any".to_string(), // 仮
    };

    {
        let mut modules = MODULES.lock().unwrap();
        let current_class = CURRENT_CLASS.lock().unwrap();
        let module = get_or_create_current_module(&mut modules);

        if let Some(ref class_name) = *current_class {
            if let Some(class) = module.classes.iter_mut().find(|c| c.name == *class_name) {
                class.methods.push(TsFunction {
                    name: func_name,
                    args,
                    return_type,
                });
            } else {
                // 万一クラスがなければ作成
                module.classes.push(TsClass {
                    name: class_name.clone(),
                    methods: vec![TsFunction {
                        name: func_name,
                        args,
                        return_type,
                    }],
                });
            }
        } else {
            module.functions.push(TsFunction {
                name: func_name,
                args,
                return_type,
            });
        }
    }

    save_tsbind_types();

    TokenStream::from(quote! { #ast })
}
