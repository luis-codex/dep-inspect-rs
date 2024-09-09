use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Root {
    pub assets: Vec<Asset>,
    pub dependency_count: u32,
    pub dependency_sizes: Vec<DependencySize>,
    pub description: String,
    pub gzip: u32,
    pub name: String,
    pub repository: String,
    pub size: u32,
    pub version: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Asset {
    gzip: u32,
    name: String,
    size: u32,
    #[serde(rename = "type")]
    asset_type: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct DependencySize {
    approximate_size: u32,
    name: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct NpmDownloads {
    pub downloads: u32,
}

pub struct Stadistics {
    pub package_name: String,
    pub value: u32,
}
