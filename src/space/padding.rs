/// Container and Text objects padding space
#[derive(Debug, Default, Clone, Copy)]
pub enum Padding {
    /// no padding
    #[default]
    None,

    /// padding only between the value inside the object and its border
    Inner {
        /// top side padding
        top: u16,
        /// bottom side padding
        bottom: u16,
        /// right side padding
        right: u16,
        /// left side padding
        left: u16,
    },

    /// padding only around the border of the object
    Outer {
        /// top side padding
        top: u16,
        /// bottom side padding
        bottom: u16,
        /// right side padding
        right: u16,
        /// left side padding
        left: u16,
    },

    /// padding both around the border and between the border and value of the object
    InOut {
        /// inner top side padding
        inner_top: u16,
        /// inner bottom side padding
        inner_bottom: u16,
        /// inner right side padding
        inner_right: u16,
        /// inner lef tside padding
        inner_left: u16,
        /// outer top side padding
        outer_top: u16,
        /// outer bottom side padding
        outer_bottom: u16,
        /// outer right side padding
        outer_right: u16,
        /// outer left side padding
        outer_left: u16,
    },
}

// TODO: need component methods
// width / width + inner padding / width + outer padding / w ip op b /
//
pub trait ComponentCoordinates {
    fn w(&self) -> u16;
    fn w_ip(&self) -> u16;
    fn w_op(&self) -> u16;
    fn w_b_ip(&self) -> u16;
    fn w_b_p(&self) -> u16;

    fn h(&self) -> u16;
    fn h_ip(&self) -> u16;
    fn h_ip_b(&self) -> u16;
    fn h_op(&self) -> u16;
    fn h_b_p(&self) -> u16;

    fn x0(&self);
    fn ax0(&self);
    fn x1(&self);
    fn ax1(&self);

    fn y0(&self);
    fn ay0(&self);
    fn y1(&self);
    fn ay1(&self);
}

impl From<()> for Padding {
    fn from(value: ()) -> Self {
        Self::None
    }
}

impl<T> From<[T; 8]> for Padding
where
    T: PartialOrd,
{
    fn from(value: [T; 8]) -> Self {
        Self::InOut {
            inner_top: value[0] as u16,
            inner_bottom: value[1] as u16,
            inner_right: value[2] as u16,
            inner_left: value[3] as u16,
            outer_top: value[4] as u16,
            outer_bottom: value[5] as u16,
            outer_right: value[6] as u16,
            outer_left: value[7] as u16,
        }
    }
}

impl<T> From<(T, T, T, T, T, T, T, T)> for Padding
where
    T: PartialOrd,
{
    fn from(value: (T, T, T, T, T, T, T, T)) -> Self {
        Self::InOut {
            inner_top: value.0 as u16,
            inner_bottom: value.1 as u16,
            inner_right: value.2 as u16,
            inner_left: value.3 as u16,
            outer_top: value.4 as u16,
            outer_bottom: value.5 as u16,
            outer_right: value.6 as u16,
            outer_left: value.7 as u16,
        }
    }
}

impl<T> From<(T, T, T, T)> for Padding
where
    T: PartialOrd,
{
    fn from(value: (T, T, T, T)) -> Self {
        Self::Inner {
            top: value.0 as u16,
            bottom: value.1 as u16,
            right: value.2 as u16,
            left: value.3 as u16,
        }
    }
}

impl<T> From<[T; 4]> for Padding
where
    T: PartialOrd,
{
    fn from(value: [T; 4]) -> Self {
        Self::Outer {
            top: value[0] as u16,
            bottom: value[1] as u16,
            right: value[2] as u16,
            left: value[3] as u16,
        }
    }
}

use std::mem::discriminant;

impl From<&Padding> for [u16; 4] {
    fn from(value: &Padding) -> Self {
        if let Padding::Inner {
            ref top,
            ref bottom,
            ref right,
            ref left,
        } = value
        {
            return [top, right, bottom.left];
        } else if let Padding::Outer {
            ref top,
            ref bottom,
            ref right,
            ref left,
        } = value
        {
            return [top, right, bottom.left];
        }

        [0; 4]
    }
}

impl From<&Padding> for [u16; 8] {
    fn from(value: &Padding) -> Self {
        if let Padding::InOut {
            ref inner_top,
            ref inner_bottom,
            ref inner_right,
            ref inner_left,
            ref outer_top,
            ref outer_bottom,
            ref outer_right,
            ref outer_left,
        } = value
        {
            return [
                inner_top,
                inner_right,
                inner_bottom,
                inner_left,
                outer_top,
                outer_right,
                outer_bottom,
                outer_left,
            ];
        }

        [0; 8]
    }
}

fn is_null(value: impl for<'a> From<&'a Padding>) -> bool {
    value.iter().all(|e| e == 0)
}

impl Padding {
    pub fn new(value: impl Into<Padding>) -> Self {
        value.into()
    }

    /// creates a new Padding with the None variant
    pub fn none() -> Self {
        Padding::None
    }

    /// creates a new Padding with the Inner variant
    pub fn inner(top: u16, bottom: u16, right: u16, left: u16) -> Self {
        Self::Inner {
            top,
            bottom,
            right,
            left,
        }
    }

