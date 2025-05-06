extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, ExprLit, Lit, Meta};

use once_cell::sync::Lazy;
use pyxel_wrapper_ts_types::{TsClass, TsFunction, TsModule};
use serde_json::json;
use std::collections::HashMap;
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
pub fn tsfunction(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as syn::ItemFn);

    let func_name = ast.sig.ident.to_string();
    let args = extract_args(&ast.sig);
    let return_type = extract_return_type(&ast.sig.output);
    let body = parse_body_attr(attr.clone());
    let meta = derive_function_meta(&func_name, &args);

    let ts_func = TsFunction {
        name: func_name,
        args,
        return_type,
        meta,
        body,
    };

    {
        let mut modules = MODULES.lock().unwrap();
        let current_class = CURRENT_CLASS.lock().unwrap();
        let module = get_or_create_current_module(&mut modules);

        if let Some(ref class_name) = *current_class {
            // ✅ クラスに既に存在するかチェックしてから methods に push
            if let Some(class) = module.classes.iter_mut().find(|c| c.name == *class_name) {
                if !class
                    .methods
                    .iter()
                    .any(|m| m.name == ts_func.name && m.args == ts_func.args)
                {
                    class.methods.push(ts_func);
                }
            } else {
                // ✅ クラスが未登録だった場合もここで push
                module.classes.push(TsClass {
                    name: class_name.clone(),
                    methods: vec![ts_func],
                });
            }
        } else {
            // ✅ CURRENT_CLASS が None の場合に限り、モジュール直下の関数として登録
            if !module
                .functions
                .iter()
                .any(|f| f.name == ts_func.name && f.args == ts_func.args)
            {
                module.functions.push(ts_func);
            }
        }
    }

    save_tsbind_types();
    TokenStream::from(quote! { #ast })
}

fn parse_body_attr(attr: TokenStream) -> String {
    if attr.is_empty() {
        return String::new();
    }

    let meta: Meta = match syn::parse(attr.into()) {
        Ok(m) => m,
        Err(_) => return String::new(),
    };

    if let Meta::NameValue(nv) = meta {
        if nv.path.is_ident("body") {
            if let Expr::Lit(ExprLit {
                lit: Lit::Str(s), ..
            }) = nv.value
            {
                return s.value();
            }
        }
    }

    String::new()
}

fn extract_args(sig: &syn::Signature) -> Vec<(String, String)> {
    sig.inputs
        .iter()
        .filter_map(|input| {
            if let syn::FnArg::Typed(pat) = input {
                if let syn::Pat::Ident(ident) = &*pat.pat {
                    let name = ident.ident.to_string();

                    let ty = match &*pat.ty {
                        syn::Type::Path(type_path) => parse_type_path(type_path),
                        syn::Type::Reference(reference) => {
                            if let syn::Type::Path(type_path) = &*reference.elem {
                                parse_type_path(type_path)
                            } else {
                                "any".to_string()
                            }
                        }
                        _ => "any".to_string(),
                    };

                    return Some((name, ty));
                }
            }
            None
        })
        .collect()
}

fn parse_type_path(type_path: &syn::TypePath) -> String {
    let segments = &type_path.path.segments;

    if segments.len() == 1 && segments[0].ident == "Option" {
        if let syn::PathArguments::AngleBracketed(ref args) = segments[0].arguments {
            if let Some(syn::GenericArgument::Type(syn::Type::Path(inner_ty_path))) =
                args.args.first()
            {
                let inner = inner_ty_path
                    .path
                    .segments
                    .last()
                    .unwrap()
                    .ident
                    .to_string();
                format!("Option<{}>", inner)
            } else {
                "Option<any>".to_string()
            }
        } else {
            "Option<any>".to_string()
        }
    } else {
        segments.last().unwrap().ident.to_string()
    }
}

fn extract_return_type(output: &syn::ReturnType) -> String {
    match output {
        syn::ReturnType::Default => "void".to_string(),
        syn::ReturnType::Type(_, ty) => match &**ty {
            syn::Type::Path(type_path) => type_path.path.segments.last().unwrap().ident.to_string(),
            _ => "any".to_string(),
        },
    }
}

fn derive_function_meta(
    name: &str,
    args: &[(String, String)],
) -> HashMap<String, serde_json::Value> {
    let mut meta = HashMap::new();

    if args.iter().any(|(_, ty)| ty == "String" || ty == "str") {
        meta.insert("requires_cwrap".to_string(), json!(true));
    }

    if name == "init" || name == "load" {
        meta.insert("is_async".to_string(), json!(true));
    }

    meta
}
