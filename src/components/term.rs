use std::collections::{HashMap, HashSet};
use std::io::StdoutLock;
use std::io::Write;

use crate::components::property::{Properties, Property};
use crate::console::winsize::winsize;
use crate::render_pipeline;
use crate::space::layout::Layout;
use crate::space::{
    area_conflicts, between, border::Border, border_fit, calc_text_abs_ori, padding::Padding,
    resolve_wh, Area, Pos,
};

use super::Style;
use super::{ContainerBuilder, InputBuilder, NoEditBuilder};

use super::{Container, Text};
use super::{IdError, SpaceError, TreeError};

/// Term object that is basically the overall wrapper around back end for the terminal display
#[derive(Debug, Default)]
pub struct Term {
    /// this term's layout, applies only to direct children
    pub layout: Layout,
    /// this Term's id
    pub id: u8,
    /// the width of the terminal window
    pub w: u16,
    /// the height of the terminal window
    pub h: u16,
    /// the current terminal cursor x coordinate
    pub crsh: u16,
    /// the current terminal cursor y coordinate
    pub crsv: u16,
    /// a vector of all the Containers inside this Term
    pub containers: HashMap<[u8; 2], Container>,
    // pub border: Border,
    // pub padding: Padding,
    /// the active Text object of this Term
    /// it is the Text object that the Term recognizes the user to be interacting with currently
    // should be an attribute
    // pub focused: Option<[u8; 3]>,
    /// properties that help with extended behavior for Terms
    /// e.g., flex-direction: row
    pub properties: Properties,
    /// attributes are like properties but they dont have values, only names
    /// e.g., focusable
    pub attributes: HashSet<&'static str>,
    pub built_on: std::time::Instant,
}

impl Term {
    /// returns a new term that holds the provided id
    ///
    /// # Examples
    /// ```
    /// let term = Term::new(0);
    /// ```
    ///
    /// # Errors
    ///
    /// the recommended way of creating a Term when a program uses more than 1 Term is to call the ComponentTree method term(id: u8)
    /// the term method would always validate the the new id before creating a term inside the tree
    /// if this function is called alongside tree's push_term() method then validating this term's
    /// id becomes the caller's job
    pub fn new(id: u8, w: u16, h: u16) -> Self {
        Term {
            id,
            w,
            h,
            ..Self::default()
        }
    }

    pub fn is_focused(&self) -> bool {
        self.attributes.contains("focused")
    }

    pub fn has_attribute(&self, attr: &str) -> bool {
        self.attributes.contains(attr)
    }

    pub fn with_area(id: u8) -> Self {
        let ws = winsize::from_ioctl();
        Term {
            id,
            w: ws.cols(),
            h: ws.rows(),
            ..Default::default()
        }
    }

    // since overlay is not implemented yet, this doesn't assign anything but just checks that the
    // area is valid
    // called on container auto and basic initializers
    pub(super) fn assign_valid_container_area(
        &self, // term
        cont: &Container,
        // layer: u8,
    ) -> Result<(), SpaceError> {
        let [hpos, vpos] = [cont.hpos, cont.vpos];
        let [w, h] = cont.decorate();

        if self.w * self.h < w * h
            || hpos > self.w
            || vpos > self.h
            || w > self.w
            || h > self.h
            || hpos + w > self.w
            || vpos + h > self.h
        {
            return Err(SpaceError::AreaOutOfBounds);
        }

        let mut e = 0;

        self.containers.values().for_each(|c| {
            if e == 0 {
                let [top, right, bottom, left] =
                    area_conflicts(hpos, vpos, cont.w, cont.h, c.hpos, c.vpos, c.w, c.h);
                // conflict case
                if (left > 0 || right < 0) && (top > 0 || bottom < 0) {
                    // TODO: actually handle overlay logic
                    e = 1;
                }
            }
        });

        if e == 1 {
            return Err(SpaceError::OriginOutOfBounds);
        }

        Ok(())
    }

    // /// makes sure that container objects are properly positioned by moving them until they don't overlap when overlay is off
    // pub fn shift_container_area(&self, text: &mut Text) -> Result<(), SpaceError> {
    //     Ok(())
    // }
}

