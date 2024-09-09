use futures::future::join_all;
use reqwest::Client;
use colored::Colorize;
use tokio::sync::Semaphore;
use std::sync::Arc;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use crate::styles::{ CYAN, ERROR, PURPLE };
use crate::ui::{ gzip_log_comparative, log_body, log_header };
use crate::types::{ NpmDownloads, Root, Stadistics };
use crate::utils::colorize_text;
use std::time::Instant;
use tokio::task;

pub async fn fetch_size_of_package(
    package: &str,
    client: &reqwest::Client
) -> Result<Root, String> {
    let url = format!("https://bundlephobia.com/api/size?package={}", package);
    let response = client
        .get(&url)
        .header("User-Agent", "reqwest")
        .send().await
        .map_err(|e| format!("Request error: {}\n", e))?;

    if response.status() == reqwest::StatusCode::OK {
        response.json().await.map_err(|e| format!("bundlephobia JSON error: {}\n", e))
    } else {
        Err("bundlephobia not-found\n".to_string())
    }
}

async fn fetch_npm_downloads(package: &str, client: &reqwest::Client) -> Stadistics {
    const BASE_PATH: &str = "https://api.npmjs.org/downloads/point/last-week/";
    let url = format!("{}{}", BASE_PATH, package);
    match client.get(&url).send().await {
        Ok(response) =>
            match response.json::<NpmDownloads>().await {
                Ok(downloads) =>
                    Stadistics {
                        package_name: package.to_string(),
                        value: downloads.downloads,
                    },
                Err(_) =>
                    Stadistics {
                        package_name: package.to_string(),
                        value: 0,
                    },
            }
        Err(_) =>
            Stadistics {
                package_name: package.to_string(),
                value: 0,
            },
    }
}

pub async fn fetch_package(packages: Vec<String>, client: &Client) {
    println!("\n\t{} comparative\n", colorize_text("DOWNLOADS", CYAN));
    let futures: Vec<_> = packages
        .iter()
        .map(|package| {
            let client = client.clone();
            let package = package.clone();
            task::spawn(async move { fetch_npm_downloads(&package, &client).await })
        })
        .collect();

    let results = join_all(futures).await;

    let mut results: Vec<Stadistics> = results.into_iter().filter_map(Result::ok).collect();
    results.sort_unstable_by(|a, b| b.value.cmp(&a.value));
    gzip_log_comparative(results, true);
}

pub async fn fetch_multiple_packages(packages: Vec<String>) {
    log_header();
    let start = Instant::now();

    let mut gzip_stadistics: Vec<Stadistics> = Vec::new();
    let mut size_stadistics: Vec<Stadistics> = Vec::new();
    let mut downloads_names: Vec<String> = Vec::new();

    let client = reqwest::Client::new();
    let semaphore = Arc::new(Semaphore::new(10));
    let mut futures = FuturesUnordered::new();

    for pkg in packages {
        let client = client.clone();
        let semaphore = semaphore.clone();
        futures.push(async move {
            let _permit = semaphore.acquire_owned().await.unwrap();

            let size_result = fetch_size_of_package(&pkg, &client).await;
            (pkg, size_result)
        });
    }

    while let Some((pkg, size_result)) = futures.next().await {
        match size_result {
            Ok(root) => {
                gzip_stadistics.push(Stadistics {
                    package_name: pkg.clone(),
                    value: root.gzip,
                });
                size_stadistics.push(Stadistics {
                    package_name: pkg.clone(),
                    value: root.size,
                });
                downloads_names.push(root.name.clone());
                log_body(&root);
            }
            Err(e) => {
                println!("{}: {}", pkg, &colorize_text(e.as_str(), ERROR).italic());
            }
        }
    }

    println!("\tBuild {} comparative\n", colorize_text("gzip", CYAN));
    gzip_log_comparative(gzip_stadistics, false);
    println!("\n\tBuild {} comparative\n", colorize_text("size", CYAN));
    gzip_log_comparative(size_stadistics, false);
    fetch_package(downloads_names, &client).await;

    let duration = start.elapsed();
    let duration_secs = duration.as_secs_f64();
    println!("\n{}", colorize_text(&format!("Total time: {:.1} seconds\n", duration_secs), PURPLE));
}
