#![feature(fmt_internals)]

use yaserde_derive::{YaDeserialize, YaSerialize};

#[derive(YaDeserialize, YaSerialize, Debug)]
struct Test {
    #[yaserde(text)]
    pub ektest: Option<String>,
}

fn main() {
    let s = "<Test>hi again</Test>";
    let t: Test = yaserde::de::from_str(s).unwrap();
    println!("{:?}", t);

    println!("{}", yaserde::ser::to_string(&t).unwrap());
}