impl Term {
    /// syncs the position of the cursor in the term display to match the data in the backend
    pub fn sync_cursor(&mut self) -> Result<(), TreeError> {
        let id = self.focused().unwrap();
        let id = [0; 3];
        let text = if id[2] % 2 == 0 {
            self.input_ref(&id)
        } else {
            self.noedit_ref(&id)
        }
        .unwrap();

        let [cx, cy] = [text.ahpos + text.crsh, text.avpos + text.crsv];

        self.crsh = cx;
        self.crsv = cy;

        Ok(())
    }

    /// makes the text object with the given id the term's current active object
    /// places cursor in the new position by calling sync_cursor
    // TODO: probably make the entire focus part of bararaq-extra crate
    pub fn focus(&mut self, id: &[u8; 3]) -> Result<(), TreeError> {
        let condition = match id[2] % 2 == 0 {
            true => self.has_input(&id),
            false => self.has_noedit(&id),
        };

        if !condition {
            return Err(TreeError::BadID);
        }

        self.focus(id);
        self.sync_cursor();

        Ok(())
    }

    /// returns a result of the active text object absolute orign coords
    /// or an error if it doesn't exist
    pub fn focused(&self) -> Result<[u8; 2], TreeError> {
        // if self.active.is_none() {
        //     return Err(ComponentTreeError::BadID);
        // }

        // BUG: same bug unwrap_or skips unwrap and automaticall does or in tests
        // let id = self.active.unwrap_or(return Err(ComponentTreeError::BadID));
        let id = match self.focused() {
            Ok(id) => id,
            Err(e) => return Err(TreeError::BadID),
        };

        let id = [0, 0, 0];

        match id[2] % 2 == 0 {
            true => {
                let t = self.input_ref(&id).unwrap();
                Ok([t.ahpos as u8, t.avpos as u8])
            }
            false => {
                let t = self.noedit_ref(&id).unwrap();
                Ok([t.ahpos as u8, t.avpos as u8])
            }
        }
    }
}

impl Term {
    /// adds a new Container object to this Term's containers
    ///
    /// # Examples
    /// ```
    /// let mut term = Term::new(0);
    /// let res = term.container(&[0, 0], 3, 3, 34, 18, Border::Uniform('+'), Padding::None);
    /// assert!(res.is_ok());
    /// ```
    /// # Errors
    /// returns an error if any of the following condition are met
    /// - the provided id is not of len == 2
    /// - the provided id is already taken by a container inside this term
    /// - hpos > Term width or vpos > Term height
    /// - w(idth) > Term width or h(eight) > Term height
    /// - this new container area infringes on a pre existing container's area in this Term and
    /// overlay is turned off for the Term
    pub fn container(
        &mut self,
        id: &[u8; 2],
        xpos: Pos,
        ypos: Pos,
        zpos: Pos,
        area: Area,
        border: Border,
        padding: Padding,
    ) -> Result<(), TreeError> {
        if !self.is_valid_container_id(&id) {
            eprintln!("bad id");
            return Err(TreeError::BadID);
        }

        let [wextra, hextra] = resolve_wh(&border, &padding);

        let [w, h] = area.unwrap([self.w, self.h]);
        let [w, h] = [w - wextra, h - hextra];

        let [hpos, vpos] = xpos.clone().point(ypos.clone(), [self.w, self.h]);
        let [hpos, vpos] = [
            if let Pos::End = xpos {
                hpos - w - wextra
            } else {
                hpos
            },
            if let Pos::End = ypos {
                vpos - h - hextra
            } else {
                vpos
            },
        ];

        if let Border::Manual { .. } = border {
            if !border_fit(&border, &padding, self.w, self.h) {
                return Err(TreeError::BoundsNotRespected);
            }
        }

        let cont = Container::new([id[0], id[1]], hpos, vpos, w, h, border, padding);

        if self.assign_valid_container_area(&cont).is_err() {
            return Err(TreeError::BoundsNotRespected);
        }

        self.containers.insert(cont.id, cont);

        Ok(())
    }

    pub fn container_from_builder(&mut self, builder: &mut ContainerBuilder) {
        self.containers.insert(builder.id(), builder.build());
    }

