use crate::colorscheme::Style;
use crate::console::winsize::winsize;
use crate::render_pipeline;
use crate::space::{area_conflicts, between, border::Border, border_fit, padding::Padding};

use std::collections::{HashMap, HashSet};
use std::io::Error;
use std::io::StdoutLock;
use std::io::Write;

pub mod builders;
pub mod checks;
pub mod container;
pub mod focus;
pub mod makers;
pub mod parent;
pub mod property;
pub mod scrollable;
pub mod term;
pub mod text;

pub(crate) use checks::*;
// re-exports
pub use builders::{BuilderError, BuilderTreeError};
pub use builders::{ContainerBuilder, InputBuilder, NoEditBuilder, TermBuilder};
pub use container::Container;
pub(crate) use makers::*;
pub use term::Term;
pub use text::Text;

use std::any::Any;

// TODO: text should be all one component
// no difference between non editable and input
// simply add an editable bool field

#[derive(Debug)]
pub enum InitError {
    TermNotMade,
    InvalidTerm { term: Term },
}

#[derive(Debug)]
pub enum IdError {
    IdAlreadyTaken,
    IdInUse,
}

/// errors for ComponentTree operations
#[derive(Debug)]
pub enum TreeError {
    NoFocusedComp,
    NoFocusedTerm,
    NoFocusedContainer {
        tid: u8,
    },
    NoFocusedText {
        cid: [u8; 2],
    },
    /// tree initialization errors
    InitError(InitError),
    /// id errors
    IdError(IdError),
    /// space errors
    SpaceError(SpaceError),
    /// id errors
    /// Obscure error; something about some id somewhere went wrong
    BadID,
    ///
    BadValue,
    /// when trying to assugn an ID that has already been assigned prior to this
    IDAlreadyExists,
    /// the parent object of some object that is being operated on was not found in this tree
    ParentNotFound,
    /// the space bounds rules were broken by some object trying to join this tree
    BoundsNotRespected,
}

/// the wrpper struct holding all the program term objects
#[derive(Debug)]
pub struct ComponentTree {
    /// the term bufs collection
    terms: HashMap<u8, Term>,
    /// window size of the terminal window
    ws: winsize,
    // pipes: PipeStream,
}

impl ComponentTree {
    /// creates a new ComponentTree instance
    /// normally, this should only be used once in a crate
    /// # Examples
    /// ```
    /// let tree = ComponentTree::new();
    /// ```
    /// this automatically creates a new Term with the id value of 0 inside this new Tree
    pub fn new() -> Self {
        let ws = winsize::from_ioctl();

        Self {
            terms: HashMap::from([(0, Term::new(0, ws.cols(), ws.rows()))]),
            ws,
        }
    }

    // pushing a term requires:
    // 1 id validation: fn has_term done
    // 2 layout based origin recalculation
    // 3 area and origin validation
    // 4 creating the comp
    // 5 pushing the comp

    pub fn term(&mut self, maker: impl Into<Term>) -> Result<(), TreeError> {
        match maker.maker_id() {
            2 => self.term_from_existing(
                (*(Box::new(maker) as Box<dyn Any>)
                    .downcast::<Term>()
                    .unwrap()),
            ),
            1 => self.term_from_builder(
                ((Box::new(maker) as Box<dyn Any>)
                    .downcast_mut::<TermBuilder>()
                    .unwrap()),
            ),
            0 => self.term_from_id((*(Box::new(maker) as Box<dyn Any>).downcast::<u8>().unwrap())),
            _ => panic!(),
        };

        Ok(())
    }

    fn term_from_builder(&mut self, builder: &mut TermBuilder) -> Result<(), TreeError> {
        if let Some(err) = self.is_valid_builder(&builder) {
            match err {
                BuilderTreeError::IdInUse => {
                    return Err(TreeError::InitError(InitError::TermNotMade))
                }
            }
        }
        self.push(builder.build());

        Ok(())
    }

    fn term_from_existing(&mut self, term: Term) -> Result<(), TreeError> {
        if !self.is_valid_term(&term) {
            return Err(TreeError::InitError(InitError::InvalidTerm { term }));
        }
        self.push(term);

        Ok(())
    }

    /// Term in this tree
    pub fn term_from_id(&mut self, id: u8) -> Result<(), TreeError> {
        if self.has_term(id) {
            eprintln!("bad id");
            return Err(TreeError::IdError(IdError::IdAlreadyTaken));
        }
        self.push(Term::new(id, self.ws.cols(), self.ws.rows()));

        Ok(())
    }

    /// adds a new Term object to this tree
    /// takes an id value for the new Term
    /// # Errors
    ///
    /// this method returns an ComponentTreeError if the id provided is already being used by another

