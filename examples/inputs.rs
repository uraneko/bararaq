use bararaq::console::raw_mode::{cooked_mode, raw_mode};
use bararaq::console::winsize::winsize;
use bararaq::inputs::keyboard::{Char, KbdEvent, Modifiers};
use bararaq::inputs::mouse::{disable_mouse_input, enable_mouse_input};
use bararaq::inputs::{event, read, Interaction};

use std::io::Write;

fn main() {
    // initialization
    let ts = raw_mode();

    let mut writer = std::io::stdout().lock();
    enable_mouse_input(&mut writer);
    _ = writer.write(b"\x1b[?1049h\x1b[0;0f");
    _ = writer.flush();

    let mut ws = winsize::from_ioctl();

    let mut reader = std::io::stdin().lock();

    let mut i = vec![];

    loop {
        let input = read(&mut reader, &mut i);
        print!("{:?}\r\n", input);

        let ui = event(input, &mut ws);
        print!("{:?}\r\n{:?}\r\n\r\n", &ui.event, &ui.time);

        // print!("{:?}\r\n", &ui);
        _ = writer.flush();

        if let Interaction::KbdEvent(KbdEvent {
            char: Char::Char('c'),
            modifiers: Modifiers(2),
        }) = ui.event
        {
            break;
        }
    }

    disable_mouse_input(&mut writer);
    cooked_mode(ts);
    _ = writer.write(b"\x1b[?1049l");
}
