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

        let mut current_class = CURRENT_CLASS.lock().unwrap();
        *current_class = None;
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
        let current_module = CURRENT_MODULE.lock().unwrap();
        let module_name = current_module.clone().expect("No current module set");

        if let Some(module) = modules.iter_mut().find(|m| m.name == module_name) {
            if !module.classes.iter().any(|c| c.name == class_name) {
                module.classes.push(TsClass {
                    name: class_name.clone(),
                    methods: Vec::new(),
                });
            }
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
            let class_name = ident.to_string();

            let mut modules = MODULES.lock().unwrap();
            let current_module = CURRENT_MODULE.lock().unwrap();
            let module_name = current_module.clone().expect("No current module set");

            if let Some(module) = modules.iter_mut().find(|m| m.name == module_name) {
                if !module.classes.iter().any(|c| c.name == class_name) {
                    module.classes.push(TsClass {
                        name: class_name.clone(),
                        methods: Vec::new(),
                    });
                }
            }

            let mut current_class = CURRENT_CLASS.lock().unwrap();
            *current_class = Some(class_name);
        }
    }

    save_tsbind_types();
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
        let current_module = CURRENT_MODULE.lock().unwrap();
        let module_name = current_module.clone().expect("No current module set");
        let module = modules
            .iter_mut()
            .find(|m| m.name == module_name)
            .expect("Module not found");

        let current_class = CURRENT_CLASS.lock().unwrap();

        if let Some(class_name) = &*current_class {
            if let Some(class) = module.classes.iter_mut().find(|c| c.name == *class_name) {
                class.methods.push(ts_func);
            }
        } else {
            module.functions.push(ts_func);
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
            if let syn::FnArg::Typed(pat_type) = input {
                if let syn::Pat::Ident(ident) = &*pat_type.pat {
                    let name = ident.ident.to_string();
                    let ty = resolve_type(&*pat_type.ty);
                    return Some((name, ty));
                }
            }
            None
        })
        .collect()
}

fn extract_return_type(output: &syn::ReturnType) -> String {
    match output {
        syn::ReturnType::Default => "void".to_string(),
        syn::ReturnType::Type(_, ty) => resolve_type(ty),
    }
}

fn resolve_type(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Reference(reference) => resolve_type(&reference.elem),
        syn::Type::Path(type_path) => {
            let segments = &type_path.path.segments;
            let segment = segments.last().unwrap();
            let ident = segment.ident.to_string();

            if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                if (ident == "Option") || (ident == "Vec") {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        let inner = resolve_type(inner_ty);
                        return format!("{}<{}>", ident, inner);
                    }
                }
            }

            ident
        }
        _ => "any".to_string(),
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