    /// pushes an existing Container to this Term's container vector
    ///
    /// # Examples
    ///
    /// ## Failure
    ///
    /// ```
    /// let mut term = Term::new(0);
    /// // wrong Term id '1' instead of '0'
    /// let cont = Container::new(&[1, 0], 3, 3, 34, 18, Border::Uniform('+'), Padding::None);
    /// let Err(res) = term.push_container(cont) else { unreachable!("you should have been an
    /// error") };
    /// assert_eq!(res.0.id, [0, 1]);
    /// ```
    ///
    /// ```
    /// let mut term = Term::new(0);
    /// // container starting x coordinate of '11111' > Term width
    /// let cont = Container::new(&[0, 0], 11111, 3, 34, 18, Border::Uniform('+'), Padding::None);
    /// let Err(res) = term.push_container(cont) else { unreachable!("you should have been an
    /// error") };
    /// assert_eq!(res.1, ComponentTreeError::BoundsNotRespected);
    /// ```
    ///
    /// ## Success
    ///
    /// ```
    /// let mut term = Term::new(0);
    /// let cont = Container::new(&[0, 0], 3, 3, 34, 18, Border::Uniform('+'), Padding::None);
    /// assert!(term.push_container(cont).is_ok());
    /// ```
    ///
    /// # Errors
    /// this method error conditions are the same as the container() method
    /// in case of an error, the Container that was passed as an argument is returned alongside the
    /// error value
    pub fn push_container(&mut self, c: Container) -> Result<(), (Container, TreeError)> {
        if self.has_container(&c.id) {
            return Err((c, TreeError::IDAlreadyExists));
        }

        // NOTE: assign_valid_thing_area series of functions need to be split to 2 fns
        // validate_thing_area and reassign_valid_thing_area
        // this fn's case only needs the validate_thing_area part

        if self.assign_valid_container_area(&c).is_err() {
            return Err((c, TreeError::BoundsNotRespected));
        }

        self.containers.insert(c.id, c);

        Ok(())
    }

    /// takes only term id and automatically assigns an id for the container
    /// returns the full new container id
    // pub fn container_auto(
    //     &mut self,
    //     id: u8,
    //     hpos: u16,
    //     vpos: u16,
    //     w: u16,
    //     h: u16,
    // ) -> Result<[u8; 2], ComponentTreeError> {
    //     /// this should actually fail
    //     if !self.has_term(id) {
    //         return Err(ComponentTreeError::ParentNotFound);
    //     }
    //
    //     let id = [id, self.assign_container_id(id)];
    //
    //     let term = self.term_mut(id[0]).unwrap();
    //
    //     if term.assign_valid_container_area(hpos, vpos, w, h).is_err() {
    //         return Err(ComponentTreeError::BoundsNotRespected);
    //     }
    //
    //     term.containers.push(Container::new(id, hpos, vpos, w, h));
    //
    //     Ok(id)
    // }

    /// pushes an existing input Text object to a child container of this Term
    pub fn push_input(&mut self, i: Text) -> Result<(), (Text, TreeError)> {
        if !self.has_container(&[i.id[0], i.id[1]]) || self.has_input(&i.id) || i.id[2] % 2 != 0 {
            return Err((i, TreeError::BadID));
        }

        self.container_mut(&[i.id[0], i.id[1]])
            .unwrap()
            .texts
            .insert(i.id, i);

        Ok(())
    }

    /// ...
    pub fn input(
        &mut self,
        id: &[u8; 3],
        xpos: Pos,
        ypos: Pos,
        zpos: Pos,
        area: Area,
        border: Border,
        padding: Padding,
    ) -> Result<(), TreeError> {
        if !self.is_valid_input_id(&id) {
            eprintln!("bad id: {:?}", id);
            return Err(TreeError::IdError(IdError::IdAlreadyTaken));
        }

        // let [hpos, vpos] = [text.hpos, text.vpos];
        // let [w, h] = text.decorate();

        let mut cont = self.container_mut(&[id[0], id[1]]).unwrap();
        let contwh = [cont.w, cont.h];

        let [wextra, hextra] = resolve_wh(&border, &padding);

        let [w, h] = area.unwrap(contwh);
        let [w, h] = [w - wextra, h - hextra];
        let [hpos, vpos] = xpos.clone().point(ypos.clone(), [w, h]);

        if cont.area_out_of_bounds(&[w, h]) {
            return Err(TreeError::SpaceError(SpaceError::AreaOutOfBounds));
        } else if cont.origin_out_of_bounds(&[w, h], &[hpos, vpos]) {
            return Err(TreeError::SpaceError(SpaceError::OriginOutOfBounds));
        }

        if let Border::Manual { .. } = border {
            if !border_fit(&border, &padding, w, h) {
                return Err(TreeError::BoundsNotRespected);
            }
        }

        let [hpos, vpos] = xpos.clone().point(ypos.clone(), contwh);
        let [hpos, vpos] = [
            if let Pos::End = xpos {
                hpos - w - wextra
            } else {
                hpos
            },
            if let Pos::End = ypos {
                vpos - h - hextra
            } else {
                vpos
            },
        ];

        let [ahpos, avpos] =
            calc_text_abs_ori(&[id[0], id[1]], &[hpos, vpos], &border, &padding, &cont);

        let input = Text::new(
            [id[0], id[1], id[2]],
            hpos,
            vpos,
            ahpos,
            avpos,
            w,
            h,
            &[],
            border,
            padding,
        );

        if cont.assign_valid_text_area(&input).is_err() {
            return Err(TreeError::BoundsNotRespected);
        }

        cont.texts.insert(input.id, input);

        Ok(())
    }

