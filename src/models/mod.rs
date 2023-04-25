use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ServiceField {
    pub name: String,
    pub field_type: String,
    pub is_optional: bool,
}

impl ServiceField {
    pub fn new(name: String, field_type: String) -> Self {
        Self {
            name,
            field_type,
            is_optional: false,
        }
    }
    pub fn new_opt(name: String, field_type: String) -> Self {
        Self {
            name,
            field_type,
            is_optional: true,
        }
    }

    pub fn get_plain_type(&self) -> String {
        self.field_type.clone()
    }

    pub fn get_type(&self) -> String {
        if self.is_optional {
            format!("Option<{}>", self.field_type)
        } else {
            self.field_type.clone()
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Entity {
    pub name: String,
    pub fields: Vec<ServiceField>,
}