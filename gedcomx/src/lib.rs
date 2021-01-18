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
