use std::mem::discriminant;

use crate::components::{makers::*, ComponentTree, Container, SpaceError, Term, Text, TreeError};
use crate::space::{Area, Pos};

#[derive(Debug, Clone, Default)]
pub enum Layout {
    #[default]
    /// no particular layout rules are applied on the children
    /// every child will follow its area and position
    Canvas,
    /// children are displayed in a flex style
    /// for more customization add a "flex" map property to this component
    /// with the needed properties
    Flex,
    /// children are displayed in a grid style
    /// for more customization add a "grid" map property to this component
    /// with the needed properties
    Grid,
}

// TODO: if flex/grid then apply rules on every comp_push()/comp() methods
// both Flex and Grid would have to use the attributes and properties feature

impl Layout {
    fn is_canvas(&self) -> bool {
        discriminant(self) == discriminant(&Self::Canvas)
    }

    fn is_flex(&self) -> bool {
        discriminant(self) == discriminant(&Self::Flex)
    }

    fn is_grid(&self) -> bool {
        discriminant(self) == discriminant(&Self::Grid)
    }
}

impl From<char> for Layout {
    fn from(value: char) -> Self {
        match value {
            'c' => Self::Canvas,
            'f' => Self::Flex,
            'g' => Self::Grid,
        }
    }
}

pub trait LayoutRules {
    fn is_canvas(&self) -> bool;
    fn is_flex(&self) -> bool;
    fn is_grid(&self) -> bool;

    // if no flex parameters are specified in the properties
    // flex defaults to
    // flex-row no-wrap 1-col-margin hv-centered
    fn flex_layout(&self, area: &mut Area, pos: &mut Position, p: &Padding);

    fn generate_hpos(&self, p: &Padding) -> u16;
    fn generate_vpos(&self, p: &Padding) -> u16;

    // if grid cols and rows are not specified in properties
    // then grid acts like flex
    // that is automatic grid cols rows assignment a la flex
    fn grid_layout(&self, area: &mut Area, pos: &mut Position);
}

impl LayoutRules for Term {
    fn is_canvas(&self) -> bool {
        self.layout.is_canvas()
    }

    fn is_flex(&self) -> bool {
        self.layout.is_flex()
    }

    fn is_grid(&self) -> bool {
        self.layout.is_grid()
    }

    fn flex_layout(&self, area: &Area, pos: &Position, p: &Padding) {
        // x0 should be the x0 + w of last child so the new hpos
        // ycenter should be same for all so the parent's flex center
        //
    }

    // respect outer padding
    fn generate_x0(&self, por: u16) -> u16 {
        if let Some(last) = self.last() {
            last.x0 + last.w + 1 + por
        } else {
            0
        }
    }

    // respect outer padding
    fn generate_y0(&self, pot: u16) -> u16 {
        self.h / 2 + pot
    }

    fn grid_layout(&self, area: &Area, pos: &Position) {}
}
impl LayoutRules for Container {
    fn is_canvas(&self) -> bool {
        self.layout.is_canvas()
    }

    fn is_flex(&self) -> bool {
        self.layout.is_flex()
    }

    fn is_grid(&self) -> bool {
        self.layout.is_grid()
    }

    fn flex_layout(&self, area: &Area, pos: &Position) {}
    fn grid_layout(&self, area: &Area, pos: &Position) {}
}

impl Term {
    fn area_origin_recalculation(&self, mut child: Container) -> Container {
        match self.layout {
            Layout::Canvas => (),
            Layout::Flex => self.flex_recalculation(&mut child),
            Layout::Grid => self.grid_recalculation(&mut child),
        }

        child
    }

    fn area_origin_recalculation_scrollable(&self, mut child: Container) -> Container {
        match self.layout {
            Layout::Canvas => (),
            Layout::Flex => self.flex_recalculation(&mut child),
            Layout::Grid => self.grid_recalculation(&mut child),
        }

        child
    }

    fn space_recalculation(&self, mut child: Container) -> Container {
        match self.is_scrollable() {
            true => self.area_origin_recalculation_scrollable(child),
            false => self.area_origin_recalculation(child),
        }
    }

    fn flex_data(&self, child: &Container) {}

    fn grid_data(&self, child: &Container) {}

    fn flex_recalculation(&self, child: &mut Container) {}

