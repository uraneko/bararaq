use std::io::Write;

use crate::components::*;

impl Container {
    pub fn scroll_on(&self, writer: &mut std::io::StdoutLock) {
        if self.attributes.contains("scrollable") {
            writer.write(format!("\x1b[{};{}r", self.vpos, self.h + self.vpos).as_bytes());
        }
    }

    fn scroll_off(&mut self) {}
}
