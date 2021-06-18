// use std::{
//     collections::HashMap,
//     fmt::{Debug, Formatter},
//     io::{self, Read, Seek},
// };
// use thiserror::Error;

// #[derive(Debug)]
// struct GedcomxFile<R> {
//     reader: zip::ZipArchive<R>,
// }

// struct GedcomxFileEntry<'a> {
//     inner: zip::read::ZipFile<'a>,
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
//     fn entries<'a>(&'a mut self) -> GedcomxFileIter<'a, R> {
//         GedcomxFileIter { i: 0, file: self }
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

// struct GedcomxFileIter<'a, R> {
//     i: usize,
//     file: &'a mut GedcomxFile<R>,
// }

// impl<'a, R: Read + Seek> Iterator for GedcomxFileIter<'a, R> {
//     type Item = GedcomxFileEntry<'a>;

//     fn next(&mut self) -> Option<<Self as Iterator>::Item> {
//         if self.i >= self.file.reader.len() {
//             return None;
//         }

//         let item = self
//             .file
//             .reader
//             .by_index(self.i)
//             .ok()
//             .map(|f| GedcomxFileEntry { inner: f });
//         self.i += 1;

//         item
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
//         for f in gxf.entries() {
//             println!("{:?}", f);
//         }
//     }
// }
