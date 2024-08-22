// when a style needs to be applied, it takes the string and mutates it to its values then it gets
// sent over to the event queue to be applied to text

use std::io::StdoutLock;
use std::io::Write;

// NOTE: should create a stylegraph that takes styles
// styles are applied according to stylegraphs
// stylegraphs define rules for which styles apply to which text
// the rules are based on text tokens' attributes
// whether a token includes or excludes (starts, ends or contains) a certain pattern
// the position of the token in the text
// or can take individual chars instead of whole tokens

type StyleId = u8;

#[derive(Default)]
pub struct Style {
    id: StyleId,
    effects: u8,
    text: Option<Color>,
    background: Option<Color>,
}

#[derive(Default)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    fn text(&self, style: &mut String) {
        let color = format!("38;2;{};{};{};", self.r, self.g, self.b);
        style.push_str(&color)
    }

    fn background(&self, style: &mut String) {
        let color = format!("48;2;{};{};{};", self.r, self.g, self.b);
        style.push_str(&color)
    }

    // fn red(&mut self, r: u8) {
    //     self.r = r;
    // }
    //
    // fn green(&mut self, g: u8) {
    //     self.g = g;
    // }
    //
    // fn blue(&mut self, b: u8) {
    //     self.b = b;
    // }
}

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

    pub fn new(name: &str) -> Self {
        Self {
            id: 0,
            background: None,
            text: None,
            effects: 0,
        }
    }

    pub fn bold(&mut self) {
        if (self.effects & Style::BOLD).ne(&0) {
            self.effects &= !Style::BOLD
        } else {
            self.effects |= Style::BOLD
        }
    }

    pub fn underline(&mut self) {
        if (self.effects & Style::UNDERLINE).ne(&0) {
            self.effects &= !Style::UNDERLINE
        } else {
            self.effects |= Style::UNDERLINE
        }
    }

    pub fn double_underline(&mut self) {
        if (self.effects & Style::DBL_UNDERLINE).ne(&0) {
            self.effects &= !Style::DBL_UNDERLINE
        } else {
            self.effects |= Style::DBL_UNDERLINE
        }
    }

    pub fn italic(&mut self) {
        if (self.effects & Style::ITALIC).ne(&0) {
            self.effects &= !Style::ITALIC
        } else {
            self.effects |= Style::ITALIC
        }
    }

    pub fn blink(&mut self) {
        if (self.effects & Style::BLINK).ne(&0) {
            self.effects &= !Style::BLINK
        } else {
            self.effects |= Style::BLINK
        }
    }

    pub fn faint(&mut self) {
        if (self.effects & Style::FAINT).ne(&0) {
            self.effects &= !Style::FAINT
        } else {
            self.effects |= Style::FAINT
        }
    }

    pub fn conceal(&mut self) {
        if (self.effects & Style::CONCEAL).ne(&0) {
            self.effects &= !Style::CONCEAL
        } else {
            self.effects |= Style::CONCEAL
        }
    }

    pub fn reverse(&mut self) {
        if (self.effects & Style::REVERSE).ne(&0) {
            self.effects &= !Style::REVERSE
        } else {
            self.effects |= Style::REVERSE
        }
    }

    pub fn reset(&mut self) {
        self.effects &= Self::RESET;
        self.text = None;
        self.background = None;
    }

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
                "there is no effect with such an index, the index must be: 0 =< idx < 8 "
            ),
        }
    }

    pub fn calibrate(&self, s: &mut String) {
        *s = self.style();
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

    pub fn txt(&mut self, color: &[u8; 3]) {
        self.text = Some(Color::new(color[0], color[1], color[2]));
    }

    pub fn bkg(&mut self, color: &[u8; 3]) {
        self.background = Some(Color::new(color[0], color[1], color[2]));
    }
}

pub trait Stylize {
    fn apply(&self, sol: &mut StdoutLock);
}

impl Stylize for String {
    fn apply(&self, sol: &mut StdoutLock) {
        _ = sol.write(&self.as_bytes());
        // unless the whole line is redrawn, the style would not update
        _ = sol.flush();
    }
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
const style_configuration: u32 = 0;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style() {}
}

struct Token {}

use std::collections::HashMap;
use std::ops::Range;

//
enum MatchingRule {
    // tokens are grouped by conditions
    Conditional,
    // tokens are grouped by appearance, first second and so forth,
    // takes an argument that decides how many tokens would go in one group
    // 1 means that each token is a group, 2 means that every two consecutive tokens make up a group
    Sequential(u8),
}

// stylegraphs describe how styles are applied to bodies of text
struct StyleGraph {
    // fn that takes a text value and tokenizes it in some way
    tokenizer: fn(&[char], MatchingRule),
    // a map matching a style id to a range inside the value
    //// that will be styled
    map: HashMap<StyleId, Vec<Range<usize>>>,
    rule: MatchingRule,
}

impl StyleGraph {
    fn new(f: fn(&[char], MatchingRule), rule: MatchingRule) -> Self {
        Self {
            map: HashMap::new(),
            tokenizer: f,
            rule,
        }
    }

    fn tokenize(&self) {}

    fn tokenizer(&mut self, f: fn(&[char], MatchingRule)) {
        self.tokenizer = f;
    }

    fn randomize(&mut self) {}

    // manually pair a style id with a Range in the map
    fn pair(&mut self, sid: u8, range: Range<usize>) {}

    fn rule(&mut self, mr: MatchingRule) {
        self.rule = mr;
    }
}
