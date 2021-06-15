# gedcomx-rs
A collection of Rust crates for working with GEDCOM X.

![CI](https://github.com/ephraimkunz/gedcomx-rs/workflows/CI/badge.svg)
[![codecov](https://codecov.io/gh/ephraimkunz/gedcomx-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/ephraimkunz/gedcomx-rs)

## Crates
* [gedcomx](./gedcomx) - Core data structures and serialization / deserialization in XML and JSON.
* [gedcomx_file](./gedcomx_file) Implementation of the GEDCOM X File Format specification. Bundles up genealogical data and resources into a file and defines how the resources within the file can link to each other.

## Features
- Fully documented:
- Well tested: hundreds of unit tests and some large integration tests. Integration tests test parsing of all the recipes in the [Recipe Book](http://www.gedcomx.org/Recipe-Book.html) as well as other test data from the [Java Gedcomx implementation](https://github.com/FamilySearch/gedcomx-java).
- Use the builder pattern to safely build Gedcomx data models.
- Use with stable Rust.
