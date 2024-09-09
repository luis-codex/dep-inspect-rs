use serde_json;
use tokio::runtime::Runtime;
use crate::service::fetch_multiple_packages;

pub fn file_package_dependencies(file: String, rt: &Runtime) {
    let contenido = std::fs::read_to_string(file).expect("No se pudo leer el archivo");

    let json: serde_json::Value = serde_json
        ::from_str(&contenido)
        .expect("Error al parsear el JSON");

    let dependencias = json.get("dependencies").unwrap_or(&serde_json::Value::Null);

    let dependencias: Vec<String> = dependencias
        .as_object()
        .unwrap()
        .iter()
        .map(|(key, value)| {
            let version = value.as_str().unwrap_or("");
            let cleaned_version = version.trim_start_matches('^').trim_matches('"');
            format!("{}@{}", key, cleaned_version)
        })
        .collect();

    rt.block_on(fetch_multiple_packages(dependencias));
}
