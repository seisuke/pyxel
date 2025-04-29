use anyhow::Result;
use pyxel_wrapper_ts_types::TsModule;
use std::env;
use std::fs;
use std::path::PathBuf;

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
    let modules: Vec<TsModule> = if json_path.exists() {
        let data = fs::read_to_string(&json_path)?;
        serde_json::from_str(&data)?
    } else {
        vec![]
    };

    println!(
        "Generating bindings from:\n{}",
        serde_json::to_string_pretty(&modules)?
    );

    fs::write(&dts_path, generate_dts(&modules))?;
    fs::write(&rust_path, generate_wrapper_rust(&modules))?;

    println!("✅ Generated: {:?} and {:?}", dts_path, rust_path);

    Ok(())
}

fn generate_dts(modules: &[TsModule]) -> String {
    let mut lines = Vec::new();
    for m in modules {
        lines.push(format!("declare module \"{}\" {{", m.name));
        for f in &m.functions {
            let args = f
                .args
                .iter()
                .map(|(n, t)| format!("{}: {}", n, t))
                .collect::<Vec<_>>()
                .join(", ");
            lines.push(format!(
                "  export function {}({}): {};",
                f.name, args, f.return_type
            ));
        }
        for class in &m.classes {
            lines.push(format!("  export class {} {{", class.name));
            for method in &class.methods {
                let args = method
                    .args
                    .iter()
                    .map(|(n, t)| format!("{}: {}", n, t))
                    .collect::<Vec<_>>()
                    .join(", ");
                lines.push(format!(
                    "    {}({}): {};",
                    method.name, args, method.return_type
                ));
            }
            lines.push("  }".to_string());
        }
        lines.push("}".to_string());
    }
    lines.join("\n")
}
