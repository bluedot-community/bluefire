// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Utils for processing static files.

#![warn(missing_docs)]

use std::{fs::File, path::PathBuf};

use bluefire_static_files_core::prelude::*;

// -------------------------------------------------------------------------------------------------

mod utils {
    pub fn path_to_str(path: &std::path::PathBuf) -> String {
        path.to_str().expect("Expected to cast path to an UTF8 string").to_string()
    }
}

// -------------------------------------------------------------------------------------------------

/// Processes static files defined in `static.yaml`. This function should be called from your
/// `build.rs` script to provide files required by `bluefire_static_files_macros`.
///
/// - uses `wasm-bindgen` to process `wasm` files
/// - uses `sass` to convert `scss` to `css`
/// - minimizes `js` files
pub fn build() {
    let path = crate::cargo::get_manifest_path();
    FileProcessor::new(path).process();
}

// -------------------------------------------------------------------------------------------------

struct FileProcessor {
    spec: Spec,
    path: PathBuf,
}

impl FileProcessor {
    pub fn new(path: PathBuf) -> Self {
        let mut template_path = path.clone();
        template_path.push("static.yaml");
        println!("cargo:rerun-if-changed={}", utils::path_to_str(&template_path));

        let spec = Spec::read(template_path);

        Self { spec, path }
    }

    pub fn process(self) {
        for source in self.spec.sources.iter() {
            match &source.variant {
                Type::Js { .. } => self.process_js(source),
                Type::Scss { .. } => self.process_scss(source),
                Type::Wasm { target_path, .. } => {
                    self.process_wasm(target_path, &source.input_base_name)
                }
            }
        }
    }

    pub fn process_js(&self, source: &Source) {
        use std::io::{Read, Write};

        let mut input_path = self.path.clone();
        input_path.push(&self.spec.source_dir);
        input_path.push(&source.input_base_name);
        input_path.set_extension("js");
        println!("cargo:rerun-if-changed={}", utils::path_to_str(&input_path));

        let mut output_path = crate::cargo::get_out_dir();
        output_path.push(&source.output_base_name);
        output_path.set_extension("js");

        let mut input_content = String::new();
        let mut input_file =
            File::open(&input_path).expect(&format!("Open file: {:?}", &input_path));
        input_file
            .read_to_string(&mut input_content)
            .expect(&format!("Read file: {:?}", &input_path));

        let content = minifier::js::minify(&input_content);
        let mut output_file =
            File::create(&output_path).expect(&format!("Create file: {:?}", &output_path));
        output_file
            .write_all(content.as_bytes())
            .expect(&format!("Write to file: {:?}", &output_path));
    }

    pub fn process_scss(&self, source: &Source) {
        let mut input_path = self.path.clone();
        input_path.push(&self.spec.source_dir);
        input_path.push(&source.input_base_name);
        input_path.set_extension("scss");
        let input_str = utils::path_to_str(&input_path);
        println!("cargo:rerun-if-changed={}", input_str);

        let mut output_path = crate::cargo::get_out_dir();
        output_path.push(&source.output_base_name);
        output_path.set_extension("css");
        let output_str = utils::path_to_str(&output_path);

        let mut cache_location = crate::cargo::get_out_dir();
        cache_location.push("sass-cache");
        let cache_str = utils::path_to_str(&cache_location);

        crate::commands::sass::Sass::new(input_str, output_str)
            .style(crate::commands::sass::Style::Expanded)
            .sourcemap(crate::commands::sass::Sourcemap::None)
            .cache_location(cache_str)
            .run();
    }

    pub fn process_wasm(&self, target_path: &str, input_base_name: &str) {
        let mut path = crate::cargo::get_manifest_path();
        path.push(target_path);
        path.push("wasm32-unknown-unknown");
        path.push(crate::cargo::compilation_mode());
        path.push(&input_base_name);
        path.set_extension("wasm");
        let path_str = utils::path_to_str(&path);
        println!("cargo:rerun-if-changed={}", path_str);

        crate::commands::wasm_bindgen::WasmBindgen::new(path_str)
            .target(crate::commands::wasm_bindgen::Target::NoModules)
            .outdir(utils::path_to_str(&crate::cargo::get_out_dir()))
            .run();
    }
}
