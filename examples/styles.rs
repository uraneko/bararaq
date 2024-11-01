use std::io::Write;

use bararaq::colorscheme::*;

const COLORS: [[u8; 3]; 7] = [
    [222, 56, 43],   // pink
    [57, 181, 74],   // orange
    [255, 199, 6],   // red
    [0, 111, 184],   // violet
    [118, 38, 113],  // dark blue
    [44, 181, 233],  // light blue
    [128, 128, 128], // gray
];

const DEFS: [&str; 8] = ["40", "41", "42", "43", "44", "45", "46", "47"];

fn main() {
    let mut writer = std::io::stdout().lock();
    let mut stl = Style::new().bold().underline();

    let value = "Imagine fancy dummy text\x1b[K\x1b[0m\n\n";
    let es = "\x1b[".to_string();

    for c in DEFS.into_iter().rev().collect::<Vec<&str>>() {
        // stl = stl.background_color(&c);
        let s = change_color(
            es.clone() + c + if c != "40" { ";30m" } else { ";0;40m" },
            value,
        );
        _ = writer.write(s.as_bytes());
    }
}

fn change_color(color: String, value: &str) -> String {
    color + value
}
