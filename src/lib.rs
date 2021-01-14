#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
// #![deny(missing_docs)]
#![allow(clippy::pub_enum_variant_names)]

mod components;
pub use components::*;

mod toplevel;
pub use toplevel::*;
