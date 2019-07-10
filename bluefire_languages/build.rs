// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Build script for `bluefire_languages`.

#![warn(missing_docs)]

use std::env;
use std::fs::File;
use std::io::{BufReader, Write};

use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    family: String,
    name_en: String,
    name_native: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    iso_639_1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    iso_639_2t: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    iso_639_2b: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    iso_639_3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    iso_639_6: Option<String>,
}

impl Record {
    pub fn validate(&self) {
        if let Some(iso_639_1) = &self.iso_639_1 {
            if iso_639_1.len() != 2 {
                panic!("The langth of ISO 639-1 code should equal two, is '{}'", iso_639_1);
            }
        }
        if let Some(iso_639_3) = &self.iso_639_3 {
            if iso_639_3.len() != 3 {
                panic!("The langth of ISO 639-3 code should equal three, is '{}'", iso_639_3);
            }
        }
    }
}

fn main() {
    let dir = env::var("OUT_DIR").expect("Read OUT_DIR variable");
    let input = BufReader::new(include_str!("data/languages.csv").as_bytes());
    let mut output = File::create(dir + "/languages.rs").expect("Create file");

    let mut records = Vec::new();
    let mut rdr = csv::ReaderBuilder::new().from_reader(input);
    for result in rdr.deserialize() {
        let record: Record = result.expect("Read entry as language record");
        record.validate();
        records.push(record);
    }

    records.sort_by(|a, b| a.iso_639_3.cmp(&b.iso_639_3));

    for record in records.iter() {
        if let (Some(iso_639_1), Some(iso_639_3)) = (&record.iso_639_1, &record.iso_639_3) {
            writeln!(output, "/// Info about '{}' language", record.name_en).unwrap();
            writeln!(output, "pub const {}: Language = Language {{", iso_639_3.to_uppercase())
                .unwrap();
            writeln!(output, " iso_639_1: \"{}\",", iso_639_1.to_lowercase()).unwrap();
            writeln!(output, " iso_639_3: \"{}\",", iso_639_3.to_lowercase()).unwrap();
            writeln!(output, " endonym: \"{}\",", record.name_native).unwrap();
            writeln!(output, "}};").unwrap();
        }
    }

    writeln!(output, "/// An array of references to all languages.").unwrap();
    write!(output, "pub const LANGUAGES: [&'static Language; 184] = [").unwrap();
    for record in records.iter() {
        if let Some(iso_639_3) = &record.iso_639_3 {
            write!(output, "&{}, ", iso_639_3.to_uppercase()).unwrap();
        }
    }
    write!(output, "];").unwrap();

    writeln!(output, "impl Language {{").unwrap();
    writeln!(output, "/// ").unwrap();
    writeln!(output, " pub fn from_iso_639_3(code: &str) -> Option<&'static Language> {{").unwrap();
    writeln!(output, "  match code {{").unwrap();
    for record in records.iter() {
        if let Some(iso_639_3) = &record.iso_639_3 {
            writeln!(
                output,
                "   \"{}\" => Some(&{}),",
                iso_639_3.to_lowercase(),
                iso_639_3.to_uppercase(),
            )
            .unwrap();
        }
    }
    writeln!(output, "   _ => None,").unwrap();
    writeln!(output, "}}}}}}").unwrap();
}
