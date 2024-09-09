use clap::Parser;
use colored::Colorize;
use controllers::{
    file_package::file_package_dependencies,
    file_remote::file_remote,
    name_packages::name_packages,
};
use styles::CYAN;
use tokio::runtime::Runtime;
use utils::{ colorize_text, msg_welcome };
use std::error::Error;

mod utils;
mod styles;
mod controllers;
mod types;
mod ui;
mod service;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short = 'r', long = "remote")]
    remote: Option<String>,
    #[arg(short = 'f', long = "file")]
    file: Option<String>,
    #[arg(short = 'p', long = "package", num_args = 1..)]
    packages: Option<Vec<String>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    msg_welcome();

    if args.remote.is_none() && args.file.is_none() && args.packages.is_none() {
        println!("\n{} {}\n", colorize_text("Uso: ", CYAN).bold(), "./program [opciones]");
        println!();
        println!("{}", colorize_text("Opciones:", CYAN).bold());
        println!("-r, --remote [URL]      Repositorio de GitHub");
        println!("-f, --file [PATH]       Archivo local");
        println!("-p, --package [PAQUETES] Paquete(s) en línea");
        println!();
        println!("{}", colorize_text("Ejemplos:", CYAN).bold());
        println!("\t./program -r https://github.com/usuario/repo");
        println!("\t./program -f /ruta/al/archivo.txt");
        println!("\t./program -p paquete1 paquete2 paquete3");
        return Ok(());
    }
    let rt = Runtime::new().unwrap();

    if let Some(remote) = args.remote {
        file_remote(&remote, &rt);
    }

    if let Some(file) = args.file {
        file_package_dependencies(file, &rt);
    }

    if let Some(packages) = args.packages {
        name_packages(packages, &rt);
    }
    Ok(())
}
