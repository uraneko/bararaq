use super::color::Color;

/// abstraction over the vt100 terminal's graphic rendition function
#[derive(Debug, Default)]
pub struct Style {
    effects: u8,
    text: Option<Color>,
    background: Option<Color>,
}

/// can only have one combination that results in the same sum
/// 0 means reset all
/// 1 means bold
/// 2 means underline
/// 4 means double underline
/// 8 means italic
/// 16 means reverse
/// 32 means conceal
/// 64 means blink
/// 128 means faint
/// the greatest effects config value possible is 255
impl Style {
    const RESET: u8 = 0; // 0
    const BOLD: u8 = 1; // 1
    const FAINT: u8 = 2; // 2
    const ITALIC: u8 = 4; // 3
    const UNDERLINE: u8 = 8; // 4
    const BLINK: u8 = 16; // 5, 6
    const REVERSE: u8 = 32; // 7
    const CONCEAL: u8 = 64; // 8
    const DBL_UNDERLINE: u8 = 128; // 21

    /// creates a new Style instance
    pub fn new() -> Self {
        Self {
            background: None,
            text: None,
            effects: 0,
        }
    }

    /// toggles the bold effect to this style's value
    /// returns self
    pub fn bold(mut self) -> Self {
        if (self.effects & Style::BOLD).ne(&0) {
            self.effects &= !Style::BOLD
        } else {
            self.effects |= Style::BOLD
        }

        self
    }

    /// toggles the underline effect to this style's value
    /// returns self
    pub fn underline(mut self) -> Self {
        if (self.effects & Style::UNDERLINE).ne(&0) {
            self.effects &= !Style::UNDERLINE
        } else {
            self.effects |= Style::UNDERLINE
        }

        self
    }

    /// toggles the double underline effect to this style's value
    /// returns self
    pub fn double_underline(mut self) -> Self {
        if (self.effects & Style::DBL_UNDERLINE).ne(&0) {
            self.effects &= !Style::DBL_UNDERLINE
        } else {
            self.effects |= Style::DBL_UNDERLINE
        }

        self
    }

    /// toggles the italic effect to this style's value
    /// returns self
    pub fn italic(mut self) -> Self {
        if (self.effects & Style::ITALIC).ne(&0) {
            self.effects &= !Style::ITALIC
        } else {
            self.effects |= Style::ITALIC
        }

        self
    }

    /// toggles the blink effect to this style's value
    /// returns self
    pub fn blink(mut self) -> Self {
        if (self.effects & Style::BLINK).ne(&0) {
            self.effects &= !Style::BLINK
        } else {
            self.effects |= Style::BLINK
        }

        self
    }

    /// toggles the faint to this style's value
    /// returns self
    pub fn faint(mut self) -> Self {
        if (self.effects & Style::FAINT).ne(&0) {
            self.effects &= !Style::FAINT
        } else {
            self.effects |= Style::FAINT
        }

        self
    }

    /// toggles the conceal to this style's value
    /// returns self
    pub fn conceal(mut self) -> Self {
        if (self.effects & Style::CONCEAL).ne(&0) {
            self.effects &= !Style::CONCEAL
        } else {
            self.effects |= Style::CONCEAL
        }

        self
    }

    /// toggles the reverse to this style's value
    /// returns self
    pub fn reverse(mut self) -> Self {
        if (self.effects & Style::REVERSE).ne(&0) {
            self.effects &= !Style::REVERSE
        } else {
            self.effects |= Style::REVERSE
        }

        self
    }

    /// resets this style, removing all effects and colors
    /// returns self
    pub fn reset(mut self) -> Self {
        self.effects &= Self::RESET;
        self.text = None;
        self.background = None;

        self
    }

    /// returns this style's escape sequence that can be written to the terminal buffer
    pub fn style(&self) -> String {
        let mut style = String::from("\x1b[");

        // add effects
        self.bits().iter().for_each(|b| style += Self::effect(b));

        // add text color
        self.text(&mut style);

        // add background color
        self.background(&mut style);

        // clean up the expression
        match style.remove(style.len() - 1) {
            '[' => style += "[0m",
            _ => style += "m",
        };

        style
    }

    fn bits(&self) -> [u8; 8] {
        [
            self.effects & Self::DBL_UNDERLINE,
            self.effects & Self::CONCEAL,
            self.effects & Self::REVERSE,
            self.effects & Self::BLINK,
            self.effects & Self::UNDERLINE,
            self.effects & Self::ITALIC,
            self.effects & Self::FAINT,
            self.effects & Self::BOLD,
        ]
    }

