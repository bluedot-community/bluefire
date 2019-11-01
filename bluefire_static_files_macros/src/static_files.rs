// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Implementation of macro generating static files names and routes provider.

use std::path::PathBuf;

use askama::Template;

use bluefire_build::cargo;
use bluefire_static_files_core::prelude::*;

const DEFAULT_PATH: &str = ".";
const DEFAULT_NAMESPACE: &str = "_";
const TEMPLATE_FILENAME: &str = "static.yaml";

const CONTENT_TYPE_CSS: &str = "text/css";
const CONTENT_TYPE_JS: &str = "application/javascript";
const CONTENT_TYPE_WASM: &str = "application/wasm";

// -------------------------------------------------------------------------------------------------

#[derive(Debug)]
struct Config {
    template_path: PathBuf,
    namespace: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            template_path: cargo::as_absolute_path(DEFAULT_PATH),
            namespace: DEFAULT_NAMESPACE.to_string(),
        }
    }
}

struct Info {
    struct_name: proc_macro2::Ident,
}

fn parse_attributes(attributes: proc_macro::TokenStream) -> Config {
    let mut config = Config::default();
    if !attributes.is_empty() {
        let meta: syn::Meta = syn::parse(attributes).expect("failed to parse attributes");
        match meta {
            syn::Meta::NameValue(value) => {
                let a = value.path.get_ident().expect("Get ident").to_string();
                match a.as_ref() {
                    "path" => match value.lit {
                        syn::Lit::Str(ref lit_str) => {
                            let path = cargo::as_absolute_path(&lit_str.value());
                            config.template_path = path;
                        }
                        _ => panic!("Argument '{}' must be a string", a),
                    },
                    "namespace" => match value.lit {
                        syn::Lit::Str(ref lit_str) => {
                            config.namespace = lit_str.value();
                        }
                        _ => panic!("Argument '{}' must be a string", a),
                    },
                    _ => panic!("Unknown argument '{}'", a),
                }
            }
            _ => panic!("All arguments are expected to be name-value"),
        }
    }
    config
}

fn parse_input(input: proc_macro::TokenStream) -> Info {
    let item: syn::Item = syn::parse(input).expect("Failed to parse token stream");
    match item {
        syn::Item::Struct(item_struct) => {
            match item_struct.fields {
                syn::Fields::Unit => {}
                _ => panic!("This macro can be applied only to unit structures"),
            }
            Info { struct_name: item_struct.ident }
        }
        _ => panic!("This macro can be applied only to structures"),
    }
}

// -------------------------------------------------------------------------------------------------

fn read_template(config: &Config) -> Spec {
    let mut path = cargo::get_manifest_path();
    path.push(&config.template_path);
    path.push(TEMPLATE_FILENAME);
    Spec::read(path)
}

// -------------------------------------------------------------------------------------------------

/// Helper structure for calling rust code from within a template.
#[derive(Clone, Debug)]
struct GeneratorCallback;

impl GeneratorCallback {
    /// Constructs a new `GeneratorCallback`.
    pub fn new() -> Self {
        Self
    }

    /// Generates
    ///  - input paths for the given source to be read from
    ///  - output paths through with they will be accessible on the server
    ///  - content type of the given resource
    pub fn make_paths(&self, source: &Source) -> Vec<(String, String, String)> {
        fn make_input_path(source: &Source, suffix: &str, extension: &str) -> String {
            let mut path = cargo::get_out_dir();
            path.push(source.input_base_name.clone() + suffix);
            path.set_extension(extension);
            path.to_str().expect("Cast path to a string").to_string()
        }

        match &source.variant {
            Type::Scss { .. } => {
                let input_path = make_input_path(&source, "", "css");
                let output_name = source.output_base_name.clone() + ".css";
                vec![(input_path, output_name, CONTENT_TYPE_CSS.to_string())]
            }
            Type::Js { .. } => {
                let input_path = make_input_path(&source, "", "js");
                let output_name = source.output_base_name.clone() + ".js";
                vec![(input_path, output_name, CONTENT_TYPE_JS.to_string())]
            }
            Type::Wasm { .. } => {
                let input_path_wasm = make_input_path(&source, "_bg", "wasm");
                let output_name_wasm = source.output_base_name.clone() + ".wasm";

                let input_path_js = make_input_path(&source, "", "js");
                let output_name_js = source.output_base_name.clone() + ".js";

                vec![
                    (input_path_wasm, output_name_wasm, CONTENT_TYPE_WASM.to_string()),
                    (input_path_js, output_name_js, CONTENT_TYPE_JS.to_string()),
                ]
            }
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Template for generating the static file info provider code.
#[derive(Template)]
#[template(path = "static_files.rs", escape = "none")]
struct StaticFilesTemplate<'a> {
    pub config: &'a Config,
    pub info: &'a Info,
    pub spec: &'a Spec,
    pub generator: GeneratorCallback,
}

impl<'a> StaticFilesTemplate<'a> {
    pub fn new(
        config: &'a Config,
        info: &'a Info,
        spec: &'a Spec,
        generator: GeneratorCallback,
    ) -> Self {
        Self { config, info, spec, generator }
    }
}

// -------------------------------------------------------------------------------------------------

pub fn generate(
    attributes: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let config = parse_attributes(attributes);
    let info = parse_input(input);
    let spec = read_template(&config);
    let gen = GeneratorCallback::new();

    StaticFilesTemplate::new(&config, &info, &spec, gen)
        .render()
        .expect("Render template")
        .parse()
        .expect("Parse template into a token stream")
}
