use std::collections::HashMap;
use std::io::StdoutLock;
use std::io::Write;
use std::ops::Range;

pub(super) mod color;
pub mod style;

pub use style::Style;

// component's colorscheme operations
pub trait ComponentColors {}

/// 'sp is the lifetime of the borrowed Styles and Patterns
/// 'p is the lifetime of the borrowed pattern value in the Pattern
type StyleGroup<'sp: 'p, 'p> = HashMap<&'sp Pattern<'p>, &'sp Style>;

// TODO: add some template theme functions to ragout-extended
// NOTE: border/text themes should be part of the properties and attributes functionalities
// example custom theme on some component text/border value
// styles should be a hashmap of <Pattern, Style>
// then we iter through chars and if we find pattern we insert raw style into value before the
// pattern text then after the pattern text we insert a raw style reset
pub fn color_scheme(value: &[char], styles: &[Style]) -> String {
    // example custom theme
    let mut idx = 0;
    value
        .into_iter()
        .map(|c| {
            let mut sc = styles[idx].style();
            sc.push(*c);
            if idx == styles.len() - 1 {
                idx = 0
            } else {
                idx += 1;
            }

            sc
        })
        .collect::<String>()
}

// takes a value str and a slice of patterns
// returns a collection of two things the value
// as items broken by all patterns and the pattern kind of the item
// this should take hashmap of <pattern, style>
pub fn iter_pats(value: &str, pats: &HashMap<&Pattern, &Style>) {}

#[derive(Debug, Default)]
pub struct Pattern<'p> {
    starts_with: &'p str,
    ends_with: &'p str,
    starts_ends_with: &'p str,
    contains: &'p str,
    excludes: &'p str,
    equals: &'p str,
}

pub(crate) struct ColorScheme {
    border_color: Box<dyn FnMut(&[char], HashMap<Pattern, Style>) -> String>,
    border_background: Box<dyn FnMut(&[char], HashMap<Pattern, Style>) -> String>,
    color: Box<dyn FnMut(&[char], HashMap<Pattern, Style>) -> String>,
    background: Box<dyn FnMut(&[char], HashMap<Pattern, Style>) -> String>,
}

impl Default for ColorScheme {
    fn default() -> Self {
        Self {
            border: Box::new(|value, styles| "".into()),
            text: Box::new(|value, styles| "".into()),
            background: Box::new(|value, styles| "".into()),
        }
    }
}
use std::any::type_name_of_val;
impl std::fmt::Debug for ColorScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "color: {}\nbackground: {}\nborder_color: {}\nborder_background: {}",
                type_name_of_val(&self.color),
                type_name_of_val(&self.background),
                type_name_of_val(&self.border_color),
                type_name_of_val(&self.border_background),
            )
        )
    }
}

///////////////////////////////////////////////////////////////
// old themes.rs

use std::collections::HashMap;
use std::io::StdoutLock;
use std::io::Write;
use std::ops::Range;

// TODO: add some template theme functions to ragout-extended
// NOTE: border/text themes should be part of the properties and attributes functionalities
// example custom theme on some component text/border value
fn theme(value: &[char], styles: &[Style]) -> String {
    // example custom theme
    let mut idx = 0;
    value
        .into_iter()
        .map(|c| {
            let mut sc = styles[idx].style();
            sc.push(*c);
            if idx == styles.len() - 1 {
                idx = 0
            } else {
                idx += 1;
            }

            sc
        })
        .collect::<String>()
}

// takes a value str and a slice of patterns
// returns a collection of two things the value as items broken by all patterns and the pattern kind of the item
pub fn iter_pats(value: &str, pats: &[&str]) {}

pub enum Patterns {
    StartsWith(&'static str),
    EndsWith(&'static str),
    StartsEndWith(&'static str),
    Contains(&'static str),
    Excludes(&'static str),
    Equals(&'static str),
}
