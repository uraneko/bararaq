use super::*;
use crate::space::Area;

pub trait Component {
    type Id;
    fn comp(value: impl TryFrom<Self>) -> Result<Self, CompError>;

    fn id(&self) -> Self::Id;

    fn area(&mut self, area: impl Into<Area>) {
        // check if new area is valid
        // this cant work without the parent area
        if self.validate_area(&area) {
            // change area
            self.w = area.w();
            self.h = area.h();
        }
    }

    fn color(&mut self, color: impl Into<Style>) {
        self.colorscheme.color = color;
    }

    fn background(&mut self, b: impl Into<Style>) {
        self.colorscheme.background = b;
    }

    fn border_color(&mut self, bc: impl Into<Style>) {
        self.colorscheme.border_color = bc;
    }

    fn border_background(&mut self, bb: impl Into<Style>) {
        self.colorscheme.border_background = bb;
    }
}

enum CompError {}

impl Component for Term {
    type Id = u8;

    fn comp(value: impl TryFrom<Self>) -> Result<Self, CompError> {
        value.try_into()
    }

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl Component for Container {
    type Id = [u8; 2];

    fn comp(value: impl TryFrom<Self>) -> Result<Self, CompError> {
        value.try_into()
    }

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl Component for Text {
    type Id = [u8; 3];

    fn comp(value: impl TryFrom<Self>) -> Result<Self, CompError> {
        value.try_into()
    }

    fn id(&self) -> Self::Id {
        self.id
    }
}

use crate::space::layout::*;

// NOTE: only Tree, Term and Container implement this
// : Component + MetaValidation + LayoutRules
pub(crate) trait ParentComponent<Comp> {
    type Error;

    /// pushes an existing Term to this tree's terms vector
    ///
    /// # Examples
    ///
    /// ## Failure
    ///
    /// ```
    /// let mut tree = ComponentTree::new();
    /// let term = Term::new(0);
    /// assert!(tree.push_term(term).is_err());
    /// ```
    ///
    /// ## Success
    /// ```
    /// let mut tree = ComponentTree::new();
    /// let term = Term::new(1);
    /// assert!(tree.push_term(term).is_ok());
    /// ```
    ///
    /// # Errors
    /// returns an error only if the new Term's id is already taken by another Term in this Tree
    ///
    fn push(&mut self, maker: impl Into<Comp>) -> Result<Comp, Self::Error>;

    /// removes the term with the given id  from this component tree and returns it
    /// returns None if such a term does not exist
    fn remove(&mut self, id: u8) -> Option<Comp>;

    /// returns an optional immutable reference of the term with the provided id if it exists
    fn child_ref(&self, id: u8) -> Option<&Comp>;

    /// returns an optional mutable reference of the term with the provided id if it exists
    fn child_mut(&mut self, id: u8) -> Option<&mut Comp>;

    /// returns the number of containers inside this term
    fn len(&self) -> usize;

    /// returns the number of containers with a property inside this term
    fn plen(&self, prop: &str) -> usize;

    /// returns the number of containers with an attribute inside this term
    fn alen(&self, attr: &str) -> usize;

    fn last(&self) -> Option<&Comp>;

    fn children_by_area(&self, value: &impl Into<Area>) -> Option<&[Comp]>;
    fn children_by_position(&self, value: &impl Into<Position>) -> Option<&[Comp]>;
    fn children_by_border(&self, value: &impl Into<Border>) -> Option<&[Comp]>;
    fn children_by_padding(&self, value: &impl Into<Padding>) -> Option<&[Comp]>;
    fn children_by_layer(&self, value: &u16) -> Option<&[Comp]>;
    fn children_by_property(&self, key: &str, value: &impl Into<Property>) -> Option<&[Comp]>;
    fn children_by_attribute(&self, value: &str) -> Option<&[Comp]>;
    fn children_by_colorscheme(&self, value: &Style) -> Option<&[Comp]>;
    // if children are text then they dont have layout
    // implement this method individually
    // fn children_by_layout(&self, value: &Layout) -> Option<&[Comp]>;
    fn child_by_id(&self, value: &impl CompId) -> Option<&Comp>;
    fn child_by_built_on(&self, value: u32) -> Option<&Comp>;
}

use super::focus::CompId;

// TODO: makers are : id(u8) \ () \ Builder \ Comp
// NOTE: use Rc<RefCell>

impl ParentComponent<Term> for ComponentTree {
    type Error = TreeError;

    fn push(&mut self, src: impl Into<Term>) -> Result<Term, Self::Error> {
        Ok(src.into())
    }

    fn remove(&mut self, id: u8) -> Option<Term> {
        self.terms.remove(&id)
    }

    fn child_ref(&self, id: u8) -> Option<&Term> {
        self.terms.get(&id)
    }

    fn child_mut(&mut self, id: u8) -> Option<&mut Term> {
        self.terms.get_mut(&id)
    }

    fn len(&self) -> usize {
        self.terms.len()
    }

    fn plen(&self, prop: &str) -> usize {
        self.terms.values().fold(0, |acc, t| {
            acc + if t.properties.contains_key(prop) {
                1
            } else {
                0
            }
        })
    }

    fn alen(&self, attr: &str) -> usize {
        self.terms
            .values()
            .map(|t| t.attriutes)
            .filter(|a| a.contains(attr))
            .count()
    }

    fn last(&self) -> Option<&Term> {
        if let Some(time) = self.terms.values().map(|t| t.built_on).max() {
            return self.child_by_built_on(time.into());
        }

        None
    }

    fn children_by_area(&self, value: &impl Into<Area>) -> Option<&[Term]> {
        None
    }
    fn children_by_position(&self, value: &impl Into<Position>) -> Option<&[Term]> {
        None
    }
    fn children_by_border(&self, value: &impl Into<Border>) -> Option<&[Term]> {
        None
    }
    fn children_by_padding(&self, value: &impl Into<Padding>) -> Option<&[Term]> {
        None
    }
    fn children_by_layer(&self, value: &u16) -> Option<&[Term]> {
        None
    }
    fn children_by_property(&self, key: &str, value: &impl Into<Property>) -> Option<&[Term]> {
        None
    }
    fn children_by_attribute(&self, value: &str) -> Option<&[Term]> {
        None
    }
    fn children_by_colorscheme(&self, value: &Style) -> Option<&[Term]> {
        None
    }
    // if children are text then they dont have layout
    // implement this method individually
    // fn children_by_layout(&self, value: &Layout) -> Option<&[Term]> {None}
    fn child_by_id(&self, value: &impl CompId) -> Option<&Term> {
        None
    }
    fn child_by_built_on(&self, value: u32) -> Option<&Term> {
        None
    }
}

use super::property::Property;

enum TermError {}
enum ContainerError {}
enum TextError {}

impl ParentComponent<Container> for Term {
    type Error = TermError;

    fn push(&mut self, maker: impl Into<Container>) -> Result<Container, Self::Error> {}

    fn remove(&mut self, id: u8) -> Option<Container> {
        self.containers.remove(&[self.id, id])
    }

    fn child_ref(&self, id: u8) -> Option<&Container> {
        self.containers.get(&[self.id, id])
    }

    fn child_mut(&mut self, id: u8) -> Option<&mut Container> {
        self.containers.get_mut(&[self.id, id])
    }

    fn len(&self) -> usize {
        self.containers.len()
    }

    fn plen(&self, prop: &str) -> usize {
        self.containers
            .values()
            .map(|t| t.properties)
            .filter(|p| p.contains_key(prop))
            .count()
    }

    fn alen(&self, attr: &str) -> usize {
        self.containers
            .values()
            .map(|t| t.attriutes)
            .filter(|a| a.contains(attr))
            .count()
    }
}

impl ParentComponent<Text> for Container {
    type Error = TextError;

    fn push(&mut self, maker: impl Into<Text>) -> Result<Text, Self::Error> {}

    fn remove(&mut self, id: u8) -> Option<Text> {
        self.texts.remove(&[self.id[0], self.id[1], id])
    }

    fn child_ref(&self, id: u8) -> Option<&Container> {
        self.texts.get(&[self.id[0], self.id[1], id])
    }

    fn child_mut(&mut self, id: u8) -> Option<&mut Container> {
        self.texts.get_mut(&[self.id[0], self.id[1], id])
    }

    fn len(&self) -> usize {
        self.texts.len()
    }

    fn plen(&self, prop: &str) -> usize {
        self.texts
            .values()
            .map(|t| t.properties)
            .filter(|p| p.contains_key(prop))
            .count()
    }

    fn alen(&self, attr: &str) -> usize {
        self.texts
            .values()
            .map(|t| t.attriutes)
            .filter(|a| a.contains(attr))
            .count()
    }
}

//////////////////////////////////////////////////////////////////////////////

// 1 //

// public api for parent
// push <- takes impl ComponentConstructor
// ref mut remove

pub trait GrandParentComponent<Comp>: ParentComponent<ParentComp>
where
    ParenComp: ParentComponent<Comp>,
{
    type GrandchildId;
    type Error;

    /// pushes a grandchild of this object
    fn push_grandchild(&mut self, maker: impl Into<Comp>) -> Result<Comp, Self::Error>;

    fn grandchild_ref(&self, id: Self::GrandchildId) -> Option<&Comp>;

    fn grandchild_mut(&mut self, id: Self::GrandchildId) -> Option<&mut Comp>;
}

impl GrandParentComponent<Container> for ComponentTree {
    type GrandchildId = [u8; 2];
    type Error = TreeError;

    fn push_grandchild(&mut self, maker: impl Into<Container>) -> Result<Container, Self::Error> {}

    fn grandchild_ref(&self, id: Self::GrandchildId) -> Option<&Container> {
        if let Some(term) = self.terms.get(&id[0]) {
            if let Some(container) = term.get(&id) {
                Some(container)
            }
        }

        None
    }

    fn grandchild_mut(&mut self, id: Self::GrandchildId) -> Option<&mut Container> {
        if let Some(term) = self.terms.get_mut(&id[0]) {
            if let Some(container) = term.get_mut(&id) {
                Some(container)
            }
        }

        None
    }
}

impl GrandParentComponent<Text> for Term {
    type GrandchildId = [u8; 3];
    type Error = TermError;

    fn push_grandchild(&mut self, maker: impl Into<Text>) -> Result<Text, Self::Error> {}

    fn grandchild_ref(&self, id: Self::GrandchildId) -> Option<&Text> {
        if let Some(term) = self.terms.get(&id[0]) {
            if let Some(container) = term.get(&[id[0], id[1]]) {
                if let Some(text) = container.get(&id) {
                    Some(text)
                }
            }
        }

        None
    }

    fn grandchild_mut(&mut self, id: Self::GrandchildId) -> Option<&mut Text> {
        if let Some(term) = self.terms.get_mut(&id[0]) {
            if let Some(container) = term.get_mut(&[id[0], id[1]]) {
                if let Some(text) = container.get_mut(&id) {
                    Some(text)
                }
            }
        }

        None
    }
}
