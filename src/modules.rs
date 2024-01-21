use core::panic;
use std::str::Lines;

use crate::color;
use crate::distro::Distro;

pub fn distro_badge(distro: &Distro) -> String {
    fn create_badge(
        logo_color: color::Color,
        background_color: color::Color,
        distro_logo: &str,
        separator: &str
    ) -> String {
        format!(
            "{} {} {}{}{}{}",
            color::color_code(Some(&logo_color), Some(&background_color), None),
            distro_logo,
            color::RESET,
            color::color_code(Some(&background_color), None, None),
            separator,
            color::RESET
        )
    }

    // TODO: Accent and background color definitions should be moved to color module.
    match distro {
        Distro::ArchLinux => create_badge(color::Color::White, color::Color::Cyan, "\u{f303}", "\u{e0b0}"),
        Distro::Debian => create_badge(color::Color::Red, color::Color::White, "\u{f306}", "\u{e0b4}"),
        _ => create_badge(color::Color::Black, color::Color::Yellow, "\u{ebc6}", "\u{e0b0}")
    }
}

pub fn from_string(module: &String) -> fn(&color::Color) -> String {
    match module.as_str() {
        "kernel" => kernel,
        "ip-address" => ip_address,
        "memory" => memory,
        _ => panic!("Unknown module in configuration '{}'", module),
    }
}

pub fn kernel(accent_color: &color::Color) -> String {
    let version_info = std::fs::read_to_string("/proc/version").unwrap();

    let kernel = version_info.split_whitespace().nth(2).unwrap();

    render_module(accent_color, "\u{e712}", kernel.to_string())
}

pub fn memory(accent_color: &color::Color) -> String {
    fn get_kb_from_line(line: &str) -> i32 {
        line.split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<i32>()
            .unwrap()
    }

    let meminfo = std::fs::read_to_string("/proc/meminfo").unwrap();
    
    let meminfo: Vec<&str> = meminfo.lines().take(3).collect();

    let total = get_kb_from_line(meminfo[0]) / 1024;
    let available = get_kb_from_line(meminfo[2]) / 1024;

    render_module(accent_color, "\u{f061a}", format!("{}M/{}M", available.to_string(), total.to_string()))
}

pub fn ip_address(accent_color: &color::Color) -> String {
    render_module(accent_color, "\u{f06f3}", local_ip_address::local_ip().unwrap().to_string())
}

fn render_module(accent_color: &color::Color, icon: &str, info: String) -> String {
    format!(
        "{}{}{} {}",
        color::color_code(Some(&accent_color), None, None),
        icon,
        color::RESET,
        info
    )
}
