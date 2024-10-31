use super::*;
use crate::colorscheme::ColorScheme;
use crate::space::{border::Border, layout::Layout, padding::Padding, Area, Origin};
use property::Properties;

use super::ComponentTree as Tree;

// term

// from id and area
impl From<(u8, [u16; 2])> for Term {
    fn from(value: (u8, [u16; 2])) -> Self {
        Term::new(value.0, value.1[0], value.1[1])
    }
}

// from builder
impl From<&mut TermBuilder> for Term {
    fn from(value: &mut TermBuilder) -> Self {
        value.build()
    }
}

// from id
impl From<u8> for Term {
    fn from(value: u8) -> Self {
        Term::with_id(value)
    }
}

//
// container

impl From<([u8; 2], [u16; 2])> for Container {
    fn from(value: ([u8; 2], [u16; 2])) -> Self {
        Container::new(value.0, value.1[0], value.1[1])
    }
}

impl From<[u8; 2]> for Container {
    fn from(value: [u8; 2]) -> Self {
        Container::with_id(value)
    }
}

impl From<&mut ContainerBuilder> for Container {
    fn from(value: &mut ContainerBuilder) -> Self {
        value.build()
    }
}

//
// text

impl From<([u8; 3], [u16; 2])> for Text {
    fn from(value: ([u8; 3], [u16; 2])) -> Self {
        Self::new(value.0, value.1[0], value.1[1])
    }
}

impl From<[u8; 3]> for Text {
    fn from(value: [u8; 3]) -> Self {
        Self::with_id(value)
    }
}

impl From<&mut InputBuilder> for Text {
    fn from(value: &mut InputBuilder) -> Self {
        value.build()
    }
}

impl From<&mut NoEditBuilder> for Text {
    fn from(value: &mut NoEditBuilder) -> Self {
        value.build()
    }
}
