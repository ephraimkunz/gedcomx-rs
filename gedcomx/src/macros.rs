macro_rules! conclusion_builder_functions {
    () => {
        pub fn id<I: Into<crate::Id>>(&mut self, id: I) -> &mut Self {
            self.0.conclusion.id = Some(id.into());
            self
        }

        pub fn lang<I: Into<crate::Lang>>(&mut self, lang: I) -> &mut Self {
            self.0.conclusion.lang = Some(lang.into());
            self
        }

        pub fn source<
            I: std::convert::TryInto<crate::SourceReference, Error = crate::GedcomxError>,
        >(
            &mut self,
            source: I,
        ) -> crate::Result<&mut Self> {
            self.0.conclusion.sources.push(source.try_into()?);
            Ok(self)
        }

        pub fn analysis(&mut self, document: &crate::Document) -> crate::Result<&mut Self> {
            use std::convert::TryInto;
            self.0.conclusion.analysis = Some(document.try_into()?);
            Ok(self)
        }

        pub fn note(&mut self, note: crate::Note) -> &mut Self {
            self.0.conclusion.notes.push(note);
            self
        }

        pub fn confidence(&mut self, confidence: crate::ConfidenceLevel) -> &mut Self {
            self.0.conclusion.confidence = Some(confidence);
            self
        }

        pub fn attribution(&mut self, attribution: crate::Attribution) -> &mut Self {
            self.0.conclusion.attribution = Some(attribution);
            self
        }
    };
}

macro_rules! subject_builder_functions {
    () => {
        // Every thing a conclusion has (just with a slightly different path)

        pub fn id<I: Into<crate::Id>>(&mut self, id: I) -> &mut Self {
            self.0.subject.conclusion.id = Some(id.into());
            self
        }

        pub fn lang<I: Into<crate::Lang>>(&mut self, lang: I) -> &mut Self {
            self.0.subject.conclusion.lang = Some(lang.into());
            self
        }

        pub fn source<
            I: std::convert::TryInto<crate::SourceReference, Error = crate::GedcomxError>,
        >(
            &mut self,
            source: I,
        ) -> crate::Result<&mut Self> {
            self.0.subject.conclusion.sources.push(source.try_into()?);
            Ok(self)
        }

        pub fn analysis(&mut self, document: &crate::Document) -> crate::Result<&mut Self> {
            use std::convert::TryInto;
            self.0.subject.conclusion.analysis = Some(document.try_into()?);
            Ok(self)
        }

        pub fn note(&mut self, note: crate::Note) -> &mut Self {
            self.0.subject.conclusion.notes.push(note);
            self
        }

        pub fn confidence(&mut self, confidence: crate::ConfidenceLevel) -> &mut Self {
            self.0.subject.conclusion.confidence = Some(confidence);
            self
        }

        pub fn attribution(&mut self, attribution: crate::Attribution) -> &mut Self {
            self.0.subject.conclusion.attribution = Some(attribution);
            self
        }

        // And stuff unique to the subject.

        pub fn extracted(&mut self, extracted: bool) -> &mut Self {
            self.0.subject.extracted = Some(extracted);
            self
        }

        pub fn evidence<
            I: std::convert::TryInto<crate::EvidenceReference, Error = crate::GedcomxError>,
        >(
            &mut self,
            e: I,
        ) -> crate::Result<&mut Self> {
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
