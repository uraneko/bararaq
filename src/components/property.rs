use std::any::Any;
use std::collections::HashMap;
use std::ops::Range;

pub type Properties = HashMap<String, Property>;

#[derive(Debug, PartialEq)]
enum PropertyError {
    NoSuchVariant,
}

pub trait PropertyMap {
    fn put(&mut self, key: String, value: impl Into<Property>) -> Option<Property> {
        self.insert(key, value.into())
    }
}

pub enum Property {
    PStr(String),
    PFn(Box<dyn FnMut(Property) -> Property>),
    PRng(std::ops::Range<u64>),
    PChr(char),
    PInt(i64),
    PUInt(u64),
    PFloat(f64),
    PBool(bool),
    PVec(Vec<Property>),
    PMap(HashMap<String, Property>),
}

// TODO: phase out my type_id method in favor of std::any::{Any, TypeId}

impl std::fmt::Debug for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            format!(
                "{}",
                match self {
                    Self::PStr(s) => format!("{:?}", s),
                    Self::Fn(f) => format!("{:?}", std::any::type_name_of_val(f)),
                    Self::PRng(r) => format!("{:?}", r),
                    Self::Char(c) => format!("{:?}", c),
                    Self::PInt(i) => format!("{:?}", i),
                    Self::PUInt(u) => format!("{:?}", u),
                    Self::Float(f) => format!("{:?}", f),
                    Self::Bool(b) => format!("{:?}", b),
                    Self::Vec(v) => format!("{:?}", v),
                    Self::Map(m) => format!("{:?}", m),
                }
            )
        )
    }
}

impl From<&str> for Property {
    fn from(value: &str) -> Self {
        Self::PStr(value.to_string())
    }
}

impl From<String> for Property {
    fn from(value: String) -> Self {
        Self::PStr(value)
    }
}

impl<T> From<[T; 2]> for Property
where
    T: PartialOrd,
{
    fn from(value: [T; 2]) -> Self {
        Self::PRng(Range {
            start: value[0] as u64,
            end: value[1] as u64,
        })
    }
}

impl<T> From<(T, T)> for Property
where
    T: PartialOrd,
{
    fn from(value: (T, T)) -> Self {
        Self::PRng(Range {
            start: value.0 as u64,
            end: value.1 as u64,
        })
    }
}

impl From<char> for Property {
    fn from(value: char) -> Self {
        Property::Char(value)
    }
}

impl From<i64> for Property {
    fn from(value: i64) -> Self {
        Self::PInt(value)
    }
}

impl From<u64> for Property {
    fn from(value: u64) -> Self {
        Self::PUInt(value)
    }
}

impl From<f64> for Property {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<bool> for Property {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<Vec<Property>> for Property {
    fn from(value: Vec<Property>) -> Self {
        Self::Vec(value)
    }
}

impl From<Properties> for Property {
    fn from(value: Properties) -> Self {
        Self::Map(value)
    }
}

impl<T> From<T> for Property
where
    T: FnMut(Property) -> Property + 'static,
{
    fn from(value: T) -> Self {
        Self::Fn(Box::new(value))
    }
}

use std::mem::discriminant;

impl std::cmp::PartialEq for Property {
    fn eq(&self, other: &Self) -> bool {
        if let Self::Fn(_) = self {
            eprintln!("can't compare functions");
            return false;
        }

        match (self, other) {
            (Self::PStr(s), Self::PStr(o)) => s == o,
            (Self::PRng(s), Self::PRng(o)) => s == o,
            (Self::Char(s), Self::Char(o)) => s == o,
            (Self::PInt(s), Self::PInt(o)) => s == o,
            (Self::PUInt(s), Self::PUInt(o)) => s == o,
            (Self::Float(s), Self::Float(o)) => s == o,
            (Self::Bool(s), Self::Bool(o)) => s == o,
            (Self::Vec(s), Self::Vec(o)) => s == o,
            (Self::Map(s), Self::Map(o)) => s == o,
            _ => false,
        }
    }
}

impl Property {
    fn is_fn(&self) -> bool {
        discriminant(self) == discriminant(&Self::Fn(Box::new(|a: Self| Self::Char('a'))))
    }

    fn is_range(&self) -> bool {
        discriminant(self) == discriminant(&Self::PRng(Range { start: 0, end: 1 }))
    }

    fn is_char(&self) -> bool {
        discriminant(self) == discriminant(&Self::Char(' '))
    }

    fn is_int(&self) -> bool {
        discriminant(self) == discriminant(&Self::PInt(0i64))
    }

    fn is_uint(&self) -> bool {
        discriminant(self) == discriminant(&Self::PUInt(0u64))
    }

    fn is_float(&self) -> bool {
        discriminant(self) == discriminant(&Self::Float(0.0))
    }

    fn is_bool(&self) -> bool {
        discriminant(self) == discriminant(&Self::Bool(true))
    }

