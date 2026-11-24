use std::{env, fs, path::PathBuf};

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=README.md");
    println!("cargo:rerun-if-changed=docs");

    let mut pages = vec![PathBuf::from("README.md")];
    for entry in fs::read_dir("docs")? {
        let path = entry?.path();
        if path.extension().is_some_and(|extension| extension == "md") {
            pages.push(path);
        }
    }
    pages.sort();

    let mut tests = String::new();
    for path in pages {
        let name = path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_lowercase()
            .replace('-', "_");
        let text = fs::read_to_string(&path)?;
        let mut fence = None;
        let mut code = String::new();
        let mut number = 0;
        for line in text.lines() {
            if let Some(language) = line.strip_prefix("```") {
                if let Some(rust) = fence.take() {
                    if rust {
                        number += 1;
                        tests.push_str(&format!(
                            "documentation_example!({name}_{number}, (temporary_directory, name, location), {{\n{code}}});\n"
                        ));
                    }
                    code.clear();
                } else {
                    fence = Some(language.split(',').next() == Some("rust"));
                }
            } else if fence == Some(true) {
                code.push_str(line);
                code.push('\n');
            }
        }
        assert!(fence.is_none(), "Unclosed code fence in {}", path.display());
    }
    assert!(!tests.is_empty(), "No Rust documentation examples found");
    fs::write(
        PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("documentation.rs"),
        tests,
    )
}