    fn grid_recalculation(&self, child: &mut Container) {}

    fn check_area_origin(&self, child: &Container) -> bool {
        let [hpos, vpos] = [child.hpos, child.vpos];
        let [w, h] = [child.w, child.h];

        self.w * self.h < w * h
            || hpos > self.w
            || vpos > self.h
            || w > self.w
            || h > self.h
            || hpos + w > self.w
            || vpos + h > self.h
    }

    fn check_area_origin_scrollable(&self, child: &Container) -> bool {
        let [hpos, vpos] = [child.hpos, child.vpos];
        let [w, h] = [child.w, child.h];

        self.w * self.h < w * h || hpos > self.w || w > self.w || hpos + w > self.w
    }
}

struct Flex {
    // flex direction: row (new child to the right), col (new child to down)
    direction: u8,
    // row direction right becomes left
    // col direction down becomes up
    invert: bool,
    // padding between children
    margin: u16,
    // allign children to right center or left within their own padding included area
    h_align: char,
    // allign children to top center or bottom within their own padding included area
    v_align: char,
    // when end of width is reached, return to new flex line or not
    wrap: bool,
}

struct Grid {
    // how many columns to split the width by
    cols: u8,
    // how many rows to split the height by
    rows: u8,
    // direction of displaying new children
    // direction: u8,
}

impl Container {
    pub(crate) fn area_out_of_bounds(&self, wh: &[u16; 2]) -> bool {
        let [w, h] = *wh;
        if self.w * self.h < w * h || w > self.w || h > self.h {
            return true;
        }

        false
    }

    // flex and grid dont respect origins
    pub(crate) fn origin_out_of_bounds(&self, xy: &[u16; 2], wh: &[u16; 2]) -> bool {
        let [x0, y0] = *xy;
        let [w, h] = *wh;
        x0 > self.w || y0 > self.h || x0 + w > self.w || y0 + h > self.h
    }

    // TODO: make width/height augmented by paddings/border or both

    // caculate new child x0 y0 to fit flex layout of this parent
    fn layout_flex(&self, text: &mut Text) {
        self.texts.values().map(|t| t);
    }

    // calculate new child x0 y0 to fit grid layout of this parent
    fn layout_grid(&self, text: &mut Text) {}

    fn input_space_validation(&self, mut text: Text) -> Result<Text, SpaceError> {
        if self.area_out_of_bounds(&[text.w, text.h]) {
            return Err(SpaceError::AreaOutOfBounds);
        } else if self.origin_out_of_bounds(&[text.w, text.h], &[text.hpos, text.vpos]) {
            return Err(SpaceError::OriginOutOfBounds);
        }

        Ok(text)
    }
}

// checks for adding a component to its parent
// 1/ id check
// 2/ layout checks
//      2.1/ check parent layout
//      2.2/ if layout is flex then ignore x0 and y0 and apply flex rules
//          2.2.1/ before applying flex rules check overlay then check area bounds
//          2.2.2/ if overlay is off and area bounds are not respected abort with error else apply
//            rules and accept
//          2.2.3/ if overlay is on then as long as component area is not bigger than parent area
//            it will be accepted
//      2.3/ if layout is grid then ignore x0 and y0 and apply grid rules
//          2.3.1/ before applying grid rules check overlay then check area bounds
//          2.3.2/ if overlay is off and area bounds are not respected abort with error else appply
//            rules and accept
//          2.3.3/ if overlay is on then as long as component area is not bigger than parent area
//            it will be accepted
//      2.4/ if layout is canvas then don't apply any additional rules
//          2.4.1/ check overlay then check area bounds
//          2.4.2/ if overlay is off and area bounds are not respected abort with error else accept
//          2.4.3/ if overlay is on then as long as component area is not bigger than parent area
//            it will be accepted

// how do i handle bad area / position
// do i panic
// do i return an error (if so)
//      do i still make the comp
//      or do i not make it
//      if i make it do i push it
//      or do i not
//      if i push it do i render it
//      or do i not
// TODO: it's more useful to render it in the end
// so that user can debug easier with the broken rendered component
//
// as for id only comp methods with_id and new can generate bad id comps
// the only way to call the 2 methods is through parent push
// which rejects bad id and doesnt create nor return the bad id comp
