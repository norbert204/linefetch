use core::panic;

mod color;
mod config;
mod distro;
mod modules;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::load();

    let distro = distro::Distro::new();
    let accent_color = color::Color::accent_color(&distro);

    let mut modules_to_load: Vec<fn(&color::Color) -> String> = Vec::new();

    for module in config.modules {
        modules_to_load.push(modules::from_string(&module))
    }

    if modules_to_load.len() != 3 {
        panic!("Error! 3 modules should be loaded!");
    }

    let mut module_strings: Vec<String> = Vec::new();

    module_strings.push(modules::distro_badge(&distro));

    for module in modules_to_load {
        module_strings.push(module(&accent_color));
    }

    println!("\n{}", module_strings.join("    "));

    Ok(())
}
