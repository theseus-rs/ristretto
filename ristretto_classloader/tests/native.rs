use convert_case::{Case, Casing};
use handlebars::Handlebars;
use ristretto_classloader::{runtime, Method, Result};
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Serialize)]
pub struct TemplateMethod {
    name: String,
    descriptor: String,
    function_name: String,
    body: String,
}

#[derive(Serialize)]
pub struct TemplateData {
    class_name: String,
    raw_class_name: String,
    methods: Vec<TemplateMethod>,
}

async fn get_native_methods(version: &str) -> Result<HashMap<String, Vec<Arc<Method>>>> {
    let (_java_home, _java_version, class_loader) = runtime::version_class_loader(version).await?;
    let class_path = class_loader.class_path();
    let class_names = class_path.class_names().await?;
    let mut native_methods = HashMap::new();
    for class_name in class_names {
        let class = class_loader.load(&class_name).await?;
        let mut methods = Vec::new();
        for method in class.methods() {
            if method.is_native() {
                methods.push(method.clone());
            }
        }
        methods.sort_by(|a, b| a.name().cmp(b.name()));
        if !methods.is_empty() {
            native_methods.insert(class.name().to_string(), methods);
        }
    }
    Ok(native_methods)
}

fn method_function_name(method: &Method) -> String {
    match method.name() {
        "initIDs" => "init_ids".to_string(),
        method_name => method_name.to_string().to_case(Case::Snake),
    }
}

fn method_body(method: &Method) -> String {
    match method.name() {
        "initIDs" | "registerNatives" => "Ok(None)".to_string(),
        _ => "todo!()".to_string(),
    }
}

#[expect(dead_code)]
async fn write_native(version: &str) -> Result<()> {
    let native_methods = get_native_methods(version).await?;
    let mut classes: Vec<String> = native_methods.keys().map(ToString::to_string).collect();
    classes.sort();

    let cargo_manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let template_path = cargo_manifest
        .join("tests/template.hbs")
        .to_string_lossy()
        .to_string();
    let native_methods_path = cargo_manifest
        .join("../ristretto_vm/src/native_methods")
        .to_string_lossy()
        .to_string();

    let mod_file_name = format!("{native_methods_path}/mod.rs");
    let mut mod_file = File::create(mod_file_name)?;

    for class_name in classes {
        let methods = native_methods.get(&class_name).cloned().unwrap_or_default();

        let data = TemplateData {
            class_name: class_name.replace('/', ".").clone(),
            raw_class_name: class_name.clone(),
            methods: methods
                .iter()
                .map(|method| TemplateMethod {
                    name: method.name().to_string(),
                    descriptor: method.descriptor().to_string(),
                    function_name: method_function_name(method),
                    body: method_body(method),
                })
                .collect(),
        };
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_file("template", template_path.as_str())
            .unwrap();

        let file_name = class_name
            .replace(['/', '$'], "_")
            .replace("__", "_")
            .to_lowercase();
        mod_file.write_all(format!("mod {file_name};\n").as_bytes())?;
        let output_file_name = format!("{native_methods_path}/{file_name}.rs");
        let mut output_file = File::create(output_file_name)?;
        let _ = handlebars.render_to_write("template", &data, &mut output_file);
    }

    mod_file.write_all("\nmod properties;\n".as_bytes())?;
    mod_file.write_all("\nmod registry;\n".as_bytes())?;
    mod_file.write_all("\npub use registry::MethodRegistry;\n".as_bytes())?;

    Ok(())
}

#[tokio::test]
async fn test_native_methods() -> Result<()> {
    // Enable to generate native method stubs
    // write_native("8.432.06.1").await?;
    // write_native("11.0.25.9.1").await?;
    // write_native("17.0.12.7.1").await?;
    // write_native("18.0.2.9.1").await?;
    // write_native("19.0.2.7.1").await?;
    // write_native("20.0.2.10.1").await?;
    // write_native("21.0.5.11.1").await?;
    // write_native("22.0.2.9.1").await?;
    // write_native("23.0.1.8.1").await?;
    Ok(())
}