    pub fn input_from_builder(&mut self, builder: &mut InputBuilder) -> Result<(), TreeError> {
        let res = self.container_mut(&builder.cid());
        if res.is_none() {
            return Err(TreeError::BadID);
        }

        let cont = res.unwrap();
        cont.texts.insert(builder.id(), builder.build());

        Ok(())
    }

    /// takes only term and container ids and automatically assigns an id for the input
    /// returns the full new input id
    /// DONT USE FOR NOW
    // pub fn input_auto(&mut self, id: &[u8]) -> Result<[u8; 3], ComponentTreeError> {
    //     if id.len() > 2 {
    //         eprintln!("use self.input(id) instead");
    //         return Err(ComponentTreeError::BadID);
    //     }
    //
    //     if !self.has_container(&[id[0], id[1]]) {
    //         eprintln!("bad id");
    //         return Err(ComponentTreeError::ParentNotFound);
    //     }
    //
    //     let id = [id[0], id[1], self.assign_input_id(id[0], id[1])];
    //
    //     self.container_mut(&[id[0], id[1]])
    //         .unwrap()
    //         .texts
    //         .push(Text::new(id));
    //
    //     Ok(id)
    // }

    pub fn noedit(
        &mut self,
        id: &[u8; 3],
        xpos: Pos,
        ypos: Pos,
        zpos: Pos,
        area: Area,
        border: Border,
        padding: Padding,
        value: &[Option<char>],
    ) -> Result<(), TreeError> {
        if !self.is_valid_noedit_id(&id) {
            eprintln!("bad id");
            return Err(TreeError::BadID);
        }

        let mut cont = self.container_mut(&[id[0], id[1]]).unwrap();
        let contwh = [cont.w, cont.h];

        let [wextra, hextra] = resolve_wh(&border, &padding);

        let [w, h] = area.unwrap(contwh);
        let [w, h] = [w - wextra, h - hextra];

        if let Border::Manual { .. } = border {
            if !border_fit(&border, &padding, w, h) {
                return Err(TreeError::BoundsNotRespected);
            }
        }

        let [hpos, vpos] = xpos.clone().point(ypos.clone(), contwh);
        let [hpos, vpos] = [
            if let Pos::End = xpos {
                hpos - w - wextra
            } else {
                hpos
            },
            if let Pos::End = ypos {
                vpos - h - hextra
            } else {
                vpos
            },
        ];

        if value.len() as u16 > w * h {
            eprintln!(
                "value of len {} too long for bounds w * h {}",
                value.len(),
                w * h
            );
            return Err(TreeError::BadValue);
        }

        let [ahpos, avpos] =
            calc_text_abs_ori(&[id[0], id[1]], &[hpos, vpos], &border, &padding, &cont);

        let noedit = Text::new(
            [id[0], id[1], id[2]],
            hpos,
            vpos,
            ahpos,
            avpos,
            w,
            h,
            value,
            border,
            padding,
        );

        if cont.assign_valid_text_area(&noedit).is_err() {
            return Err(TreeError::BoundsNotRespected);
        }

        cont.texts.insert(noedit.id, noedit);

        Ok(())
    }

    pub fn noedit_from_builder(&mut self, builder: &mut NoEditBuilder) -> Result<(), TreeError> {
        let res = self.container_mut(&builder.cid());
        if res.is_none() {
            return Err(TreeError::BadID);
        }

        let cont = res.unwrap();
        cont.texts.insert(builder.id(), builder.build());

        Ok(())
    }

