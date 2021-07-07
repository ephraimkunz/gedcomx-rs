// use std::{
//     collections::HashMap,
//     fmt::{Debug, Formatter},
//     io::{self},
// };
// use thiserror::Error;

// #[derive(Debug)]
// struct GedcomxFile<R> {
//     reader: zip::ZipArchive<R>,
// }

// struct GedcomxFileEntry<'a> {
//     filename: &'a str,
// }

// impl<'a> Debug for GedcomxFileEntry<'a> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
//         f.debug_struct("GedcomxFileEntry").finish()
//     }
// }

// impl<R: io::Read + io::Seek> GedcomxFile<R> {
//     fn from_reader(reader: R) -> Result<Self, GedcomxFileError> {
//         let zip = zip::ZipArchive::new(reader)?;
//         Ok(Self { reader: zip })
//     }

//     /// Get the entries found in this GEDCOM X file. Does not include the manifest.
//     fn entries<'a>(&'a mut self) -> impl Iterator<Item = GedcomxFileEntry> {
//         GedcomxFileIter {
//             inner: self.reader.file_names(),
//         }
//     }

//     /// Get the manifest.
//     fn manifest(&self) -> GedcomxManifest {
//         todo!()
//     }

//     /// Get the attributes that have been associated with this GEDCOM X file.
//     fn attributes(&self) -> HashMap<String, String> {
//         todo!()
//     }
// }

// struct GedcomxFileIter<I> {
//     inner: I,
// }

// impl<'a, I: Iterator<Item = &'a str>> Iterator for GedcomxFileIter<I> {
//     type Item = GedcomxFileEntry<'a>;

//     fn next(&mut self) -> Option<GedcomxFileEntry<'a>> {
//         let filename = match self.inner.next() {
//             Some(s) => s,
//             None => return None,
//         };

//         if filename == "META-INF/MANIFEST.MF" {
//             return self.next();
//         }

//         Some(GedcomxFileEntry { filename })
//     }
// }

// struct GedcomxManifest {}

// #[derive(Error, Debug)]
// enum GedcomxFileError {
//     #[error("zip error")]
//     ZipError(#[from] zip::result::ZipError),
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::{fs::File, path::Path};

//     #[test]
//     fn read() {
//         let fp = Path::new("data/sample.gedx");
//         let f = File::open(fp).unwrap();
//         let mut gxf = GedcomxFile::from_reader(f).unwrap();
//         for entry in gxf.entries() {
//             let gx = gxf.read_resource(entry);
//         }
//     }
// }
