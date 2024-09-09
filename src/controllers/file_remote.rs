use reqwest;
use serde_json;
use tokio;

use crate::service::fetch_multiple_packages;

async fn parse_json(response: reqwest::Response) -> Result<serde_json::Value, reqwest::Error> {
    match response.json().await {
        Ok(json) => Ok(json),
        Err(e) => {
            eprintln!("Error al parsear el JSON: {}", e);
            Err(e)
        }
    }
}

struct Root {
    dependencies: serde_json::Value,
}

async fn fetch(url: &str) -> Result<Root, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let json = parse_json(response).await?;
    Ok(Root {
        dependencies: json,
    })
}

pub fn file_remote(path: &str, _rt: &tokio::runtime::Runtime) {
    let url = format!("https://raw.githubusercontent.com/{}", path);
    println!("Descargando archivo remoto: {}", url);

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        match fetch(&url).await {
            Ok(root) => {
                let dependencies = root.dependencies
                    .get("dependencies")
                    .unwrap_or(&serde_json::Value::Null);

                let dependencias: Vec<String> = dependencies
                    .as_object()
                    .unwrap()
                    .iter()
                    .map(|(key, value)| {
                        let version = value.as_str().unwrap_or("");
                        let cleaned_version = version.trim_start_matches('^').trim_matches('"');
                        format!("{}@{}", key, cleaned_version)
                    })
                    .collect();

                fetch_multiple_packages(dependencias).await
            }
            Err(e) => eprintln!("Error al obtener el archivo remoto: {}", e),
        }
    });
}