    /// pushes provided non editable Text object into a the Container with the given id if it
    /// exists and the Text object is valid, otherwise returns the error and Text object instance
    pub fn push_noedit(&mut self, ne: Text) -> Result<(), (Text, TreeError)> {
        if !self.has_container(&[ne.id[0], ne.id[1]]) || self.has_input(&ne.id) || ne.id[2] % 2 == 0
        {
            return Err((ne, TreeError::BadID));
        }

        self.container_mut(&[ne.id[0], ne.id[1]])
            .unwrap()
            .texts
            .insert(ne.id, ne);

        Ok(())
    }

    /// takes only term and container ids and automatically assigns an id for the noedit
    /// returns the full new noedit id
    // pub fn noedit_auto(&mut self, id: &[u8]) -> Result<[u8; 3], ComponentTreeError> {
    //     if id.len() > 2 {
    //         eprintln!("use self.noedit(id) instead");
    //         return Err(ComponentTreeError::BadID);
    //     }
    //
    //     if !self.has_container(&[id[0], id[1]]) {
    //         eprintln!("bad id");
    //         return Err(ComponentTreeError::ParentNotFound);
    //     }
    //
    //     let id = [id[0], id[1], self.assign_noedit_id(id[0], id[1])];
    //
    //     self.container_mut(&[id[0], id[1]])
    //         .unwrap()
    //         .texts
    //         .push(Text::new(id));
    //
    //     Ok(id)
    // }

    /// return the sum of all the input text objects inside this term
    pub fn ilen(&self) -> usize {
        self.containers
            .values()
            .map(|c| c.texts.values().filter(|t| t.id[2] % 2 == 0).count())
            .sum::<usize>()
    }

    /// return the sum of all the noeditable text objects inside this term
    pub fn nelen(&self) -> usize {
        self.containers
            .values()
            .map(|c| c.texts.values().filter(|t| t.id[2] % 2 != 0).count())
            .sum::<usize>()
    }

    /// returns whether the term has a container with the provided id
    pub fn has_container(&self, id: &[u8; 2]) -> bool {
        self.containers
            .iter()
            .map(|(_, i)| i)
            .find(|c| c.id == *id)
            .is_some()
    }

    /// returns whether any container in the term has an input with the provided id
    pub fn has_input(&self, id: &[u8; 3]) -> bool {
        match self.container_ref(&[id[0], id[1]]) {
            Some(cont) => cont
                .texts
                .iter()
                .map(|(_, i)| i)
                .find(|input| input.id[2] % 2 == 0 && input.id == *id)
                .is_some(),
            None => {
                eprintln!("no container with such id was found {:?}", &id[..2]);
                false
            }
        }
    }

    /// returns whether any container in the term has an noeditable with the provided id
    pub fn has_noedit(&self, id: &[u8; 3]) -> bool {
        match self.container_ref(&[id[0], id[1]]) {
            Some(cont) => cont
                .texts
                .iter()
                .map(|(_, i)| i)
                .find(|input| input.id[2] % 2 != 0 && input.id == *id)
                .is_some(),
            None => {
                eprintln!("no container with such id was found {:?}", &id[..2]);
                false
            }
        }
    }

    // NOTE: this method does not check the validity of the provided term id
    fn assign_container_id(&self, term: u8) -> u8 {
        let mut id = 0;
        for cont in self.containers.values() {
            if cont.id[1] == id {
                id += 1;
            } else {
                break;
            }
        }

        id
    }

    // NOTE: this method does not check the validity of the provided term and container ids
    fn assign_input_id(&self, term: u8, cont: u8) -> u8 {
        let cont = self.container_ref(&[term, cont]).unwrap();

        let mut id = 0;
        let mut iter = cont
            .texts
            .iter()
            .map(|(_, i)| i)
            .filter(|i| i.id[2] % 2 == 0);
        while let Some(item) = iter.next() {
            if item.id[2] == id {
                id += 2;
            } else {
                break;
            }
        }

        id
    }

    // NOTE: this method does not check the validity of the provided term and container ids
    fn assign_noedit_id(&self, term: u8, cont: u8) -> u8 {
        let cont = self.container_ref(&[term, cont]).unwrap();

        let mut id = 0;
        let mut iter = cont
            .texts
            .iter()
            .map(|(_, i)| i)
            .filter(|i| i.id[2] % 2 != 0);
        while let Some(item) = iter.next() {
            if item.id[2] == id {
                id += 2;
            } else {
                break;
            }
        }

        id
    }
}
