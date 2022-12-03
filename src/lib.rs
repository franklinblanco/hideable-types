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
/* 
"primerobjeto": {
    "vectordeobjetos": {
        "1": {},
        '2': {},
        "3": {},
    }
}
*/

/// 
impl<T: Hideable> Hideable for Vec<T> {
    fn hide_fields(&self, attributes: Vec<String>) -> Result<serde_json::Map<String, serde_json::Value>, String> {
        let mut values: Vec<Value> = Vec::new();
        for element in self {
            let key_value = match element.hide_fields(attributes.clone()) {
                Ok(kv) => kv,
                Err(e) => return Err(e),
            };
            values.push(key_value.into());
        }
        let mut map = Map::new();
        map.insert("k".into(), values.into());
        Ok(map)
    }
}

