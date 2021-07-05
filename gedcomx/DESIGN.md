# Design Decisions
This document will catalog important design decisions I made in this library.

## Inheritance
The GEDCOM X specifications use inheritance to model various entities, with Conclusion and 
Source being the base types that many other types inherit from. Rust doesn't support inheritance,
so I had to decide how to model this. Originally I had a ConclusionData and SourceData type that each
"subclass" owned that provided the base fields. This proved unworkable to due to the XML library Yaserde (see [XML support](#xml-support))
not supporting deserializing attributes through multiple nested struct definitions. 

I next tried writing proc macros that would auto-generate the base class fields into structs marked with a proc macro attribute.
This turned out to be very tricky and I was never able to get it to work fully. It also relied upon the proc macros adding the new fields
to run before the proc macros generating serialization / deserialization code, which could not be guaranteed.

The current design manually copied the properties from the "base class" into each "derived class" along with doc comments. Because builder methods are in implementation blocks, we use macro_rules! macros to generate builder methods for each field in the base type. This approach
is nice for development because rust-analyzer can show property type hinting, etc without having to invoke a proc macro. It might become
a maintenance burden because we'll need to be careful to keep all the copies in sync whenever the base classes change. This "inheritance" is
also not visible to the user of the library, so we might need to provide some traits like Conclusion or Source that we can expose to users 
of the library in the future. This would be necessary if the users wanted to use multiple different types implementing a common interface
through a trait object.

## Builder Pattern
The [Java GEDCOM X implementation](https://github.com/FamilySearch/gedcomx-java) that this library is loosely based on lacks guarantees that 
the spec makes, such as requiring that certain arrays are non-empty, or that certain URIs resolve to object instances of a certain type.
This library attempts to remedy that by using the [Builder pattern](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html). All 
structs are marked non-exhaustive to prevent creating these structs using struct initialization (and bypassing the type-safety provided by 
the builders). Builders should be used to create and serialize new GEDCOM X documents to get maximum type safety. The default trait 
implementation of all structs means there is an escape hatch, which users can access by using the `default()` method and then setting various 
"raw" properties on structs. This exists because not supporting Default would have made lots of the code much more verbose.
I chose to manually implement the builder pattern instead of using a macro from one of several crates to do so both for increased flexibility 
to add methods later, and to ensure that `Result`s were only returned where necessary. 

However, users may also parse XML / JSON from various sources that may not obey the spec strictly, and thus need access to the underlying types. To support this use case, and to avoid having getter / setter methods for every property on all structs, fields for each struct are public. 
Care should be taken when accessing these structs without going through builders, as it is easy to create non-spec-valid GEDCOM X documents in these cases.

## XML support
Rust's XML ecosystem is somewhat immature for more advanced XML use-cases. XML doesn't work well with serde, despite some support in the 
quick-xml crate, because serde doesn't really support namespaces. After evaluating several libraries, the choice came down to implementing 
serialization / deserialzation manually for every struct using a SAX parser like quick-xml, doing the same but using DOM parser like 
roxmltree, or using Yaserde, which is basically a fork of Serde to support XML specifically. I chose the later path to avoid writing a 
massive amount of serialization code, which has worked out quite well after upstreaming some patches for some more tricky situations we face. 
This means that each struct will have both serde annotations (to support JSON ser/de), and Yaserde annotations (to support XML ser/de).