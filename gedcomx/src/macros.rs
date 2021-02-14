macro_rules! try_from_evidencereference {
    ($from_type: ty) => {
        impl TryFrom<&$from_type> for crate::EvidenceReference {
            type Error = GedcomxError;

            fn try_from(f: &$from_type) -> Result<Self, Self::Error> {
                use crate::Conclusion;
                match &f.conclusion().id {
                    Some(id) => Ok(Self::new(id.into(), None)),
                    None => Err(GedcomxError::NoId(f.type_name())),
                }
            }
        }
    };
}

macro_rules! impl_enumasstring_yaserialize_yadeserialize {
    ($for_type: ty, $name: tt) => {
        impl yaserde::YaSerialize for $for_type {
            fn serialize<W: std::io::Write>(
                &self,
                writer: &mut yaserde::ser::Serializer<W>,
            ) -> std::result::Result<(), String> {
                let _ret = writer.write(xml::writer::XmlEvent::characters(&self.to_string()));
                Ok(())
            }

            fn serialize_attributes(
                &self,
                attributes: Vec<xml::attribute::OwnedAttribute>,
                namespace: xml::namespace::Namespace,
            ) -> std::result::Result<
                (
                    Vec<xml::attribute::OwnedAttribute>,
                    xml::namespace::Namespace,
                ),
                String,
            > {
                Ok((attributes, namespace))
            }
        }

        impl yaserde::YaDeserialize for $for_type {
            fn deserialize<R: std::io::Read>(
                reader: &mut yaserde::de::Deserializer<R>,
            ) -> std::result::Result<Self, String> {
                if let xml::reader::XmlEvent::StartElement { name, .. } = reader.peek()?.to_owned()
                {
                    let expected_name = $name.to_owned();
                    if name.local_name != expected_name {
                        return Err(format!(
                            "Wrong StartElement name: {}, expected: {}",
                            name, expected_name
                        ));
                    }
                    let _next = reader.next_event();
                } else {
                    return Err("StartElement missing".to_string());
                }

                if let xml::reader::XmlEvent::Characters(text) = reader.peek()?.to_owned() {
                    let enum_as_string = crate::EnumAsString(text);
                    Ok(Self::from(enum_as_string))
                } else {
                    Err("Characters missing".to_string())
                }
            }
        }
    };
}

macro_rules! impl_characters_yaserialize_yadeserialize {
    ($for_type: ty, $name: tt) => {
        impl yaserde::YaSerialize for $for_type {
            fn serialize<W: std::io::Write>(
                &self,
                writer: &mut yaserde::ser::Serializer<W>,
            ) -> Result<(), String> {
                let _ret = writer.write(xml::writer::XmlEvent::characters(&self.0));
                Ok(())
            }

            fn serialize_attributes(
                &self,
                attributes: Vec<xml::attribute::OwnedAttribute>,
                namespace: xml::namespace::Namespace,
            ) -> Result<
                (
                    Vec<xml::attribute::OwnedAttribute>,
                    xml::namespace::Namespace,
                ),
                String,
            > {
                Ok((attributes, namespace))
            }
        }

        impl yaserde::YaDeserialize for $for_type {
            fn deserialize<R: std::io::Read>(
                reader: &mut yaserde::de::Deserializer<R>,
            ) -> Result<Self, String> {
                if let xml::reader::XmlEvent::StartElement { name, .. } = reader.peek()?.to_owned()
                {
                    let expected_name = $name.to_owned();
                    if name.local_name != expected_name {
                        return Err(format!(
                            "Wrong StartElement name: {}, expected: {}",
                            name, expected_name
                        ));
                    }
                    let _next = reader.next_event();
                } else {
                    return Err("StartElement missing".to_string());
                }

                if let xml::reader::XmlEvent::Characters(text) = reader.peek()?.to_owned() {
                    Ok(Self(text))
                } else {
                    Err("Characters missing".to_string())
                }
            }
        }
    };
}

macro_rules! conclusion_builder_functions {
    ($final_type: ty) => {
        pub fn id<I: Into<crate::Id>>(&mut self, id: I) -> &mut Self {
            use crate::Conclusion;
            self.0.conclusion_mut().id = Some(id.into());
            self
        }

        pub fn lang<I: Into<crate::Lang>>(&mut self, lang: I) -> &mut Self {
            self.0.conclusion_mut().lang = Some(lang.into());
            self
        }

        /// # Errors
        ///
        /// Will return [`GedcomxError::NoId`](crate::GedcomxError::NoId) if a
        /// conversion into [`SourceReference`](crate::SourceReference) fails.
        /// This happens if `source` has no `id` set.
        pub fn source<
            I: std::convert::TryInto<crate::SourceReference, Error = crate::GedcomxError>,
        >(
            &mut self,
            source: I,
        ) -> crate::Result<&mut Self> {
            self.0.conclusion_mut().sources.push(source.try_into()?);
            Ok(self)
        }

        /// # Errors
        ///
        /// Will return [`GedcomxError`](crate::GedcomxError) if a conversion into
        /// [`Document`](crate::Document) fails. This happens if `document` has no `id`
        /// set or has the wrong `document_type`.
        pub fn analysis(&mut self, document: &crate::Document) -> crate::Result<&mut Self> {
            use std::convert::TryInto;
            self.0.conclusion_mut().analysis = Some(document.try_into()?);
            Ok(self)
        }

        pub fn note(&mut self, note: crate::Note) -> &mut Self {
            self.0.conclusion_mut().notes.push(note);
            self
        }

        pub fn confidence(&mut self, confidence: crate::ConfidenceLevel) -> &mut Self {
            self.0.conclusion_mut().confidence = Some(confidence);
            self
        }

        pub fn attribution(&mut self, attribution: crate::Attribution) -> &mut Self {
            self.0.conclusion_mut().attribution = Some(attribution);
            self
        }
    };
}

macro_rules! subject_builder_functions {
    ($final_type: ty) => {
        conclusion_builder_functions!($final_type);

        pub fn extracted(&mut self, extracted: bool) -> &mut Self {
            self.0.subject.extracted = Some(extracted);
            self
        }

        /// # Errors
        ///
        /// Will return [`GedcomxError::NoId`](crate::GedcomxError::NoId) if a
        /// conversion into [`EvidenceReference`](crate::EvidenceReference) fails.
        /// This happens if the passed argument has no `id` set.
        pub fn evidence(&mut self, e: &$final_type) -> crate::Result<&mut Self> {
            use std::convert::TryInto;
            self.0.subject.evidence.push(e.try_into()?);
            Ok(self)
        }

        /// # Errors
        ///
        /// Will return [`GedcomxError::NoId`](crate::GedcomxError::NoId) if a
        /// conversion into [`SourceReference`](crate::SourceReference) fails.
        /// This happens if `media` has no `id` set.
        pub fn media(&mut self, media: &crate::SourceDescription) -> crate::Result<&mut Self> {
            use std::convert::TryInto;
            self.0.subject.media.push(media.try_into()?);
            Ok(self)
        }

        pub fn identifier(&mut self, identifier: crate::Identifier) -> &mut Self {
            self.0.subject.identifiers.push(identifier);
            self
        }
    };
}
