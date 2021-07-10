use std::{
    collections::HashMap,
    fmt::Debug,
    io::{self, BufRead, BufReader, Read},
    path::Path,
};
use thiserror::Error;
use zip::{read::ZipFile, result::ZipError};

/// Types of entries in a GedcomxFile.
#[derive(Debug)]
pub enum GedcomxFileEntry<R: Read> {
    /// A JSON or XML document in GEDCOM X format that has been deserialized.
    Gedcomx(gedcomx::Gedcomx),

    /// A GedcomxFile manifest.
    Manifest(GedcomxManifest),

    /// Any other filetypes are returned as a type implementing Read.
    Reader(R),
}

const MANIFEST_STR: &str = "META-INF/MANIFEST.MF";

#[derive(Debug)]
pub struct GedcomxFile<R> {
    inner: zip::ZipArchive<R>,
}

impl<R: io::Read + io::Seek> GedcomxFile<R> {
    pub fn from_reader(reader: R) -> Result<Self, GedcomxFileError> {
        let zip = zip::ZipArchive::new(reader)?;
        Ok(Self { inner: zip })
    }

    /// Number of files contained in this GedcomxFile.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Whether this GedcomxFile contains no files.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn by_index(
        &mut self,
        file_number: usize,
    ) -> Result<GedcomxFileEntry<impl Read + '_>, GedcomxFileError> {
        let entry = self.inner.by_index(file_number)?;
        Self::file_entry_from_entry(entry)
    }

    /// Get the names of the files in this GedcomxFile. These can be used as arguments for `by_name`.
    pub fn file_names(&self) -> impl Iterator<Item = &str> {
        self.inner.file_names()
    }

    /// Get the entries by name in this GEDCOM X file.
    pub fn by_name(
        &mut self,
        name: &str,
    ) -> Result<GedcomxFileEntry<impl Read + '_>, GedcomxFileError> {
        let entry = self.inner.by_name(name)?;
        Self::file_entry_from_entry(entry)
    }

    fn file_entry_from_entry(
        mut entry: ZipFile,
    ) -> Result<GedcomxFileEntry<impl Read + '_>, GedcomxFileError> {
        if entry.enclosed_name() == Some(Path::new(MANIFEST_STR)) {
            return Ok(GedcomxFileEntry::Manifest(GedcomxManifest::from_reader(
                entry,
            )?));
        }

        match entry
            .enclosed_name()
            .and_then(|n| n.extension())
            .and_then(|e| e.to_str())
        {
            Some("json") => match gedcomx::Gedcomx::from_json_reader(&mut entry) {
                Ok(gx) => Ok(GedcomxFileEntry::Gedcomx(gx)),
                Err(e) => Err(GedcomxFileError::GedcomxError(e)),
            },
            Some("xml") => match gedcomx::Gedcomx::from_xml_reader(&mut entry) {
                Ok(gx) => Ok(GedcomxFileEntry::Gedcomx(gx)),
                Err(e) => Err(GedcomxFileError::GedcomxError(e)),
            },
            _ => Ok(GedcomxFileEntry::Reader(entry)),
        }
    }

    /// Get the manifest, or return an error if it's missing or unreadable.
    pub fn manifest(&mut self) -> Result<GedcomxManifest, GedcomxFileError> {
        match self.by_name(MANIFEST_STR)? {
            GedcomxFileEntry::Manifest(m) => Ok(m),
            _ => Err(GedcomxFileError::MissingManifest),
        }
    }

    /// Get the attributes that have been associated with this GEDCOM X file.
    pub fn attributes_by_name(
        &mut self,
        name: &str,
    ) -> Result<HashMap<String, String>, GedcomxFileError> {
        let manifest = self.manifest()?;
        manifest
            .attributes_by_name(name)
            .ok_or(GedcomxFileError::ZipError(ZipError::FileNotFound))
    }

    /// Get the attributes that have been associated with this GEDCOM X file.
    pub fn attributes_by_index(
        &mut self,
        file_number: usize,
    ) -> Result<HashMap<String, String>, GedcomxFileError> {
        let name = {
            let entry = self.inner.by_index(file_number)?;
            let name = entry.name().to_string();
            name
        };
        self.attributes_by_name(&name)
    }
}

#[derive(Debug)]
pub struct GedcomxManifest {
    inner: HashMap<String, HashMap<String, String>>,
}

