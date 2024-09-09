use colored::Colorize;
use crate::{
    styles::{ CYAN, GREEN, PURPLE },
    types::{ Root, Stadistics },
    utils::{ colorize_text, format_size, format_with_commas, progress_bar },
};

pub fn log_header() {
    println!(
        "{:<10} {:<10} {:<10} {:<20}\n",
        colorize_text("gzip", CYAN).bold(),
        "size".bold(),
        "version".bold(),
        colorize_text("name", PURPLE).bold().italic()
    );
}
pub fn log_body(root: &Root) {
    println!(
        "{:<10} {:<10} {:<10} {:<20}\n",
        colorize_text(format_size(root.gzip).as_str().trim(), CYAN),
        format_size(root.size).as_str().trim(),
        format!("@{}", root.version).as_str().trim(),
        colorize_text(root.name.as_str().trim(), PURPLE).italic()
    );

    // let description = wrap_text(root.description.as_str(), 50);
    // println!("{}\n", description);
}

pub fn gzip_log_comparative(mut gzip_stadistics: Vec<Stadistics>, is_downloads: bool) {
    gzip_stadistics.sort_by(|a, b| b.value.cmp(&a.value));
    let total_elements = gzip_stadistics.len();

    for (index, stat) in gzip_stadistics.iter().enumerate() {
        let percentage =
            (((total_elements as f64) - (index as f64)) / (total_elements as f64)) * 100.0;
        let progress = progress_bar(percentage);

        if is_downloads {
            println!(
                "{} {:<15} {:<10}",
                progress,
                colorize_text(
                    format!("⇅ {}", format_with_commas(stat.value as u64))
                        .as_str()
                        .trim(),
                    GREEN
                ),
                colorize_text(&stat.package_name, PURPLE)
            );
        } else {
            println!(
                "{} {:<8} {:<10}",
                progress,
                format_size(stat.value as u32).to_string(),
                colorize_text(&stat.package_name, PURPLE)
            );
        }
    }
}