    pub fn focused_extended(&self) -> Result<[u8; 3], TreeError> {
        let t = self.terms.iter().find(|(_, t)| t.is_focused());
        if let Some((tid, term)) = t {
            let c = term.containers.iter().find(|(id, cont)| cont.is_focused());
            if let Some((cid, cont)) = c {
                let txt = cont.texts.iter().find(|(id, txt)| txt.is_focused());
                if let Some((id, _)) = txt {
                    return Ok(*id);
                }

                return Err(TreeError::NoFocusedText { cid: *cid });
            }

            return Err(TreeError::NoFocusedContainer { tid: *tid });
        }

        Err(TreeError::NoFocusedComp)
    }

    /// changes the active Term of this tree
    /// the active term is the term that gets rendered
    ///
    /// # Errors
    ///
    /// returns an error if a Term with the provided id does not exist in this tree
    pub fn focus(&mut self, id: u8) -> Result<(), TreeError> {
        if self.has_term(id) {
            self.term_mut(id).unwrap().attributes.insert("focused");

            return Ok(());
        }

        Err(TreeError::BadID)
    }

    /// takes no id and automatically assigns an id while adding a new Term
    /// returns the new term id
    pub fn term_auto(&mut self) -> u8 {
        let id = self.assign_term_id();

        self.terms
            .insert(id, Term::new(id, self.ws.cols(), self.ws.rows()));

        id
    }

    // methods of the has_object series do not check for duplicate ids
    // because those are already being screened by earlier id assignment methods
    // and there is no way in the api to bypass those checks and push an object to the tree
    // which means that duplicate ids can never happen

    // FIXME: this has to also resize all children
    // then this gets called inside a render_resize method
    fn resize(&mut self) {
        let ws = winsize::from_ioctl();
        let [cols, rows] = [ws.cols(), ws.rows()];

        self.ws = ws;
        self.terms.iter_mut().map(|(_, t)| t).for_each(|t| {
            t.w = cols;
            t.h = rows;
        });
    }
}

#[derive(Debug)]
pub enum SpaceError {
    AreaOutOfBounds,
    OriginOutOfBounds,
}

#[cfg(test)]
mod tree {
    use super::{ComponentTree, Term, TreeError};

    #[test]
    fn active() {
        let mut tree = ComponentTree::init();
        assert!(tree.term_from_id(0).is_err());
        assert_eq!(tree.terms.len(), 1);

        tree.term_from_id(7);

        assert_eq!(tree.terms.len(), 2);

        assert_eq!(tree.focused().unwrap(), 0);
        tree.focus(3);
        assert_eq!(tree.focused().unwrap(), 0);
        tree.focus(7);
        assert_eq!(tree.focused().unwrap(), 7);
    }

    #[test]
    fn assign() {
        let mut tree = ComponentTree::init();
        tree.term_auto();
        assert!(tree.has_term(0));
        let term: &Term = tree.term_ref(0).unwrap();
        let term: &mut Term = tree.term_mut(0).unwrap();
        assert!(tree.term_ref(78).is_none());
        tree.term_from_id(1);
        tree.term_from_id(2);
        tree.term_from_id(4);
        assert_eq!(tree.assign_term_id(), 3);
    }
}

#[cfg(test)]
mod test_term {
    use super::{Container, Term, Text};

    #[test]
    fn area() {
        let mut term = Term::new(5, 500, 500);
        let mut c1 = Container::default();
        c1.w = 24;
        c1.h = 32;
        c1.hpos = 2;
        c1.vpos = 5;
        assert!(term.assign_valid_container_area(&c1).is_ok());
        c1.w = 8354;
        c1.h = 3;
        c1.hpos = 2;
        c1.vpos = 5;
        assert!(term.assign_valid_container_area(&c1).is_err());
        c1.w = 4;
        c1.h = 8324;
        c1.hpos = 2;
        c1.vpos = 5;
        assert!(term.assign_valid_container_area(&c1).is_err());
        c1.w = 4;
        c1.h = 3;
        c1.hpos = 8355;
        c1.vpos = 5;
        assert!(term.assign_valid_container_area(&c1).is_err());
        c1.w = 4;
        c1.h = 3;
        c1.hpos = 2;
        c1.vpos = 8653;
        assert!(term.assign_valid_container_area(&c1).is_err());
    }

    // FIXME: tests have been broken

