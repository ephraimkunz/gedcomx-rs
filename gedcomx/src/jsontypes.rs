use crate::toplevel::Uri;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ResourceReference {
    pub resource: String,
}

impl<T> From<Uri<T>> for ResourceReference {
    fn from(uri: Uri<T>) -> Self {
        match uri {
            Uri::Id(id) => Self { resource: id },
            Uri::Some(s) => Self {
                resource: String::new(), // TODO: Should use T?
            },
        }
    }
}
