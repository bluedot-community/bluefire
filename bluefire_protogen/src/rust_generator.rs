// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Generation of `Rust` API code.

use askama::Template;

use crate::{spec, utils};

// -------------------------------------------------------------------------------------------------

impl spec::HttpResponse {
    /// Formats given response code as a `StatusCode` from `Rust` `http` crate.
    fn rust_format(&self) -> &'static str {
        match self {
            spec::HttpResponse::Ok => "http::StatusCode::OK",
            spec::HttpResponse::Created => "http::StatusCode::CREATED",
            spec::HttpResponse::BadRequest => "http::StatusCode::BAD_REQUEST",
            spec::HttpResponse::Unauthorized => "http::StatusCode::UNAUTHORIZED",
            spec::HttpResponse::Forbidden => "http::StatusCode::FORBIDDEN",
            spec::HttpResponse::NotFound => "http::StatusCode::NOT_FOUND",
            spec::HttpResponse::Conflict => "http::StatusCode::CONFLICT",
            spec::HttpResponse::InternalServerError => "http::StatusCode::INTERNAL_SERVER_ERROR",
        }
    }
}

impl spec::HttpMethod {
    /// Formats given method as a `Method` from `Rust` `http` crate.
    fn rust_format(&self) -> &'static str {
        match self {
            spec::HttpMethod::Get => "http::Method::GET",
            spec::HttpMethod::Post => "http::Method::POST",
            spec::HttpMethod::Put => "http::Method::PUT",
            spec::HttpMethod::Patch => "http::Method::PATCH",
            spec::HttpMethod::Delete => "http::Method::DELETE",
        }
    }
}

impl spec::SimpleType {
    /// Formats given type as a `Rust` type.
    fn rust_format(&self) -> &'static str {
        match self {
            spec::SimpleType::U8 => "u8",
            spec::SimpleType::U32 => "u32",
            spec::SimpleType::I32 => "i32",
            spec::SimpleType::F32 => "f32",
            spec::SimpleType::F64 => "f64",
            spec::SimpleType::Str => "String",
            spec::SimpleType::Id => "bluefire_twine::Id",
        }
    }
}

impl spec::Member {
    fn name(&self) -> &utils::Name {
        &self.name
    }

    fn rust_type(&self) -> String {
        let raw_type = match &self.tipe {
            spec::MemberType::Simple(tipe) => tipe.rust_format().to_string(),
            spec::MemberType::Defined(name) => name.camel_case(),
        };

        if let Some(container) = &self.container {
            match container {
                spec::ContainerType::Vector => format!("Vec<{}>", raw_type),
                spec::ContainerType::Optional => format!("Option<{}>", raw_type),
            }
        } else {
            raw_type
        }
    }
}

// -------------------------------------------------------------------------------------------------

/// Template for type imports.
#[derive(Template)]
#[template(path = "imports.rs", escape = "none")]
struct RustImportsTemplate;

impl RustImportsTemplate {
    pub fn new() -> Self {
        Self
    }
}

/// Template for generating types.
#[derive(Template)]
#[template(path = "types.rs", escape = "none")]
struct RustTypesTemplate<'a> {
    pub api: &'a spec::Api,
}

impl<'a> RustTypesTemplate<'a> {
    pub fn new(api: &'a spec::Api) -> Self {
        Self { api }
    }
}

/// Template for generating paths.
#[derive(Template)]
#[template(path = "paths.rs", escape = "none")]
struct RustPathsTemplate<'a> {
    pub paths: &'a Vec<spec::Path>,
}

impl<'a> RustPathsTemplate<'a> {
    pub fn new(paths: &'a Vec<spec::Path>) -> Self {
        Self { paths }
    }
}

/// Template for generating yields.
#[derive(Template)]
#[template(path = "yields.rs", escape = "none")]
struct RustYieldsTemplate<'a> {
    pub api: &'a spec::Api,
}

impl<'a> RustYieldsTemplate<'a> {
    pub fn new(api: &'a spec::Api) -> Self {
        Self { api }
    }
}

/// Template for generating failure and error reasons.
#[derive(Template)]
#[template(path = "reasons.rs", escape = "none")]
struct RustReasonsTemplate<'a> {
    pub api: &'a spec::Api,
}

impl<'a> RustReasonsTemplate<'a> {
    pub fn new(api: &'a spec::Api) -> Self {
        Self { api }
    }
}

/// Template for generating requests and responses.
#[derive(Template)]
#[template(path = "methods.rs", escape = "none")]
struct RustMethodsTemplate<'a> {
    pub api: &'a spec::Api,
    pub generator: GeneratorCallback,
}

impl<'a> RustMethodsTemplate<'a> {
    pub fn new(api: &'a spec::Api, generator: GeneratorCallback) -> Self {
        Self { api, generator }
    }
}

/// Template for generating route (called recursively).
#[derive(Template)]
#[template(path = "route.rs", escape = "none")]
struct RustRouteTemplate<'a> {
    pub route: &'a spec::Route,
    pub generator: GeneratorCallback,
}

impl<'a> RustRouteTemplate<'a> {
    pub fn new(route: &'a spec::Route, generator: GeneratorCallback) -> Self {
        Self { route, generator }
    }
}

/// Template for generating routes (uses `RustRouteTemplate`).
#[derive(Template)]
#[template(path = "routes.rs", escape = "none")]
struct RustRoutesTemplate<'a> {
    pub routes: &'a spec::Routes,
    pub generator: GeneratorCallback,
}