    /// creates a new Padding with the Outer variant
    pub fn outer(top: u16, bottom: u16, right: u16, left: u16) -> Self {
        Self::Inner {
            top,
            bottom,
            right,
            left,
        }
    }

    // FIXME: this should error
    fn spread(&self, spread_padding: &mut impl From<Padding>) {
        *spread_padding = self.into();
    }

    pub fn in_out(
        inner_top: u16,
        inner_bottom: u16,
        inner_right: u16,
        inner_left: u16,
        outer_top: u16,
        outer_bottom: u16,
        outer_right: u16,
        outer_left: u16,
    ) -> Self {
        Self::InOut {
            inner_top,
            inner_bottom,
            inner_right,
            inner_left,
            outer_top,
            outer_bottom,
            outer_right,
            outer_left,
        }
    }

    /// mutates the top padding value and returns the padding enum value
    /// or returns self value in case variant is neither Inner nor Outer
    pub fn top(self, top: u16) -> Self {
        if let Self::Inner {
            bottom,
            right,
            left,
            ..
        } = self
        {
            return Self::Inner {
                bottom,
                right,
                left,
                top,
            };
        } else if let Self::Outer {
            bottom,
            right,
            left,
            ..
        } = self
        {
            return Self::Outer {
                bottom,
                right,
                left,
                top,
            };
        }

        self
    }

    /// mutates the buttom padding value and returns the padding enum value
    /// or returns self value in case variant is neither Inner nor Outer
    pub fn bottom(self, bottom: u16) -> Self {
        if let Self::Inner {
            top, right, left, ..
        } = self
        {
            return Self::Inner {
                bottom,
                right,
                left,
                top,
            };
        } else if let Self::Outer {
            top, right, left, ..
        } = self
        {
            return Self::Outer {
                bottom,
                right,
                left,
                top,
            };
        }

        self
    }

    /// mutates the right padding value and returns the padding enum value
    /// or returns self value in case variant is neither Inner nor Outer
    pub fn right(self, right: u16) -> Self {
        if let Self::Inner {
            top, bottom, left, ..
        } = self
        {
            return Self::Inner {
                bottom,
                right,
                left,
                top,
            };
        } else if let Self::Outer {
            top, bottom, left, ..
        } = self
        {
            return Self::Outer {
                bottom,
                right,
                left,
                top,
            };
        }

        self
    }

    /// mutates the left padding value and returns the padding enum value
    /// or returns self value in case variant is neither Inner nor Outer
    pub fn left(self, left: u16) -> Self {
        if let Self::Inner {
            top, bottom, right, ..
        } = self
        {
            return Self::Inner {
                bottom,
                right,
                left,
                top,
            };
        } else if let Self::Outer {
            top, bottom, right, ..
        } = self
        {
            return Self::Outer {
                bottom,
                right,
                left,
                top,
            };
        }

        self
    }

    /// mutates the inner_top padding value and returns the padding enum value
    /// or returns self value in case variant is not InOut
    pub fn inner_top(self, inner_top: u16) -> Self {
        if let Self::InOut {
            inner_bottom,
            inner_left,
            inner_right,
            outer_top,
            outer_bottom,
            outer_right,
            outer_left,
            ..
        } = self
        {
            return Self::InOut {
                inner_top,
                inner_bottom,
                inner_left,
                inner_right,
                outer_top,
                outer_bottom,
                outer_right,
                outer_left,
            };
        }

        self
    }

    /// mutates the inner_bottom padding value and returns the padding enum value
    /// or returns self value in case variant is not InOut
    pub fn inner_bottom(self, inner_bottom: u16) -> Self {
        if let Self::InOut {
            inner_top,
            inner_left,
            inner_right,
            outer_top,
            outer_bottom,
            outer_right,
            outer_left,
            ..
        } = self
        {
            return Self::InOut {
                inner_top,
                inner_bottom,
                inner_left,
                inner_right,
                outer_top,
                outer_bottom,
                outer_right,
                outer_left,
            };
        }

        self
    }

    /// mutates the inner_right padding value and returns the padding enum value
    /// or returns self value in case variant is not InOut
    pub fn inner_right(self, inner_right: u16) -> Self {
        if let Self::InOut {
            inner_top,
            inner_left,
            inner_bottom,
            outer_top,
            outer_bottom,
            outer_right,
            outer_left,
            ..
        } = self
        {
            return Self::InOut {
                inner_top,
                inner_bottom,
                inner_left,
                inner_right,
                outer_top,
                outer_bottom,
                outer_right,
                outer_left,
            };
        }

        self
    }

    /// mutates the inner_left padding value and returns the padding enum value
    /// or returns self value in case variant is not InOut
    pub fn inner_left(self, inner_left: u16) -> Self {
        if let Self::InOut {
            inner_top,
            inner_right,
            inner_bottom,
            outer_top,
            outer_bottom,
            outer_right,
            outer_left,
            ..
        } = self
        {
            return Self::InOut {
                inner_top,
                inner_bottom,
                inner_left,
                inner_right,
                outer_top,
                outer_bottom,
                outer_right,
                outer_left,
            };
        }

        self
    }
}
