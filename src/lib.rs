use std::fmt::Display;

use serde_json::{Map, Value};

#[derive(Debug)]
pub enum FieldAttribute {
    User,
    Employee,
}
impl Display for FieldAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldAttribute::User => write!(f, "User"),
            FieldAttribute::Employee => write!(f, "Employee"),
        }
    }
}

pub struct Field<V> {
    /// Field name
    pub key: String,
    /// Field value, doesn't matter type
    pub value: V,
    /// Attributes that are applied to field
    pub attributes: Vec<FieldAttribute>
}

pub trait Hideable {
    /// Skip fields that contain any of the specified attributes.
    fn hide_fields(&self, attributes: Vec<String>) -> Result<Map<String, Value>, String>;
    // /// Skip all fields except those that contain the specified attributes
    //fn hide_all_except_fields(attributes: Vec<String>);
}