impl<'a> RustRoutesTemplate<'a> {
    pub fn new(routes: &'a spec::Routes, generator: GeneratorCallback) -> Self {
        Self { routes, generator }
    }
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

    /// Renders the route template with the give route.
    pub fn route(&self, route: &spec::Route) -> String {
        RustRouteTemplate::new(route, self.clone()).render().expect("Render route template")
    }

    /// Searches for a `TypeDef` with given name.
    pub fn find_type(&self, name: &utils::Name, types: &Vec<spec::TypeDef>) -> spec::TypeDef {
        spec::find_type(name, types)
    }

    /// Searches for a `Yield` with given name.
    pub fn find_yield(&self, name: utils::Name, yields: &Vec<spec::Yield>) -> spec::Yield {
        spec::find_yield(&name, yields)
    }

    /// Searches for a `Reason` with given name.
    pub fn find_reason(&self, name: utils::Name, reasons: &Vec<spec::Reason>) -> spec::Reason {
        spec::find_reason(&name, reasons)
    }
}

// -------------------------------------------------------------------------------------------------

/// Generator of `Rust` API code.
pub struct RustGenerator;

impl RustGenerator {
    /// Constructs a new `RustGenerator`.
    pub fn new() -> Self {
        Self
    }

    /// Generate API.
    pub fn generate_api(self, api: &spec::Api) -> String {
        let paths = spec::routes_to_paths(&api.routes);
        let imports_template = RustImportsTemplate::new();
        let types_template = RustTypesTemplate::new(&api);
        let paths_template = RustPathsTemplate::new(&paths);
        let yields_template = RustYieldsTemplate::new(&api);
        let reasons_template = RustReasonsTemplate::new(&api);
        let methods_template = RustMethodsTemplate::new(&api, GeneratorCallback::new());

        let buffer = [
            imports_template.render().expect("Render imports template"),
            types_template.render().expect("Render types template"),
            paths_template.render().expect("Render paths template"),
            yields_template.render().expect("Render yields template"),
            reasons_template.render().expect("Render reasons template"),
            methods_template.render().expect("Render methods template"),
        ];

        buffer.concat()
    }

    /// Generate path definitions.
    pub fn generate_paths(self, routes: &spec::Routes) -> String {
        let paths = spec::routes_to_paths(&routes.routes);
        let imports_template = RustImportsTemplate::new();
        let paths_template = RustPathsTemplate::new(&paths);

        let buffer = [
            imports_template.render().expect("Render imports template"),
            paths_template.render().expect("Render paths template"),
        ];

        buffer.concat()
    }

    /// Generate routes (`bluefire_backend::Route`).
    pub fn generate_routes(self, routes: &spec::Routes) -> String {
        RustRoutesTemplate::new(routes, GeneratorCallback::new())
            .render()
            .expect("Render routes template")
    }

    /// Generate API from given input file and save to the given output file.
    pub fn generate_api_file(self, input: &str, output: &str) {
        let content = Self::read_manifest_path(input);
        let api = match spec::Api::from_str(&content) {
            Ok(api) => api,
            Err(err) => panic!("Parse file ({}): {}", input, err),
        };
        let result = self.generate_api(&api);
        Self::write_output_file(output, &result);
        println!("cargo:rerun-if-changed={}", input);
    }

    /// Generate paths from given input file and save to the given output file.
    pub fn generate_paths_file(self, input: &str, output: &str) {
        let content = Self::read_manifest_path(input);
        let paths = match spec::Routes::from_str(&content) {
            Ok(api) => api,
            Err(err) => panic!("Parse file ({}): {}", input, err),
        };
        let result = self.generate_paths(&paths);
        Self::write_output_file(output, &result);
        println!("cargo:rerun-if-changed={}", input);
    }

    /// Generate routes from given input file and save to the given output file.
    pub fn generate_routes_file(self, input: &str, output: &str) {
        let content = Self::read_manifest_path(input);
        let routes = match spec::Routes::from_str(&content) {
            Ok(api) => api,
            Err(err) => panic!("Parse file ({}): {}", input, err),
        };
        let result = self.generate_routes(&routes);
        Self::write_output_file(output, &result);
        println!("cargo:rerun-if-changed={}", input);
    }
}

impl RustGenerator {
    /// Reads a file from the cargo manifest path.
    pub fn read_manifest_path(input: &str) -> String {
        let input_dir =
            std::env::var("CARGO_MANIFEST_DIR").expect("Cargo manifest directory not provided");

        let mut input_path = std::path::PathBuf::new();
        input_path.push(&input_dir);
        input_path.push(input);

        std::fs::read_to_string(input_path.clone()).expect(&format!("Read file: {:?}", input_path))
    }

    /// Writes to a file in output directory.
    pub fn write_output_file(output: &str, content: &str) {
        use std::io::Write;

        let output_dir = std::env::var("OUT_DIR").expect("Read OUT_DIR variable");

        let mut output_path = std::path::PathBuf::new();
        output_path.push(&output_dir);
        output_path.push(output);

        let mut file =
            std::fs::File::create(&output_path).expect(&format!("Create file: {:?}", &output_path));
        file.write_all(content.as_bytes()).expect(&format!("Write to file: {:?}", &output_path));

        #[cfg(feature = "fmt")]
        {
            let out = &mut &mut std::io::stdout();
            let config = rustfmt_nightly::Config::default();
            let mut session = rustfmt_nightly::Session::new(config, Some(out));
            session.format(rustfmt_nightly::Input::File(output_path)).expect("Format with rustfmt");
        }
    }
}
