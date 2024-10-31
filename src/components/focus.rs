use super::*;
pub enum ComponentError {}

pub(crate) trait CompId {
    fn is_tree(&self) -> bool {
        false
    }
    fn is_term(&self) -> bool {
        false
    }
    fn is_container(&self) -> bool {
        false
    }
    fn is_text(&self) -> bool {
        false
    }
    fn is_input(&self) -> bool {
        false
    }
    fn is_noedit(&self) -> bool {
        false
    }
}

// impl CompId for () {
//     fn len(&self) -> usize {
//         0
//     }
//
//     fn is_tree(&self) -> bool {
//         true
//     }
// }

impl CompId for u8 {
    fn is_term(&self) -> bool {
        true
    }
}

impl CompId for [u8; 2] {
    fn is_container(&self) -> bool {
        true
    }
}

impl CompId for [u8; 3] {
    fn is_text(&self) -> bool {
        true
    }
    fn is_input(&self) -> bool {
        self[2] % 2 == 0
    }
    fn is_noedit(&self) -> bool {
        self[2] % 2 != 0
    }
}

pub trait ComponentFocus {
    fn is_focused(&self) -> bool {
        true
    }

    fn give_focus(&mut self, id: [u8; 3]) -> [u8; 3];

    // returns the full id of the currently focused text component
    fn focused(&self) -> [u8; 3];
}

impl ComponentFocus for ComponentTree {
    fn give_focus(&mut self, id: [u8; 3]) -> [u8; 3] {
        let old = self.terms.values().find(|t| {
            t.containers
                .values()
                .find(|c| c.text.attributes.contains("focus"))
        });

        if old.is_none() {
            return old;
        }

        if self.id_in_use(id) {
            self.text_mut(id).attributes.push("focused");
        } else {
            return None;
        }

        old
    }

    fn focused(&self) -> [u8; 3] {
        self.current().containers.values().find(|c| {
            c.texts
                .values()
                .find(|text| text.attributes.contains("focused"))
        })
    }
}

impl ComponentFocus for Term {
    fn is_focused(&self) -> bool {
        self.containers
            .values()
            .find(|t| {
                t.containers
                    .values()
                    .find(|c| c.text.attributes.contains("focused"))
            })
            .is_some()
    }

    fn give_focus(&mut self, id: [u8; 3]) -> [u8; 3] {}

    fn focused(&self) -> [u8; 3] {}
}

// NOTE: focused and current are not optional
// if there are no comps carrying the two attributes then that is an error

// impl current methods for term and tree
// tree
// fn current() -> u8
// fn make_current() -> result<u8, currenterror>
//
// term
// is_current() -> bool
