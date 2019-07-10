// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! An application converting `bluefire_protogen` API specifications to code representation.

#![warn(missing_docs)]

use std::io::Write;
use std::str::FromStr;

enum Mode {
    Proto,
    Routes,
    Paths,
}

impl FromStr for Mode {
    type Err = ();

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if string == "protocol" {
            Ok(Mode::Proto)
        } else if string == "routes" {
            Ok(Mode::Routes)
        } else if string == "paths" {
            Ok(Mode::Paths)
        } else {
            Err(())
        }
    }
}

fn gen_proto(input: String) {
    let api = bluefire_protogen::spec::Api::from_str(&input).expect("Parse the spec file");
    let generator = bluefire_protogen::rust_generator::RustGenerator::new();
    let result = generator.generate_api(&api);
    std::io::stdout().write(result.as_ref()).unwrap();
}

fn gen_routes(input: String) {
    let routes = bluefire_protogen::spec::Routes::from_str(&input).expect("Parse the spec file");
    let generator = bluefire_protogen::rust_generator::RustGenerator::new();
    let result = generator.generate_routes(&routes);
    std::io::stdout().write(result.as_ref()).unwrap();
}

fn gen_paths(input: String) {
    let routes = bluefire_protogen::spec::Routes::from_str(&input).expect("Parse the spec file");
    let generator = bluefire_protogen::rust_generator::RustGenerator::new();
    let result = generator.generate_paths(&routes);
    std::io::stdout().write(result.as_ref()).unwrap();
}

fn main() {
    let matches = clap::App::new("BlueFire Protocol Generator")
        .arg(
            clap::Arg::with_name("mode")
                .long("mode")
                .value_name("MODE")
                .help("Sets the mode")
                .takes_value(true)
                .required(true)
                .possible_values(&["protocol", "routes", "paths"]),
        )
        .arg(
            clap::Arg::with_name("input")
                .long("input")
                .value_name("INPUT")
                .help("File to read the specifications from")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let mode = Mode::from_str(matches.value_of("mode").unwrap()).unwrap();
    let input = matches.value_of("input").unwrap();
    let spec = std::fs::read_to_string(input).expect("Read file");
    match mode {
        Mode::Proto => gen_proto(spec),
        Mode::Routes => gen_routes(spec),
        Mode::Paths => gen_paths(spec),
    }
}
