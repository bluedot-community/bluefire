// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! `cargo`-related utils.

use std::path::PathBuf;

/// Returns the `cargo` manifest path.
pub fn get_manifest_path() -> PathBuf {
    let dir = std::env::var("CARGO_MANIFEST_DIR").expect("Read CARGO_MANIFEST_DIR variable");
    let mut path = std::path::PathBuf::new();
    path.push(&dir);
    path
}

/// Returns the `cargo` output directory.
pub fn get_out_dir() -> PathBuf {
    let dir = std::env::var("OUT_DIR").expect("Read OUT_DIR variable");
    let mut path = std::path::PathBuf::new();
    path.push(&dir);
    path
}

/// Transforms a relative path inside the `cargo` manifest directory into an absolute path.
pub fn as_absolute_path(relative_path: &str) -> PathBuf {
    let mut path = get_manifest_path();
    path.push(relative_path);
    path
}

/// Reads a file from the cargo manifest path.
pub fn read_manifest_path(input: &str) -> String {
    let mut input_path = get_manifest_path();
    input_path.push(input);

    std::fs::read_to_string(input_path.clone()).expect(&format!("Read file: {:?}", input_path))
}

/// Writes to a file in the output directory.
pub fn write_out_file(output: &str, content: &str) -> PathBuf {
    use std::io::Write;

    let mut output_path = get_out_dir();
    output_path.push(output);

    let mut file =
        std::fs::File::create(&output_path).expect(&format!("Create file: {:?}", &output_path));
    file.write_all(content.as_bytes()).expect(&format!("Write to file: {:?}", &output_path));

    output_path
}

/// Returns a string representing a compilition mode: "release" in release mode and "debug"
/// otherwise.
pub fn compilation_mode() -> &'static str {
    #[cfg(build = "release")]
    {
        "release"
    }

    #[cfg(not(build = "release"))]
    {
        "debug"
    }
}
