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
        /// Will return `GedcomxError` if a conversion into `SourceReference` fails.
        /// This happens if the `source` has no Id set.
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
        /// Will return `GedcomxError` if a conversion into [`Document`](crate::Document) fails.
        /// This happens if the `document` has no [`Id`](crate::Id) set or has the wrong [`DocumentType`](crate::DocumentType).
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

        pub fn evidence(&mut self, e: &$final_type) -> crate::Result<&mut Self> {
            use std::convert::TryInto;
            self.0.subject.evidence.push(e.try_into()?);
            Ok(self)
        }

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