    fn is_vec(&self) -> bool {
        discriminant(self) == discriminant(&Self::Vec(vec![]))
    }

    fn is_map(&self) -> bool {
        discriminant(self) == discriminant(&Self::Map(HashMap::new()))
    }
}

trait UnwrapVal {
    fn str(self) -> String;

    fn range(self) -> Range<u64>;

    fn vec(self) -> Vec<Property>;

    fn map(self) -> Properties;
}

impl UnwrapVal for Option<Property> {
    fn str(self) -> String {
        let Some(Property::PStr(s)) = self else {
            panic!()
        };

        s
    }

    fn range(self) -> Range<u64> {
        let Some(Property::PRng(r)) = self else {
            panic!()
        };

        r
    }

    fn vec(self) -> Vec<Property> {
        let Some(Property::Vec(vec)) = self else {
            panic!()
        };

        vec
    }

    fn map(self) -> Properties {
        let Some(Property::Map(map)) = self else {
            panic!()
        };

        map
    }
}

trait UnwrapRef {
    fn str_ref(&self) -> &str;

    fn range_ref(&self) -> &Range<u64>;

    fn vec_ref(&self) -> &Vec<Property>;

    fn map_ref(&self) -> &Properties;
}

impl UnwrapRef for Option<&Property> {
    fn str_ref(&self) -> &str {
        let Some(Property::PStr(ref s)) = self else {
            panic!()
        };

        s
    }

    fn range_ref(&self) -> &Range<u64> {
        let Some(Property::PRng(ref r)) = self else {
            panic!()
        };

        r
    }

    fn vec_ref(&self) -> &Vec<Property> {
        let Some(Property::Vec(ref vec)) = self else {
            panic!()
        };

        vec
    }
    fn map_ref(&self) -> &Properties {
        let Some(Property::Map(ref map)) = self else {
            panic!()
        };

        map
    }
}

trait UnwrapMut {
    fn str_mut(&mut self) -> &mut String;

    fn range_mut(&mut self) -> &mut Range<u64>;

    fn vec_mut(&mut self) -> &mut Vec<Property>;

    fn map_mut(&mut self) -> &mut Properties;
}

impl UnwrapMut for Option<&mut Property> {
    fn str_mut(&mut self) -> &mut String {
        let Some(Property::PStr(ref mut s)) = self else {
            panic!()
        };

        s
    }

    fn range_mut(&mut self) -> &mut Range<u64> {
        let Some(Property::PRng(ref mut r)) = self else {
            panic!()
        };

        r
    }

    fn vec_mut(&mut self) -> &mut Vec<Property> {
        let Some(Property::Vec(ref mut vec)) = self else {
            panic!()
        };

        vec
    }

    fn map_mut(&mut self) -> &mut Properties {
        let Some(Property::Map(ref mut map)) = self else {
            panic!()
        };

        map
    }
}

trait UnwrapFunc {
    fn func(&self) -> &dyn FnMut(Property) -> Property;
}

impl UnwrapFunc for Option<&Property> {
    fn func(&self) -> &dyn FnMut(Property) -> Property {
        let Some(Property::Fn(f)) = self else {
            panic!()
        };

        f
    }
}

trait UnwrapPrimitive {
    fn char(&self) -> char;

    fn uint(&self) -> u64;

    fn int(&self) -> i64;

    fn float(&self) -> f64;

    fn bool(&self) -> bool;
}

impl UnwrapPrimitive for Option<&Property> {
    fn char(&self) -> char {
        let Some(Property::Char(c)) = self else {
            panic!()
        };

        *c
    }

    fn uint(&self) -> u64 {
        let Some(Property::PUInt(u)) = self else {
            panic!()
        };

        *u
    }

    fn int(&self) -> i64 {
        let Some(Property::PInt(i)) = self else {
            panic!()
        };

        *i
    }

    fn float(&self) -> f64 {
        let Some(Property::Float(f)) = self else {
            panic!()
        };

        *f
    }

    fn bool(&self) -> bool {
        let Some(Property::Bool(b)) = self else {
            panic!()
        };

        *b
    }
}

impl UnwrapPrimitive for Option<Property> {
    fn char(&self) -> char {
        let Some(Property::Char(c)) = self else {
            panic!("not a char")
        };

        *c
    }

    fn uint(&self) -> u64 {
        let Some(Property::PUInt(u)) = self else {
            panic!("not a u64")
        };

        *u
    }

    fn int(&self) -> i64 {
        let Some(Property::PInt(i)) = self else {
            panic!("not an i64")
        };

        *i
    }

    fn float(&self) -> f64 {
        let Some(Property::Float(f)) = self else {
            panic!("not a float")
        };

        *f
    }

    fn bool(&self) -> bool {
        let Some(Property::Bool(b)) = self else {
            panic!("not a bool")
        };

        *b
    }
}

impl Property {
    fn new(value: impl Into<Property>) -> Self {
        value.into()
    }
}
