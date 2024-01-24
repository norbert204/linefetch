use crate::distro::Distro;

pub const RESET: &'static str = "\x1b[0m";

#[allow(dead_code)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    LightGray,
    Gray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    White
}

impl Color {
    pub fn accent_color(distro: &Distro) -> Self {
        match distro {
            Distro::ArchLinux => Color::Cyan,
            Distro::Debian => Color::Red,
            _ => Color::Yellow,
        }
    }
}

#[allow(dead_code)]
pub enum Modifier {
    Reset,
    Bold,
    Faint,
    Italics,
    Underlined
}

pub fn color_code(
    foreground: Option<&Color>,
    background: Option<&Color>,
    modifier: Option<&Modifier>
) -> String {
    let mut codes: Vec<&str> = vec![];

    if let Some(c) = foreground {
        codes.push(get_foreground_color(&c));
    }

    if let Some(c) = background {
        codes.push(get_background_color(&c));
    }

    if let Some(m) = modifier {
        codes.push(get_modifier(&m));
    }

    format!("\x1b[{}m", codes.join(";"))
}

fn get_foreground_color(color: &Color) -> &'static str {
    match color {
        Color::Black => "30",
        Color::Red => "31",
        Color::Green => "32",
        Color::Yellow => "33",
        Color::Blue => "34",
        Color::Magenta => "35",
        Color::Cyan => "36",
        Color::LightGray => "37",
        Color::Gray => "90",
        Color::LightRed => "91",
        Color::LightGreen => "92",
        Color::LightYellow => "93",
        Color::LightBlue => "94",
        Color::LightMagenta => "95",
        Color::LightCyan => "96",
        Color::White => "97",	
    }
}

fn get_background_color(color: &Color) -> &'static str {
    match color {
        Color::Black => "40",
        Color::Red  => "41",
        Color::Green => "42",
        Color::Yellow => "43",
        Color::Blue => "44",
        Color::Magenta  => "45",
        Color::Cyan => "46",
        Color::LightGray => "47",
        Color::Gray => "100",
        Color::LightRed => "101",
        Color::LightGreen  => "102",
        Color::LightYellow => "103",
        Color::LightBlue => "104",
        Color::LightMagenta => "105",
        Color::LightCyan => "106",
        Color::White => "107",
    }
}

fn get_modifier(modifier: &Modifier) -> &'static str {
    match modifier {
 	    Modifier::Reset => "0",
 	    Modifier::Bold => "1",
 	    Modifier::Faint => "2",
 	    Modifier::Italics => "3",
 	    Modifier::Underlined => "4",
    }
}