impl GedcomxManifest {
    fn from_reader<R>(reader: R) -> Result<Self, GedcomxFileError>
    where
        R: Read,
    {
        let mut sections = HashMap::new();

        let mut current_section = {
            let mut m = HashMap::new();
            m.insert("Name".to_string(), "main".to_string());
            m
        };

        let buf_reader = BufReader::new(reader);
        for line in buf_reader.lines() {
            let line = line.map_err(|_| GedcomxFileError::InvalidManifest)?;

            if line.is_empty() {
                // New section, save the old one.
                if !current_section.is_empty() {
                    let name = current_section
                        .get("Name")
                        .ok_or(GedcomxFileError::InvalidManifest)?
                        .to_string();
                    sections.insert(name, current_section.clone());
                    current_section.clear();
                }
            } else if let Some((key, value)) = line.split_once(":") {
                current_section.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
        Ok(Self { inner: sections })
    }

    pub fn attributes_by_name(&self, name: &str) -> Option<HashMap<String, String>> {
        self.inner.get(name).cloned()
    }
}

/// Errors produced by the crate.
#[derive(Error, Debug)]
pub enum GedcomxFileError {
    /// Error while zipping / unzipping a file.
    #[error("zip error")]
    ZipError(#[from] zip::result::ZipError),

    /// Error while parsing the contents of a GEDCOM X file.
    #[error("gedcomx error")]
    GedcomxError(#[from] gedcomx::GedcomxError),

    /// No manifest file was found in this GedcomxFile.
    #[error("no manifest in gedcomx file")]
    MissingManifest,

    /// The manifest did not have the correct format.
    #[error("invalid manifest")]
    InvalidManifest,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, iter::FromIterator, path::Path};

    #[test]
    fn read_by_name() {
        let fp = Path::new("data/sample.gedx");
        let f = File::open(fp).unwrap();
        let mut gxf = GedcomxFile::from_reader(f).unwrap();
        let names: Vec<_> = gxf.file_names().map(|s| s.to_string()).collect();
        for name in names {
            let gx = gxf.by_name(name.as_str()).unwrap();
            match gx {
                GedcomxFileEntry::Gedcomx(g) => println!("{:?}", g),
                GedcomxFileEntry::Manifest(m) => println!("Manifest: {:?}", m),
                GedcomxFileEntry::Reader(_) => println!("Reader"),
            }
        }
    }

    #[test]
    fn read_by_index() {
        let fp = Path::new("data/sample.gedx");
        let f = File::open(fp).unwrap();
        let mut gxf = GedcomxFile::from_reader(f).unwrap();
        for index in 0..gxf.len() {
            let gx = gxf.by_index(index).unwrap();
            match gx {
                GedcomxFileEntry::Gedcomx(g) => println!("Gedcomx {:?}", g),
                GedcomxFileEntry::Manifest(m) => println!("Manifest: {:?}", m),
                GedcomxFileEntry::Reader(_) => println!("Reader"),
            }
        }
    }

    #[test]
    fn manifest() {
        let fp = Path::new("data/sample.gedx");
        let f = File::open(fp).unwrap();
        let mut gxf = GedcomxFile::from_reader(f).unwrap();

        let expected = {
            let main = HashMap::<_, _>::from_iter([
                ("Name".to_string(), "main".to_string()),
                ("Manifest-Version".to_string(), "1.0".to_string()),
                (
                    "Created-By".to_string(),
                    "FamilySearch Platform API 0.1".to_string(),
                ),
            ]);
            let person1 = HashMap::<_, _>::from_iter([
                ("Name".to_string(), "person1.png".to_string()),
                ("Content-Type".to_string(), "image/png".to_string()),
                (
                    "X-DC-modified".to_string(),
                    "2014-10-07T21:15:57.161Z".to_string(),
                ),
            ]);
            let person2 = HashMap::<_, _>::from_iter([
                ("Name".to_string(), "person2.png".to_string()),
                ("Content-Type".to_string(), "image/png".to_string()),
                (
                    "X-DC-modified".to_string(),
                    "2014-10-07T21:15:57.162Z".to_string(),
                ),
            ]);
            let tree = HashMap::<_, _>::from_iter([
                ("Name".to_string(), "tree.xml".to_string()),
                (
                    "Content-Type".to_string(),
                    "application/x-gedcomx-v1+xml".to_string(),
                ),
                (
                    "X-DC-modified".to_string(),
                    "2014-10-07T21:15:57.148Z".to_string(),
                ),
            ]);

            HashMap::<_, _>::from_iter([
                ("main".to_string(), main),
                ("person1.png".to_string(), person1),
                ("person2.png".to_string(), person2),
                ("tree.xml".to_string(), tree),
            ])
        };

        let actual = gxf.manifest().unwrap().inner;

        // Outer keys
        assert!(actual.keys().all(|k| expected.contains_key(k)));
        assert!(expected.keys().all(|k| actual.contains_key(k)));

        // Inner keys and values
        for (name, section) in &actual {
            for (k, v) in section {
                assert_eq!(v, expected.get(name).unwrap().get(k).unwrap());
            }
        }

        for (name, section) in &expected {
            for (k, v) in section {
                assert_eq!(v, actual.get(name).unwrap().get(k).unwrap());
            }
        }
    }
}
