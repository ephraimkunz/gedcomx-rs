// use yaserde_derive::YaDeserialize;

// fn main() {
//     let xml = "<A><b><c>test</c></b></A>";
//     let item: A = yaserde::de::from_str(xml).unwrap();
//     println!("{:?}", item);
// }

// #[derive(YaDeserialize, Default, Debug)]
// struct A {
//     b: B
// }

// #[derive(YaDeserialize, Default, Debug)]
// struct B {
//     b: C
// }

// #[derive(YaDeserialize, Default, Debug)]
// struct C {
//     c: String
// }

use yaserde_derive::YaDeserialize;
fn main() {
    let xml = "<A><b><c>test</c></b></A>";
    let item: A = yaserde::de::from_str(xml).unwrap();
    println!("{:?}", item);
}
struct A {
    b: B,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_YA_DESERIALIZE_FOR_A: () = {
    use ::std::str::FromStr as _;
    use ::yaserde::Visitor as _;
    impl ::yaserde::YaDeserialize for A {
        #[allow(unused_variables)]
        fn deserialize<R: ::std::io::Read>(
            reader: &mut ::yaserde::de::Deserializer<R>,
        ) -> ::std::result::Result<Self, ::std::string::String> {
            let (named_element, struct_namespace) =
                if let ::xml::reader::XmlEvent::StartElement { name, .. } =
                    reader.peek()?.to_owned()
                {
                    (name.local_name.to_owned(), name.namespace.clone())
                } else {
                    (
                        ::std::string::String::from("A"),
                        ::std::option::Option::None,
                    )
                };
            let start_depth = reader.depth();

            if reader.depth() == 0 {
                if let Some(namespace) = struct_namespace {
                    match namespace.as_str() {
                        bad_namespace => {
                            let msg = {
                                let res = String::from("bad namespace");
                                res
                            };
                            return Err(msg);
                        }
                    }
                }
            }
            #[allow(unused_mut)]
            let mut __b_value: B = <B as ::std::default::Default>::default();
            #[allow(non_snake_case, non_camel_case_types)]
            struct __Visitor_B_B;
            impl<'de> ::yaserde::Visitor<'de> for __Visitor_B_B {
                type Value = B;
                fn visit_str(
                    self,
                    v: &str,
                ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                    let content = "<".to_string() + "B" + ">" + v + "</" + "B" + ">";
                    ::yaserde::de::from_str(&content)
                }
            }
            let mut depth = 0;
            loop {
                let event = reader.peek()?.to_owned();

                match event {
                    ::xml::reader::XmlEvent::StartElement {
                        ref name,
                        ref attributes,
                        ..
                    } => {
                        match name.local_name.as_str() {
                            "b" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::xml::reader::XmlEvent::StartElement { .. }) =
                                    reader.peek()
                                {
                                    let value =
                                        <B as ::yaserde::YaDeserialize>::deserialize(reader)?;
                                    __b_value = value;
                                }
                            }
                            _ => {
                                let event = reader.next_event()?;
                                if depth > 0 {
                                    reader.skip_element(|event| {})?;
                                }
                            }
                        }
                        if depth == 0 {}
                        depth += 1;
                    }
                    ::xml::reader::XmlEvent::EndElement { ref name } => {
                        if name.local_name == named_element {
                            break;
                        }
                        let event = reader.next_event()?;
                        depth -= 1;
                    }
                    ::xml::reader::XmlEvent::EndDocument => {
                        if false {
                            break;
                        }
                    }
                    ::xml::reader::XmlEvent::Characters(ref text_content) => {
                        let event = reader.next_event()?;
                    }
                    event => {
                        return ::std::result::Result::Err({
                            let res = String::from("unknown");
                            res
                        });
                    }
                }
            }

            ::std::result::Result::Ok(A { b: __b_value })
        }
    }
};
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::default::Default for A {
    #[inline]
    fn default() -> A {
        A {
            b: ::core::default::Default::default(),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            A { b: ref __self_0_0 } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "A");
                let _ = ::core::fmt::DebugStruct::field(debug_trait_builder, "b", &&(*__self_0_0));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
struct B {
    b: C,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_YA_DESERIALIZE_FOR_B: () = {
    use ::std::str::FromStr as _;
    use ::yaserde::Visitor as _;
    impl ::yaserde::YaDeserialize for B {
        #[allow(unused_variables)]
        fn deserialize<R: ::std::io::Read>(
            reader: &mut ::yaserde::de::Deserializer<R>,
        ) -> ::std::result::Result<Self, ::std::string::String> {
            let (named_element, struct_namespace) =
                if let ::xml::reader::XmlEvent::StartElement { name, .. } =
                    reader.peek()?.to_owned()
                {
                    (name.local_name.to_owned(), name.namespace.clone())
                } else {
                    (
                        ::std::string::String::from("B"),
                        ::std::option::Option::None,
                    )
                };
            let start_depth = reader.depth();

            if reader.depth() == 0 {
                if let Some(namespace) = struct_namespace {
                    match namespace.as_str() {
                        bad_namespace => {
                            let msg = String::from("bad namespace");
                            return Err(msg);
                        }
                    }
                }
            }
            #[allow(unused_mut)]
            let mut __b_value: C = <C as ::std::default::Default>::default();
            #[allow(non_snake_case, non_camel_case_types)]
            struct __Visitor_B_C;
            impl<'de> ::yaserde::Visitor<'de> for __Visitor_B_C {
                type Value = C;
                fn visit_str(
                    self,
                    v: &str,
                ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                    let content = "<".to_string() + "C" + ">" + v + "</" + "C" + ">";
                    ::yaserde::de::from_str(&content)
                }
            }
            let mut depth = 0;
            loop {
                let event = reader.peek()?.to_owned();

                match event {
                    ::xml::reader::XmlEvent::StartElement {
                        ref name,
                        ref attributes,
                        ..
                    } => {
                        match name.local_name.as_str() {
                            "b" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::xml::reader::XmlEvent::StartElement { .. }) =
                                    reader.peek()
                                {
                                    let value =
                                        <C as ::yaserde::YaDeserialize>::deserialize(reader)?;
                                    __b_value = value;
                                }
                            }
                            _ => {
                                let event = reader.next_event()?;
                                if depth > 0 {
                                    reader.skip_element(|event| {})?;
                                }
                            }
                        }
                        if depth == 0 {}
                        depth += 1;
                    }
                    ::xml::reader::XmlEvent::EndElement { ref name } => {
                        if name.local_name == named_element {
                            break;
                        }
                        let event = reader.next_event()?;
                        depth -= 1;
                    }
                    ::xml::reader::XmlEvent::EndDocument => {
                        if false {
                            break;
                        }
                    }
                    ::xml::reader::XmlEvent::Characters(ref text_content) => {
                        let event = reader.next_event()?;
                    }
                    event => {
                        return ::std::result::Result::Err({
                            let res = String::from("unknown");
                            res
                        });
                    }
                }
            }

            ::std::result::Result::Ok(B { b: __b_value })
        }
    }
};
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::default::Default for B {
    #[inline]
    fn default() -> B {
        B {
            b: ::core::default::Default::default(),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for B {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            B { b: ref __self_0_0 } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "B");
                let _ = ::core::fmt::DebugStruct::field(debug_trait_builder, "b", &&(*__self_0_0));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
struct C {
    c: String,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_YA_DESERIALIZE_FOR_C: () = {
    use ::std::str::FromStr as _;
    use ::yaserde::Visitor as _;
    impl ::yaserde::YaDeserialize for C {
        #[allow(unused_variables)]
        fn deserialize<R: ::std::io::Read>(
            reader: &mut ::yaserde::de::Deserializer<R>,
        ) -> ::std::result::Result<Self, ::std::string::String> {
            let (named_element, struct_namespace) =
                if let ::xml::reader::XmlEvent::StartElement { name, .. } =
                    reader.peek()?.to_owned()
                {
                    (name.local_name.to_owned(), name.namespace.clone())
                } else {
                    (
                        ::std::string::String::from("C"),
                        ::std::option::Option::None,
                    )
                };
            let start_depth = reader.depth();

            if reader.depth() == 0 {
                if let Some(namespace) = struct_namespace {
                    match namespace.as_str() {
                        bad_namespace => {
                            let msg = String::from("bad namespace");
                            return Err(msg);
                        }
                    }
                }
            }
            #[allow(unused_mut)]
            let mut __c_value: ::std::string::String =
                <::std::string::String as ::std::default::Default>::default();
            #[allow(non_snake_case, non_camel_case_types)]
            struct __Visitor_C_;
            impl<'de> ::yaserde::Visitor<'de> for __Visitor_C_ {
                type Value = ::std::string::String;
                fn visit_str(
                    self,
                    v: &str,
                ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                    ::std::result::Result::Ok(::std::string::String::from_str(v).unwrap())
                }
            }
            let mut depth = 0;
            loop {
                let event = reader.peek()?.to_owned();

                match event {
                    ::xml::reader::XmlEvent::StartElement {
                        ref name,
                        ref attributes,
                        ..
                    } => {
                        match name.local_name.as_str() {
                            "c" => {
                                let visitor = __Visitor_C_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        bad_namespace => {
                                            let msg = String::from("bad namespace");

                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err({
                                                String::from("Unable to parse content for")
                                            })
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __c_value = value
                                }
                            }
                            _ => {
                                let event = reader.next_event()?;
                                if depth > 0 {
                                    reader.skip_element(|event| {})?;
                                }
                            }
                        }
                        if depth == 0 {}
                        depth += 1;
                    }
                    ::xml::reader::XmlEvent::EndElement { ref name } => {
                        if name.local_name == named_element {
                            break;
                        }
                        let event = reader.next_event()?;
                        depth -= 1;
                    }
                    ::xml::reader::XmlEvent::EndDocument => {
                        if false {
                            break;
                        }
                    }
                    ::xml::reader::XmlEvent::Characters(ref text_content) => {
                        let event = reader.next_event()?;
                    }
                    event => {
                        return ::std::result::Result::Err({
                            let res = String::from("unknown");
                            res
                        });
                    }
                }
            }

            ::std::result::Result::Ok(C { c: __c_value })
        }
    }
};
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::default::Default for C {
    #[inline]
    fn default() -> C {
        C {
            c: ::core::default::Default::default(),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for C {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            C { c: ref __self_0_0 } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "C");
                let _ = ::core::fmt::DebugStruct::field(debug_trait_builder, "c", &&(*__self_0_0));
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
