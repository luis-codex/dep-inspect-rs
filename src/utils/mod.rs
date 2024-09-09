use std::env;
use colored::{ ColoredString, Colorize };
use crate::styles::CYAN;

#[allow(dead_code)]
fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
    (r, g, b)
}

pub fn colorize_text(text: &str, hex: &str) -> ColoredString {
    let (r, g, b) = hex_to_rgb(hex);
    text.truecolor(r, g, b)
}

#[allow(dead_code)]
pub fn colorize_text_with_bg(text: &str, hex: &str, bg_hex: &str) -> ColoredString {
    let (r, g, b) = hex_to_rgb(hex);
    let (bg_r, bg_g, bg_b) = hex_to_rgb(bg_hex);
    text.truecolor(r, g, b).on_truecolor(bg_r, bg_g, bg_b)
}

#[allow(dead_code)]
pub fn colorize_bg(text: &str, bg_hex: &str) -> ColoredString {
    let (bg_r, bg_g, bg_b) = hex_to_rgb(bg_hex);
    text.on_truecolor(bg_r, bg_g, bg_b)
}

#[allow(dead_code)]
pub fn format_size(size: u32) -> String {
    if size >= 1_000_000 {
        format!("{:.1}MB", (size as f64) / 1_000_000.0)
    } else if size >= 1_000 {
        format!("{:.1}kB", (size as f64) / 1_000.0)
    } else {
        format!("{}B", size)
    }
}

#[allow(dead_code)]
pub fn wrap_text(text: &str, max_width: usize) -> String {
    let mut wrapped_text = String::new();
    let mut line_length = 0;
    let prefix = "| ".bold();

    wrapped_text.push_str(&prefix);

    for word in text.split_whitespace() {
        if line_length + word.len() + 1 > max_width {
            wrapped_text.push('\n');
            wrapped_text.push_str(&prefix);
            line_length = 0;
        } else if line_length > 0 {
            wrapped_text.push(' ');
            line_length += 1;
        }
        wrapped_text.push_str(&word.italic().to_string());
        line_length += word.len();
    }
    wrapped_text
}
#[allow(dead_code)]
pub fn format_with_commas(number: u64) -> String {
    let num_str = number.to_string();
    let mut formatted_str = String::new();
    let mut count = 0;

    for ch in num_str.chars().rev() {
        if count == 3 {
            formatted_str.push('.');
            count = 0;
        }
        formatted_str.push(ch);
        count += 1;
    }

    formatted_str.chars().rev().collect()
}

fn name_sistemas_operativos() -> String {
    let os = env::consts::OS;
    (
        match os {
            "macos" => "macOS 🍏",
            "windows" => "Windows 🪟",
            "linux" => "linux 🐧",
            _ => os,
        }
    ).to_string()
}

pub fn msg_welcome() {
    println!(
        "\n\t {}    {} \n",
        colorize_text("◭ pack-analyzer.npmjs", CYAN).bold(),
        name_sistemas_operativos()
    );
}

pub fn progress_bar(percentage: f64) -> String {
    let total_blocks = 20;
    let filled_blocks = ((percentage / 100.0) * (total_blocks as f64)).round() as usize;
    let empty_blocks = total_blocks - filled_blocks;

    let filled_part = "∷".repeat(filled_blocks);
    let empty_part = " ".repeat(empty_blocks);
    format!("[{}{}]", filled_part, empty_part)
}