    fn effect<'a>(effect: &u8) -> &'a str {
        match effect {
            0 => "",
            1 => "1;",
            2 => "2;",
            4 => "3;",
            8 => "4;",
            16 => "5;",
            32 => "7;",
            64 => "8;",
            128 => "21;",
            _ => unreachable!(
                "there is no effect with such an index, the index must be: 0 =< idx < 8"
            ),
        }
    }

    /// dumps the current style values into a pre-existing string argument
    /// the same as the style method but this one takes a mutable reference to a String and
    /// modifies it in place
    pub fn dump_style(&self, s: &mut String) {
        s.clear();
        s.push_str("\x1b[");

        // add effects
        self.bits().iter().for_each(|b| *s += Self::effect(b));

        // add text color
        self.text(s);

        // add background color
        self.background(s);

        // clean up the expression
        match s.remove(s.len() - 1) {
            '[' => *s += "[0m",
            _ => *s += "m",
        };
    }

    fn text(&self, style: &mut String) {
        if self.text.is_some() {
            self.text.as_ref().unwrap().text(style);
        }
    }

    fn background(&self, style: &mut String) {
        if self.background.is_some() {
            self.background.as_ref().unwrap().background(style);
        }
    }

    /// changes the style text color to the provided rgb value
    pub fn text_color(mut self, color: &[u8; 3]) -> Self {
        self.text = Some(Color::new(color[0], color[1], color[2]));

        self
    }

    /// changes the style text color's red value with the provided new one
    pub fn text_red(mut self, r: u8) -> Self {
        self.text.as_mut().unwrap().red(r);

        self
    }

    /// changes the style text color's green value with the provided new one
    pub fn text_green(mut self, g: u8) -> Self {
        self.text.as_mut().unwrap().green(g);

        self
    }

    /// changes the style text color's blue value with the provided new one
    pub fn text_blue(mut self, b: u8) -> Self {
        self.text.as_mut().unwrap().blue(b);

        self
    }

    /// changes the style background color to the provided rgb value
    pub fn background_color(mut self, color: &[u8; 3]) -> Self {
        self.background = Some(Color::new(color[0], color[1], color[2]));

        self
    }

    /// changes the style background color's red value with the provided new one
    pub fn background_red(mut self, r: u8) -> Self {
        self.background.as_mut().unwrap().red(r);

        self
    }

    /// changes the style background color's green value with the provided new one
    pub fn background_green(mut self, g: u8) -> Self {
        self.background.as_mut().unwrap().green(g);

        self
    }

    /// changes the style background color's blue value with the provided new one
    pub fn background_blue(mut self, b: u8) -> Self {
        self.background.as_mut().unwrap().blue(b);

        self
    }
}

#[cfg(test)]
mod styles {
    use super::Style;

    #[test]
    fn effects() {
        let mut s = Style::new();
        assert_eq!(s.effects, Style::RESET);
        s = s
            .bold()
            .faint()
            .italic()
            .underline()
            .double_underline()
            .blink()
            .reverse()
            .conceal();

        assert_eq!(s.effects & Style::BOLD, 1);
        assert_eq!(s.effects & Style::FAINT, 2);
        assert_eq!(s.effects & Style::ITALIC, 4);
        assert_eq!(s.effects & Style::UNDERLINE, 8);
        assert_eq!(s.effects & Style::BLINK, 16);
        assert_eq!(s.effects & Style::REVERSE, 32);
        assert_eq!(s.effects & Style::CONCEAL, 64);
        assert_eq!(s.effects & Style::DBL_UNDERLINE, 128);

        s = s.reset();
        assert_eq!(s.effects, Style::RESET);
    }

    #[test]
    fn text() {
        let s = Style::new().text_color(&[34, 34, 34]);
        assert_eq!(s.text.as_ref().unwrap().array(), [34, 34, 34]);

        let t0 = s.style();
        assert_eq!(&t0[..], "\x1b[38;2;34;34;34m");

        let mut t = "".to_string();
        s.dump_style(&mut t);
        assert_eq!(&t[..], "\x1b[38;2;34;34;34m");
    }

    #[test]
    fn background() {
        let s = Style::new().background_color(&[34, 34, 34]);
        assert_eq!(s.background.as_ref().unwrap().array(), [34, 34, 34]);

        let t0 = s.style();
        assert_eq!(&t0[..], "\x1b[48;2;34;34;34m");

        let mut t = "".to_string();
        s.dump_style(&mut t);
        assert_eq!(&t[..], "\x1b[48;2;34;34;34m");
    }
}
