#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
// #![deny(clippy::cargo)]
// #![deny(missing_docs)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::new_without_default)]
#![allow(clippy::pub_enum_variant_names)]
#![allow(clippy::clippy::must_use_candidate)]
#![allow(clippy::missing_const_for_fn)]

mod components;
pub use components::*;

mod toplevel;
pub use toplevel::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GedcomxError {
    #[error("Can't get a non-None id for `{0}`")]
    NoId(String),

    #[error("Wrong type. Expected: {0}, Actual: {1}")]
    WrongType(String, String),
}

pub type Result<T> = std::result::Result<T, GedcomxError>;
