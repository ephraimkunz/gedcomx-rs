# gedcomx
The core data structures and serialization / deserialization of the GEDCOM X format.

![CI](https://github.com/ephraimkunz/gedcomx-rs/workflows/CI/badge.svg)
[![codecov](https://codecov.io/gh/ephraimkunz/gedcomx-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/ephraimkunz/gedcomx-rs)

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
