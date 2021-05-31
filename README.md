# gedcomx-rs
A collection of Rust crates for working with GEDCOM X.

![CI](https://github.com/ephraimkunz/gedcomx-rs/workflows/CI/badge.svg)
[![codecov](https://codecov.io/gh/ephraimkunz/gedcomx-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/ephraimkunz/gedcomx-rs)

## Crates
* [gedcomx](./gedcomx) - Core data structures and serialization / deserialization in XML and JSON.
* File format crate with spec compliance.

## Specification Compliance

- [ ] [GEDCOM X File Format 1.0](https://github.com/FamilySearch/gedcomx/blob/master/specifications/file-format-specification.md)
- [ ] [GEDCOM X Standard Header Set 1.0](https://github.com/FamilySearch/gedcomx/blob/master/specifications/standard-header-set-specification.md)

## Features
- Fully documented:
- Well tested: hundreds of unit tests and some large integration tests. Integration tests test parsing of all the recipes in the [Recipe Book](http://www.gedcomx.org/Recipe-Book.html) as well as other test data from the [Java Gedcomx implementation](https://github.com/FamilySearch/gedcomx-java).
- Use the builder pattern to safely build Gedcomx data models.
- Use with stable Rust.
