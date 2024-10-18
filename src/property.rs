use std::any::Any;
use std::collections::HashMap;
use std::ops::Range;

pub type Properties = HashMap<String, Property>;

trait PMap {
    fn grab_ref(&self, k: &str) -> Option<&Property>;

    fn grab_mut(&mut self, k: &str) -> Option<&mut Property>;

    fn assign(&mut self, k: &str, v: impl IsProperty) -> Option<Property>;

    fn discard(&mut self, k: &str) -> Option<Property>;

    fn contains(&self, k: &str) -> bool;
}

impl PMap for Properties {
    fn grab_ref(&self, k: &str) -> Option<&Property> {
        self.get(k)
    }

    fn grab_mut(&mut self, k: &str) -> Option<&mut Property> {
        self.get_mut(k)
    }

    fn assign(&mut self, k: &str, v: impl IsProperty) -> Option<Property> {
        self.insert(k.to_string(), Property::new(v))
    }

    fn discard(&mut self, k: &str) -> Option<Property> {
        self.remove(k)
    }

    fn contains(&self, k: &str) -> bool {
        self.contains_key(k)
    }
}

#[derive(Debug, PartialEq)]
enum PropertyError {
    NoSuchVariant,
}

pub(crate) enum Property {
    String(String),
    Fn(Box<dyn FnMut(Property) -> Property>),
    Range(std::ops::Range<u64>),
    Char(char),
    Int(i64),
    UInt(u64),
    Float(f64),
    Bool(bool),
    Vec(Vec<Property>),
    Map(HashMap<String, Property>),
    None,
    Err(PropertyError),
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
                    Self::String(s) => format!("{:?}", s),
                    Self::Fn(f) => format!("{:?}", std::any::type_name_of_val(f)),
                    Self::Range(r) => format!("{:?}", r),
                    Self::Char(c) => format!("{:?}", c),
                    Self::Int(i) => format!("{:?}", i),
                    Self::UInt(u) => format!("{:?}", u),
                    Self::Float(f) => format!("{:?}", f),
                    Self::Bool(b) => format!("{:?}", b),
                    Self::Vec(v) => format!("{:?}", v),
                    Self::Map(m) => format!("{:?}", m),
                    Self::None => format!("None",),
                    Self::Err(e) => format!("{:?}", e),
                }
            )
        )
    }
}

trait IsProperty {
    fn to_any(self) -> Box<dyn Any>;
    fn type_id(&self) -> char;
}

impl IsProperty for String {
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }

    fn type_id(&self) -> char {
        'S'
    }
}

impl<T> IsProperty for T
where
    T: FnMut(Property) -> Property + 'static,
{
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }

    fn type_id(&self) -> char {
        'F'
    }
}

impl IsProperty for Range<u64> {
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }

    fn type_id(&self) -> char {
        'r'
    }
}

impl IsProperty for char {
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }

    fn type_id(&self) -> char {
        'c'
    }
}

impl IsProperty for i64 {
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }

    fn type_id(&self) -> char {
        'i'
    }
}

impl IsProperty for u64 {
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }

    fn type_id(&self) -> char {
        'u'
    }
}

impl IsProperty for f64 {
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }

    fn type_id(&self) -> char {
        'f'
    }
}

impl IsProperty for bool {
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }

    fn type_id(&self) -> char {
        'b'
    }
}

impl IsProperty for Vec<Property> {
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }

    fn type_id(&self) -> char {
        'V'
    }
}

impl IsProperty for Properties {
    fn to_any(self) -> Box<dyn Any> {
        Box::new(self)
    }

    fn type_id(&self) -> char {
        'M'
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
            (Self::String(s), Self::String(o)) => s == o,
            (Self::Range(s), Self::Range(o)) => s == o,
            (Self::Char(s), Self::Char(o)) => s == o,
            (Self::Int(s), Self::Int(o)) => s == o,
            (Self::UInt(s), Self::UInt(o)) => s == o,
            (Self::Float(s), Self::Float(o)) => s == o,
            (Self::Bool(s), Self::Bool(o)) => s == o,
            (Self::Vec(s), Self::Vec(o)) => s == o,
            (Self::Map(s), Self::Map(o)) => s == o,
            (Self::Err(s), Self::Err(o)) => s == o,
            (Self::None, Self::None) => true,
            _ => false,
        }
    }
}

impl Property {
    fn is_none(&self) -> bool {
        self == &Self::None
    }

    fn is_err(&self) -> bool {
        discriminant(self) == discriminant(&Self::Err(PropertyError::NoSuchVariant))
    }

    fn is_err_get(&self) -> Option<&PropertyError> {
        if let Self::Err(e) = self {
            Some(e)
        } else {
            None
        }
    }

    fn is_fn(&self) -> bool {
        discriminant(self) == discriminant(&Self::Fn(Box::new(|a: Self| Self::None)))
    }

    fn is_range(&self) -> bool {
        discriminant(self) == discriminant(&Self::Range(Range { start: 0, end: 1 }))
    }

    fn is_char(&self) -> bool {
        discriminant(self) == discriminant(&Self::Char(' '))
    }

    fn is_int(&self) -> bool {
        discriminant(self) == discriminant(&Self::Int(0i64))
    }

    fn is_uint(&self) -> bool {
        discriminant(self) == discriminant(&Self::UInt(0u64))
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
        let Some(Property::String(s)) = self else {
            panic!()
        };

        s
    }

    fn range(self) -> Range<u64> {
        let Some(Property::Range(r)) = self else {
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
        let Some(Property::String(ref s)) = self else {
            panic!()
        };

        s
    }

    fn range_ref(&self) -> &Range<u64> {
        let Some(Property::Range(ref r)) = self else {
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
        let Some(Property::String(ref mut s)) = self else {
            panic!()
        };

        s
    }

    fn range_mut(&mut self) -> &mut Range<u64> {
        let Some(Property::Range(ref mut r)) = self else {
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
        let Some(Property::UInt(u)) = self else {
            panic!()
        };

        *u
    }

    fn int(&self) -> i64 {
        let Some(Property::Int(i)) = self else {
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
        let Some(Property::UInt(u)) = self else {
            panic!("not a u64")
        };

        *u
    }

    fn int(&self) -> i64 {
        let Some(Property::Int(i)) = self else {
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
    fn new(v: impl IsProperty) -> Self {
        let type_id = v.type_id();
        let v = v.to_any();
        match type_id {
            'S' => Self::String(*v.downcast::<String>().unwrap()),
            'F' => Self::Fn(
                *v.downcast::<Box<dyn FnMut(Property) -> Property>>()
                    .unwrap(),
            ),
            'r' => Self::Range(*v.downcast::<Range<u64>>().unwrap()),
            'c' => Self::Char(*v.downcast::<char>().unwrap()),
            'i' => Self::Int(*v.downcast::<i64>().unwrap()),
            'u' => Self::UInt(*v.downcast::<u64>().unwrap()),
            'f' => Self::Float(*v.downcast::<f64>().unwrap()),
            'b' => Self::Bool(*v.downcast::<bool>().unwrap()),
            'V' => Self::Vec(*v.downcast::<Vec<Property>>().unwrap()),
            'M' => Self::Map(*v.downcast::<Properties>().unwrap()),
            _ => Self::Err(PropertyError::NoSuchVariant),
        }
    }
}
