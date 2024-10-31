use super::focus::CompId;
use super::property::Properties;
use super::*;
use crate::space::{Area, Origin, Pos};

#[derive(Debug)]
pub enum ValidationError {
    IdInUse,
    WidthOverflow,
    HeightOverflow,
    WHOverflow,
    XOutofBounds,
    YOutOfBounds,
}

pub struct Position {
    x: u8,
}

pub trait MetaValidation {
    fn id_in_use(&self, id: impl CompId) -> bool;

    fn generate_id(&self, bad: u8) -> u8;

    fn layout_is_valid(&self, area: &Area, pos: &Position) -> bool /* Result<(), ValidationError> */;

    // this has to work with layout submodule form the space module
    fn shift_position(&self, vertices: Vertices) -> [Pos; 4] {
        [Pos::Start; 4]
    }
}

// NOTE: builder and existing component both have id, area and position
// they can run all the checks normally
// clean component from parent can be given an auto id
// as for area it can take the last child area
// as for position, depends on layout
//      if canvas overlay on top of last one
//      if flex move to next flex position
//      if grid move to next grid position

struct Vertices {
    value: [u16; 4],
}

// NOTE all check traits are super traits of ParentComponent
// NOTE: Rc<RefCell>

impl MetaValidation for ComponentTree {
    /// checks for the existence of a Term with the provided id inside this tree
    fn id_in_use(&self, id: impl CompId) -> bool {
        if id.is_term() {
            self.terms.contains_key(&id)
        } else if id.is_container() {
            self.current().containers.contains_key(&id)
        } else if id.is_text() {
            self.current()
                .child_ref(&[id[0], id[1]])
                .texts
                .contains_key(id)
        }
    }

    fn generate_id(&self, bad: u8) -> u8 {
        // TODO: if bad > 127 then id = 254 and sub else 0 and add
        let mut id = 0;
        for tid in self.terms.keys() {
            if tid == id {
                id += 1;
            } else {
                break;
            }
        }

        id
    }

    fn layout_is_valid(&self, area: &Area, pos: &Position) -> bool /* Result<(), ValidationError> */
    {
        Ok(())
    }
}

impl MetaValidation for Term {
    fn id_in_use(&self, id: impl CompId) -> bool {
        if id.is_container() {
            self.containers.contains_key(&id)
        } else if id.is_text() {
            self.child_ref(&[id[0], id[1]]).texts.contains_key(id)
        }
    }

    fn generate_id(&self, bad: u8) -> u8 {
        let mut id = 0;
        for cid in self.containers.keys() {
            if cid[1] == id {
                id += 1;
            } else {
                break;
            }
        }

        id
    }

    fn layout_is_valid(&self, area: &Area, pos: &Position) -> bool {
        let child = Text::default();
        let [hpos, vpos] = [child.hpos, child.vpos];
        let [w, h] = [child.w, child.h];

        self.w * self.h < w * h
            || hpos > self.w
            || vpos > self.h
            || w > self.w
            || h > self.h
            || hpos + w > self.w
            || vpos + h > self.h

        // let [w, h] = [area.w().unwrap(), area.h.unwrap()];
        //         if self.w < w && self.h < h {
        //             return Err(AreaError::WHOverflow);
        //         } else if self.h < h {
        //             return Err(AreaError::HeightOverflow);
        //         } else if self.w < w {
        //             return Err(AreaError::WidthOverflow);
        //         }
        //
        //         Ok(())

        // if let Pos::Value(x) = origin.x() {
        //             if x > self.w {
        //                 return Err(OriginError::XOutOfBounds);
        //             }
        //         }
        //         if let Pos::Value(y) = origin.y() {
        //             if y > self.h {
        //                 return Err(OriginError::YOutOfBounds);
        //             }
        //         }
        //
        //         Ok(())
    }
}

impl MetaValidation for Container {
    fn id_in_use(&self, id: impl CompId) -> bool {
        self.texts.contains_key(&id)
    }

    fn generate_id(&self, bad: u8) -> u8 {
        let mut id = if bad % 2 == 0 { 0 } else { 1 };
        for c in self.texts.values() {
            if c.id[2] == id {
                id += 2;
            } else {
                break;
            }
        }

        id
    }

    // has nothing to do with the layout field
    // checks validity of area and position
    fn layout_is_valid(&self, area: &Area, pos: &Position) -> bool {
        // let [w, h] = [area.w().unwrap(), area.h().unwrap()];
        // if self.w < w && self.h < h {
        //     return Err(AreaError::WHOverflow);
        // } else if self.h < h {
        //     return Err(AreaError::HeightOverflow);
        // } else if self.w < w {
        //     return Err(AreaError::WidthOverflow);
        // }
        //
        // Ok(())
        true
    }

    fn shift_position(&self, vertices: Vertices) -> [Pos; 4] {}
}

// NOTE: a component position/area includes it padding and border
// ie, padding doesn't leave the bounds of origin
