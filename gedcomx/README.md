# gedcomx
The core data structures and serialization / deserialization of the GEDCOM X format.

![CI](https://github.com/ephraimkunz/gedcomx-rs/workflows/CI/badge.svg)
[![codecov](https://codecov.io/gh/ephraimkunz/gedcomx-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/ephraimkunz/gedcomx-rs)
[![](https://img.shields.io/crates/v/gedcomx.svg)](https://crates.io/crates/gedcomx)
[![API](https://docs.rs/gedcomx/badge.svg)](https://docs.rs/gedcomx)

## Specification Compliance
This crate provides conformance to the following GEDCOM X specs:
- [x] [GEDCOM X XML 1.0](https://github.com/FamilySearch/gedcomx/blob/master/specifications/xml-format-specification.md) fully implemented using [Yaserde](https://github.com/media-io/yaserde) for XML serialization and deserialization.
- [x] [GEDCOM X JSON 1.0](https://github.com/FamilySearch/gedcomx/blob/master/specifications/json-format-specification.md) fully implemented using [Serde](https://github.com/serde-rs/serde) for JSON serialization and deserialization.
- [x] [GEDCOM X Event Types 1.0](https://github.com/FamilySearch/gedcomx/blob/master/specifications/event-types-specification.md) fully implemented.
- [x] [GEDCOM X Fact Types 1.0](https://github.com/FamilySearch/gedcomx/blob/master/specifications/fact-types-specification.md) fully implemented. 
- [x] [GEDCOM X Name Part Qualifiers 1.0](https://github.com/FamilySearch/gedcomx/blob/master/specifications/name-part-qualifiers-specification.md) fully implemented.
- [x] [GEDCOM X Relationship Types 1.0](https://github.com/FamilySearch/gedcomx/blob/master/specifications/relationship-types-specification.md) fully implemented.
- [x] [GEDCOM X Date 1.0](https://github.com/FamilySearch/gedcomx/blob/master/specifications/date-format-specification.md) compliant via the [gedcomx_date](https://github.com/nicompte/gedcomx-date-rs) crate.
- [ ] [GEDCOM X Field Types 1.0](https://github.com/FamilySearch/gedcomx-record/blob/master/specifications/field-types-specification.md)
- [ ] [GEDCOM X Record Extensions 1.0](https://github.com/FamilySearch/gedcomx-record/blob/master/specifications/record-specification.md)
- [ ] [FamilySearch GEDCOM X Extensions](https://github.com/FamilySearch/gedcomx-familysearch-extensions/blob/master/specifications/gedcomx-familysearch-specification.md)
- [ ] [GEDCOM X Atom Extensions 1.0](https://github.com/FamilySearch/gedcomx-rs/blob/master/specifications/atom-model-specification.md)
- [ ] [GEDCOM X RS 1.0](https://github.com/FamilySearch/gedcomx-rs/blob/master/specifications/rs-specification.md)

## Features
- Well tested: hundreds of unit tests and some large integration tests. Integration tests parsing of all the recipes in the [Recipe Book](http://www.gedcomx.org/Recipe-Book.html) as well as other test data from the [Java Gedcomx implementation](https://github.com/FamilySearch/gedcomx-java).
- [Fuzzed](https://github.com/rust-fuzz/cargo-fuzz) and [quickchecked](https://github.com/BurntSushi/quickcheck).
- Use the builder pattern to safely build GEDCOM X data models.
- XML and JSON serialization and deserialization supported.

## Documentation
https://docs.rs/gedcomx

## Usage
Add this to your Cargo.toml:

```toml
[dependencies]
gedcomx = "0.1"
```

## Example
A GEDCOM X document can be deserialized from JSON:

```rust
use gedcomx::Gedcomx;

fn main() {
    let json = std::fs::read_to_string("../data/birth.json").unwrap();
    let gx = Gedcomx::from_json_str(&json).unwrap();
    println!(
        "Successfully deserialized GEDCOM X document from JSON with {} people inside!",
        gx.persons.len()
    );

    assert_eq!(gx.persons.len(), 4);
}

```

Similarly, you can deserialize from XML with the Gedcomx struct's `from_xml_str` method.

In-memory GEDCOM X documents can be built by instantiating individual components and adding them to an instance of `Gedcomx`.
This can then be serialized to JSON or XML using a family of functions defined on `Gedcomx`:

```rust
use gedcomx::{Gedcomx, Name, NameForm, NameType, Person};

let gx = Gedcomx::builder()
    .person(
        Person::builder()
            .private(true)
            .name(
                Name::builder(
                    NameForm::builder()
                        .full_text("Jim Halpert")
                        .lang("en")
                        .build(),
                )
                .name_type(NameType::BirthName)
                .build(),
            )
            .build(),
    )
    .build();

let json = gx.to_json_string_pretty().unwrap();

assert_eq!(json.len(), 285);
```

## Contributing
See the [Design Doc](DESIGN.md) for more information about why various choices were made. PRs welcome!
