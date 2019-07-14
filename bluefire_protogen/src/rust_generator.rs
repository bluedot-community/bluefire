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
        match self {
            spec::Member::Simple { name, .. } => name,
            spec::Member::Defined { name, .. } => name,
            spec::Member::Contained { name, .. } => name,
        }
    }

    fn rust_type(&self) -> String {
        match self {
            spec::Member::Simple { tipe, .. } => tipe.rust_format().to_string(),
            spec::Member::Defined { tipe, .. } => tipe.camel_case(),
            spec::Member::Contained { tipe, container, .. } => match container {
                spec::ContainerType::Vector => format!("Vec<{}>", tipe.camel_case()),
                spec::ContainerType::Optional => format!("Option<{}>", tipe.camel_case()),
            },
        }
    }
}

// -------------------------------------------------------------------------------------------------

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
        let types_template = RustTypesTemplate::new(&api);
        let paths_template = RustPathsTemplate::new(&paths);
        let yields_template = RustYieldsTemplate::new(&api);
        let reasons_template = RustReasonsTemplate::new(&api);
        let methods_template = RustMethodsTemplate::new(&api, GeneratorCallback::new());

        let buffer = [
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
        RustPathsTemplate::new(&paths).render().expect("Render paths template")
    }

    /// Generate routes (`bluefire_backend::Route`).
    pub fn generate_routes(self, routes: &spec::Routes) -> String {
        RustRoutesTemplate::new(&routes, GeneratorCallback::new())
            .render()
            .expect("Render routes template")
    }

    /// Generate API from given input file and save to the given output file.
    pub fn generate_api_file(self, input: &str, output: &str) {
        use std::io::Write;

        let output_dir = std::env::var("OUT_DIR").expect("Read OUT_DIR variable");
        let input_dir =
            std::env::var("CARGO_MANIFEST_DIR").expect("Cargo manifest directory not provided");

        let mut input_path = std::path::PathBuf::new();
        input_path.push(&input_dir);
        input_path.push(input);

        let mut output_path = std::path::PathBuf::new();
        output_path.push(&output_dir);
        output_path.push(output);

        let api_str = std::fs::read_to_string(input_path.clone())
            .expect(&format!("Read file: {:?}", input_path));
        let api =
            spec::Api::from_str(&api_str).expect(&format!("Parse {:?} spec file", input_path));

        let result = self.generate_api(&api);

        let mut file =
            std::fs::File::create(&output_path).expect(&format!("Create file: {:?}", &output_path));
        file.write_all(result.as_bytes()).expect(&format!("Write to file: {:?}", &output_path));

        #[cfg(feature = "fmt")]
        {
            let out = &mut &mut std::io::stdout();
            let config = rustfmt_nightly::Config::default();
            let mut session = rustfmt_nightly::Session::new(config, Some(out));
            session.format(rustfmt_nightly::Input::File(output_path)).expect("Format with rustfmt");
        }
    }
}
