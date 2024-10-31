use std::collections::{HashMap, HashSet};
use std::io::StdoutLock;
use std::io::Write;

use crate::colorscheme::ColorScheme;
use crate::components::property::{Properties, Property};
use crate::console::winsize::winsize;
use crate::render_pipeline;
use crate::space::{area_conflicts, between, border::Border, border_fit, padding::Padding};

use super::Style;
use super::{Container, Term};
use super::{SpaceError, TreeError};

/// Text objects are direct children of the Container objects
/// and indirect children of the Term grand parent
#[derive(Debug, Default)]
pub struct Text {
    /// the layer of this Text inside its parent Container
    /// decide which Text takes render priority in case of conflict
    /// think of it like css z-index
    pub layer: u8,
    /// unique id
    pub id: [u8; 3],
    /// temporary value holder for use when scorrling history
    // this should be part of properties
    pub temp: Vec<Option<char>>,
    /// the value inside this Text object
    pub value: Vec<Option<char>>,
    /// history cursor current value
    // this field should be part of properties
    pub hicu: usize,
    /// width
    pub w: u16,
    /// height
    pub h: u16,
    /// this Text's cursor x coordinate
    pub crsh: u16,
    /// this Text's cursor y coordinate
    pub crsv: u16,
    /// origin point x coordinate relative to the dimensions of the parent Container
    pub hpos: u16,
    /// origin point y coordinate relative to the dimensions of the parent Container
    pub vpos: u16,
    /// origin point x coordinate absolute value inside the Term
    pub ahpos: u16,
    /// origin point y coordinate absolute value inside the Term
    pub avpos: u16,
    /// border value
    pub border: Border,
    /// padding value
    pub padding: Padding,
    // colorscheme
    pub colorscheme: ColorScheme,
    // should be property
    /// border style
    // pub bstyle: String,
    /// value style
    // pub vstyle: String,
    pub properties: Properties,
    pub attributes: HashSet<&'static str>,
    pub built_on: std::time::Instant,
    pub editable: bool,
}

// NOTE: Inputs can only have pair IDs
// while NoEdits can only have odd IDs
impl Text {
    /// creates a new Text objects
    /// takes most of Text's field values as arguments and returns a Text instance
    pub fn new(
        id: [u8; 3],
        hpos: u16,
        vpos: u16,
        ahpos: u16,
        avpos: u16,
        w: u16,
        h: u16,
        value: &[Option<char>],
        border: Border,
        padding: Padding,
    ) -> Text {
        Text {
            id,
            w,
            h,
            temp: vec![],
            hicu: 0,
            hpos,
            vpos,
            ahpos,
            avpos,
            properties: HashMap::new(),
            attributes: HashSet::new(),
            colorscheme: ColorScheme::default(),
            border,
            padding,
            value: {
                let mut v = Vec::with_capacity((w * h) as usize);
                v.resize((w * h) as usize, None);
                v.extend_from_slice(value);

                v
            },
            crsh: 0,
            crsv: 0,

            layer: 0,
            built_on: std::time::Instant::now(),
        }
    }

    pub fn is_focused(&self) -> bool {
        self.attributes.contains("focused")
    }

    pub fn has_attribute(&self, attr: &str) -> bool {
        self.attributes.contains(attr)
    }

    /// changes the value style of this container
    // pub fn vstyle(&mut self, style: &Style) {
    //     self.vstyle = style.style();
    // }

    /// changes the border style of this text
    // pub fn bstyle(&mut self, style: &Style) {
    //     self.bstyle = style.style();
    // }

    // pub fn with_layer(id: [u8; 3], layer: u8) -> Self {
    //     Text {
    //         layer,
    //         id,
    //         w: 37,
    //         h: 5,
    //         hpos: 5,
    //         vpos: 2,
    //     }
    // }

    /// returns the id of the parent container of this text
    pub fn parent(&self) -> [u8; 2] {
        [self.id[0], self.id[1]]
    }
}