    #[test]
    fn active() {
        let mut term = Term::new(5, 500, 500);
        let mut c = Container::default();
        c.hpos = 4;
        c.vpos = 5;
        c.id = [5, 1];
        let mut ne = Text::default();
        ne.hpos = 7;
        ne.vpos = 4;
        ne.id = [5, 1, 3];

        // when more area manipulation methods are written
        // return to this and make it a proper check of those methods
        [ne.hpos, ne.vpos] = [ne.hpos + c.hpos, ne.vpos + c.vpos];

        let id = ne.id;

        term.push_container(c);
        term.push_noedit(ne);

        let res = term.focus(&[5, 1, 8]);
        assert!(res.is_err());

        let res = term.focus(&id);
        assert!(res.is_ok());
        assert_eq!(term.focused().unwrap(), [0, 8]);
        assert_eq!(term.focused().unwrap(), [11, 9]);
    }

    use crate::space::{border::Border, padding::Padding, Area, Pos};

    #[test]
    fn cursor() {
        let mut term = Term::new(5, 500, 500);
        _ = term.container(
            &[0, 0],
            Pos::Value(56),
            Pos::Value(15),
            Pos::Value(0),
            Area::Values { w: 35, h: 8 },
            Border::None,
            Padding::None,
        );
        _ = term.input(
            &[0, 0, 0],
            Pos::Value(1),
            Pos::Value(1),
            Pos::Value(0),
            Area::Values { w: 23, h: 2 },
            Border::None,
            Padding::None,
        );

        let res = term.focus(&[0, 0, 0]);
        assert_eq!([term.crsh, term.crsv], [56 + 1 + 1, 15 + 1]);
    }

    #[test]
    fn objects() {
        let mut term = Term::new(0, 600, 600);
        term.push_container(Container::default());
        term.container(
            &[0, 1],
            Pos::Value(56),
            Pos::Value(15),
            Pos::Value(0),
            Area::Values { w: 35, h: 18 },
            Border::None,
            Padding::None,
        );
        assert_eq!(term.containers.len(), 2);
        term.push_input({ Text::default() });
        term.noedit(
            &[0, 1, 1],
            Pos::Value(12),
            Pos::Value(12),
            Pos::Value(0),
            Area::Values { w: 2, h: 2 },
            Border::None,
            Padding::None,
            &[],
        );

        assert_eq!(term.tlen(), 2)
    }

    // test calc_text_abs_ori

    #[test]
    fn objects1() {
        let mut term = Term::new(0, 500, 500);
        term.push_container(Container::default());
        term.container(
            &[0, 1],
            Pos::Value(56),
            Pos::Value(15),
            Pos::Value(0),
            Area::Values { w: 35, h: 18 },
            Border::None,
            Padding::None,
        );
        assert_eq!(term.containers.len(), 2);
        term.push_input({ Text::default() });
        term.noedit(
            &[0, 1, 1],
            Pos::Value(12),
            Pos::Value(12),
            Pos::Value(0),
            Area::Values { w: 2, h: 2 },
            Border::None,
            Padding::None,
            &[],
        );

        assert_eq!(term.tlen(), 2);
    }

    #[test]
    fn objects_count() {
        let mut term = Term::new(0, 500, 500);

        term.container(
            &[0, 0],
            Pos::Value(5),
            Pos::Value(5),
            Pos::Value(0),
            Area::Values { w: 10, h: 10 },
            Border::None,
            Padding::None,
        );
        term.container(
            &[0, 1],
            Pos::Value(15),
            Pos::Value(15),
            Pos::Value(0),
            Area::Values { w: 10, h: 10 },
            Border::None,
            Padding::None,
        );
        term.container(
            &[0, 2],
            Pos::Value(25),
            Pos::Value(25),
            Pos::Value(0),
            Area::Values { w: 10, h: 10 },
            Border::None,
            Padding::None,
        );

        term.input(
            &[0, 2, 0],
            Pos::Value(1),
            Pos::Value(2),
            Pos::Value(0),
            Area::Values { w: 2, h: 2 },
            Border::None,
            Padding::None,
        );
        term.noedit(
            &[0, 1, 1],
            Pos::Value(2),
            Pos::Value(2),
            Pos::Value(0),
            Area::Values { w: 2, h: 2 },
            Border::None,
            Padding::None,
            &[],
        );

        term.noedit(
            &[0, 0, 1],
            Pos::Value(1),
            Pos::Value(1),
            Pos::Value(0),
            Area::Values { w: 2, h: 2 },
            Border::None,
            Padding::None,
            &[],
        );

        assert_eq!(term.tlen(), 3);
        assert_eq!(term.ilen(), 1);
        assert_eq!(term.nelen(), 2);
        // assert_eq!(term.itlen(), 2);
        // assert_eq!(term.nitlen(), 1);
    }
}

// TODO: move space related method into the space module
// NOTE: commit 'f22c752' mentions fixing 'some bug/errors'
// amongst those was an object area validation bug which made valid areas not pass the check
// should have mentioned it by name in the commit message

#[cfg(test)]
mod test_container {}

#[cfg(test)]
mod test_text {}
