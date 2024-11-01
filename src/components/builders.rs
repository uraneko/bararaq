use crate::components::{Container, Term, Text};
use crate::space::{border::Border, layout::Layout, padding::Padding, Area, Pos};

// pass the meta series to the component making methods in tree and term

#[derive(Debug, Clone)]
pub struct TermBuilder {
    layout: Layout,
    area: Area,
    id: u8,
}

impl TermBuilder {
    pub fn new() -> Self {
        Self {
            layout: Layout::Flex,
            area: Area::Zero,
            id: 0,
        }
    }

    pub fn id(mut self, id: u8) -> u8 {
        self.id
    }

    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    pub fn area(mut self, area: Area) -> Self {
        self.area = area;
        self
    }

    pub fn clear(self) -> Self {
        Self::new()
    }

    pub(super) fn build(&mut self) -> Term {
        Term {
            id: {
                let id = self.id;
                self.bump_id();
                id
            },
            layout: self.layout.clone(),
            w: self.area.width().unwrap(),
            h: self.area.height().unwrap(),
            ..Term::default()
        }
    }

    pub fn offset_id(&mut self, mut id: u8) {
        self.id = id;
    }

    fn bump_id(&mut self) {
        self.id += 1;
    }
}

#[derive(Debug, Clone)]
pub struct ContainerBuilder {
    layer: u8,
    id: [u8; 2],
    border: Border,
    padding: Padding,
    area: Area,
    layout: Layout,
    hpos: Pos,
    vpos: Pos,
}

impl ContainerBuilder {
    pub fn new() -> Self {
        Self {
            layer: 0,
            id: [0; 2],
            padding: Padding::None,
            border: Border::None,
            area: Area::Fill,
            hpos: Pos::Center,
            vpos: Pos::Center,
            layout: Layout::Flex,
        }
    }

    pub fn overlay(mut self, overlay: bool) -> Self {
        self
    }

    pub fn layer(mut self, layer: u8) -> Self {
        self.layer = layer;
        self
    }

    pub fn border(mut self, border: Border) -> Self {
        self.border = border;
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn area(mut self, area: Area) -> Self {
        self.area = area;
        self
    }

    pub fn hpos(mut self, hpos: Pos) -> Self {
        self.hpos = hpos;
        self
    }

    pub fn vpos(mut self, vpos: Pos) -> Self {
        self.vpos = vpos;
        self
    }

    pub fn layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    pub fn bump_tid(&mut self, id: u8) {
        self.id[0] = id;
    }

    pub fn bump_cid(&mut self, id: u8) {
        self.id[1] = id
    }

    pub fn id(&self) -> [u8; 2] {
        self.id
    }

    pub(super) fn build(&mut self) -> Container {
        Container {
            id: self.id(),
            layout: self.layout.clone(),
            w: self.area.width().unwrap(),
            h: self.area.height().unwrap(),
            ..Container::default()
        }
    }

    pub fn offset_id(&mut self, mut id: Vec<u8>) -> Result<u8, BuilderError> {
        match id.len() {
            0 => return Ok(0),
            1 => {
                self.id[0] = id[0];
                return Ok(1);
            }
            2 => {
                self.id = [id[0], id[1]];
                return Ok(2);
            }
            _ => return Err(BuilderError::TooManyIds),
        }
    }

    fn clear(self) -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum BuilderTreeError {
    IdInUse,
}
#[derive(Debug)]
pub enum BuilderError {
    InvalidInputId,
    InvalidNoEditId,
    TooManyIds,
}

#[derive(Debug, Clone)]
pub struct TextBuilder {
    layer: u8,
    id: [u8; 3],
    border: Border,
    padding: Padding,
    area: Area,
    hpos: Pos,
    vpos: Pos,
}

impl TextBuilder {
    pub fn new() -> Self {
        Self {
            layer: 0,
            id: [0; 3],
            padding: Padding::None,
            border: Border::None,
            area: Area::Fill,
            hpos: Pos::Center,
            vpos: Pos::Center,
        }
    }

    pub fn layer(mut self, layer: u8) -> Self {
        self.layer = layer;
        self
    }

    pub fn border(mut self, border: Border) -> Self {
        self.border = border;
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn area(mut self, area: Area) -> Self {
        self.area = area;
        self
    }

    pub fn hpos(mut self, hpos: Pos) -> Self {
        self.hpos = hpos;
        self
    }

    pub fn vpos(mut self, vpos: Pos) -> Self {
        self.vpos = vpos;
        self
    }

    pub fn clear(self) -> Self {
        Self::new()
    }

    pub fn build(&mut self) -> Text {
        Text {
            id: {
                let id = self.id();
                self.bump_iid();
                id
            },
            w: self.area.width().unwrap_or(0),
            h: self.area.height().unwrap_or(0),
            ..Text::default()
        }
    }

    pub fn offset_id(&mut self, mut id: Vec<u8>) -> Result<u8, BuilderError> {
        match id.len() {
            0 => return Ok(0),
            1 => {
                self.id[0] = id[0];
                return Ok(1);
            }
            2 => {
                self.id = [id[0], id[1], self.id[2]];
                return Ok(2);
            }
            3 => {
                if id[2] % 2 != 0 {
                    return Err(BuilderError::InvalidInputId);
                }

                self.id = [id[0], id[1], id[2]];
                return Ok(3);
            }
            _ => return Err(BuilderError::TooManyIds),
        }
    }

    fn bump_tid(&mut self) {
        self.id[0] += 1;
    }

    fn bump_cid(&mut self) {
        self.id[1] += 1;
    }

    fn bump_iid(&mut self) {
        self.id[2] += 2;
    }

    pub(super) fn cid(&self) -> [u8; 2] {
        [self.id[0], self.id[1]]
    }

    pub(super) fn id(&self) -> [u8; 3] {
        self.id
    }
}

// TODO: phase out input and noedit
// in favor of text with editable field
