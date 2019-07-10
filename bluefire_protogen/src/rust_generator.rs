// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Generation of `Rust` API code.

use crate::buffer;
use crate::spec;
use crate::utils;

/// Generator of `Rust` API code.
pub struct RustGenerator {
    buffer: buffer::Buffer,
}

impl RustGenerator {
    /// Constructs a new `RustGenerator`.
    pub fn new() -> Self {
        Self { buffer: buffer::Buffer::new(0) }
    }

    /// Generate API.
    pub fn generate_api(mut self, api: &spec::Api) -> String {
        self.gen_imports();
        self.gen_types(&api.types);
        self.gen_paths(&api.routes);
        self.gen_reasons(&api.reasons);
        self.gen_methods(&api);
        self.get_content()
    }

    /// Generate path definitions.
    pub fn generate_paths(mut self, routes: &spec::Routes) -> String {
        self.gen_paths(&routes.routes);
        self.get_content()
    }

    /// Generate routes (`bluefire_backend::Route`).
    pub fn generate_routes(mut self, routes: &spec::Routes) -> String {
        self.gen_routes(&routes);
        self.get_content()
    }
}

// Public helper methods
impl RustGenerator {
    /// Formats given method as a `Method` from `Rust` `http` crate.
    pub fn format_method(method: &spec::HttpMethod) -> &'static str {
        match method {
            spec::HttpMethod::Get => "http::Method::GET",
            spec::HttpMethod::Post => "http::Method::POST",
            spec::HttpMethod::Put => "http::Method::PUT",
            spec::HttpMethod::Patch => "http::Method::PATCH",
            spec::HttpMethod::Delete => "http::Method::DELETE",
        }
    }

    /// Formats given type as a `Rust` type.
    pub fn format_simple_type(simple_type: &spec::SimpleType) -> &'static str {
        match simple_type {
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

// Private methods
impl RustGenerator {
    fn get_content(&mut self) -> String {
        self.buffer.get_content()
    }

    fn gen_imports(&mut self) {
        self.buffer.push_line("use serde_derive::{Serialize, Deserialize};");
        self.buffer.new_line();
    }

    // ---------------------------------------------------------------------------------------------
    // TYPES

    fn gen_types(&mut self, types: &Vec<spec::TypeDef>) {
        for type_iter in types.iter() {
            let name = utils::Name::new(&type_iter.name);
            match &type_iter.container {
                spec::TypeRepr::Simple { simple_type, validation } => {
                    self.gen_simple_type(&name, simple_type, validation);
                }
                spec::TypeRepr::Struct { members } => {
                    self.gen_struct_type(&name.camel_case(), members);
                }
                spec::TypeRepr::Union { members } => {
                    self.gen_union_type(&name.camel_case(), members);
                }
                spec::TypeRepr::Enum { values } => {
                    self.gen_enum_type(&name.camel_case(), values);
                }
            }
        }
    }

    fn gen_simple_type(
        &mut self,
        name: &utils::Name,
        simple_type: &spec::SimpleType,
        validation: &Option<spec::Validation>,
    ) {
        let type_name = Self::format_simple_type(&simple_type);
        let snake = name.snake_case();
        let camel = name.camel_case();

        // Type Definition
        self.buffer.push_line(&format!("pub type {} = {};", camel, type_name));
        self.buffer.new_line();

        // Validation
        if let Some(validation) = validation {
            // Validation result
            self.buffer.push_line(&format!("pub enum {}ValidationResult {{", camel));
            self.buffer.increase_indent();

            self.buffer.push_line("Ok,");
            for check in validation.checks.iter() {
                self.buffer.push_indent();
                self.buffer.push(&check.get_error_name().camel_case());
                self.buffer.push(",");
                self.buffer.new_line();
            }
            for condition in validation.conditions.iter() {
                self.buffer.push_indent();
                self.buffer.push(&condition.get_error_name().camel_case());
                self.buffer.push(",");
                self.buffer.new_line();
            }

            self.buffer.decrease_indent();
            self.buffer.push_line("}");
            self.buffer.new_line();

            // Validation function
            self.buffer.push_line(&format!("pub fn __validate_{}(item: &{})", snake, camel));
            self.buffer.push("-> bluefire_twine::ValidationResult<");
            self.buffer.push(&camel);
            self.buffer.push("ValidationResult> {");
            self.buffer.increase_indent();

            self.buffer.push("let mut validation_result = ");
            self.buffer.push("bluefire_twine::ValidationResult::new();");
            self.buffer.new_line();
            self.gen_conditions(&name, &simple_type, &validation.conditions);
            self.gen_checks(&name, &validation.checks);
            self.buffer.push_line("validation_result");

            self.buffer.decrease_indent();
            self.buffer.push_line("}");
            self.buffer.new_line();
        }
    }

    fn gen_checks(&mut self, name: &utils::Name, checks: &Vec<spec::Check>) {
        for check in checks.iter() {
            let name_camel = name.camel_case();
            let error = check.get_error_name().camel_case();
            match check {
                spec::Check::Email => {
                    self.buffer.push_multiline(&format!(
                        "
                        if !bluefire_twine::validation::validate_email(item) {{
                            validation_result.add(
                            {}ValidationResult::{});
                        }}",
                        name_camel, error
                    ));
                }
            }
        }
    }

    fn gen_conditions(
        &mut self,
        name: &utils::Name,
        simple_type: &spec::SimpleType,
        conditions: &Vec<spec::Condition>,
    ) {
        for condition in conditions.iter() {
            let name_camel = name.camel_case();
            let error = condition.get_error_name().camel_case();
            match condition {
                spec::Condition::Le(value) => match simple_type {
                    spec::SimpleType::U8 | spec::SimpleType::U32 | spec::SimpleType::I32 => {
                        self.buffer.push_line(&format!("if *item > {} {{", value));
                    }
                    spec::SimpleType::F32 | spec::SimpleType::F64 => {
                        self.buffer.push_line(&format!("if *item > {:.4} {{", value));
                    }
                    spec::SimpleType::Str | spec::SimpleType::Id => {
                        panic!("Type {:?} can't be compared");
                    }
                },
                spec::Condition::Ge(value) => match simple_type {
                    spec::SimpleType::U8 | spec::SimpleType::U32 | spec::SimpleType::I32 => {
                        self.buffer.push_line(&format!("if *item < {} {{", value));
                    }
                    spec::SimpleType::F32 | spec::SimpleType::F64 => {
                        self.buffer.push_line(&format!("if *item < {:.4} {{", value));
                    }
                    spec::SimpleType::Str | spec::SimpleType::Id => {
                        panic!("Type {:?} can't be compared", simple_type);
                    }
                },
                spec::Condition::LenEq(len) => match simple_type {
                    spec::SimpleType::Str => {
                        self.buffer.push_line(&format!("if item.len() != {} {{", len));
                    }
                    _ => {
                        panic!("Type {:?} does not have length");
                    }
                },
                spec::Condition::LenLe(len) => match simple_type {
                    spec::SimpleType::Str => {
                        self.buffer.push_line(&format!("if item.len() > {} {{", len));
                    }
                    _ => {
                        panic!("Type {:?} does not have length");
                    }
                },
                spec::Condition::LenGe(len) => match simple_type {
                    spec::SimpleType::Str => {
                        self.buffer.push_line(&format!("if item.len() < {} {{", len));
                    }
                    _ => {
                        panic!("Type {:?} does not have length");
                    }
                },
            }
            self.buffer.increase_indent();
            self.buffer.push("validation_result.add(");
            self.buffer.push(&format!("{}ValidationResult::{});", name_camel, error));
            self.buffer.decrease_indent();
            self.buffer.new_line();
            self.buffer.push_line("}");
        }
    }

    fn gen_struct_type(&mut self, name: &str, members: &Vec<spec::Member>) {
        self.push_derive_line();
        self.buffer.push_line(&format!("pub struct {} {{", name));
        self.gen_members(&members);
        self.buffer.push_line("}");
        self.buffer.new_line();

        self.buffer.push_line(&format!("impl {} {{", name));
        self.buffer.increase_indent();
        self.gen_constructor(members);
        self.buffer.decrease_indent();
        self.buffer.push_line("}");
        self.buffer.new_line();
    }

    fn gen_union_type(&mut self, name: &str, members: &Vec<spec::Member>) {
        self.push_derive_line();
        self.buffer.push_line("#[serde(tag = \"variant\", content = \"content\")]");
        self.buffer.push_line(&format!("pub enum {} {{", name));
        self.buffer.increase_indent();
        for member in members.iter() {
            let (name, formated_type) = Self::format_member(&member);
            self.buffer.push_line(&format!("#[serde(rename = \"{}\")]", name.snake_case()));
            self.buffer.push_line(&format!("{}({}),", name.camel_case(), formated_type));
        }
        self.buffer.decrease_indent();
        self.buffer.push_line("}");
        self.buffer.new_line();
    }

    fn gen_enum_type(&mut self, name: &str, values: &Vec<String>) {
        self.push_derive_line();
        self.buffer.push_line(&format!("pub enum {} {{", name));
        self.buffer.increase_indent();
        for value in values.iter() {
            let name = utils::Name::new(&value);
            self.buffer.push_line(&format!("#[serde(rename = \"{}\")]", name.snake_case()));
            self.buffer.push_line(&format!("{},", name.camel_case()));
        }
        self.buffer.decrease_indent();
        self.buffer.push_line("}");
        self.buffer.new_line();

        self.buffer.push_line(&format!("impl {} {{", name));
        self.buffer.increase_indent();
        self.buffer.push_line("pub fn to_str(&self) -> &'static str {");
        self.buffer.increase_indent();
        self.buffer.push_line("match &self {");
        self.buffer.increase_indent();
        for value in values.iter() {
            let value_name = utils::Name::new(&value);
            self.buffer.push_line(&format!(
                "{}::{} => \"{}\",",
                name,
                value_name.camel_case(),
                value_name.snake_case()
            ));
        }
        self.buffer.decrease_indent();
        self.buffer.push_line("}");
        self.buffer.decrease_indent();
        self.buffer.push_line("}");
        self.buffer.push_line("pub fn from_str(text: &str) -> Option<Self> {");
        self.buffer.increase_indent();
        self.buffer.push_line("match text {");
        self.buffer.increase_indent();
        for value in values.iter() {
            let value_name = utils::Name::new(&value);
            self.buffer.push_line(&format!(
                "\"{}\" => Some({}::{}),",
                value_name.snake_case(),
                name,
                value_name.camel_case()
            ));
        }
        self.buffer.push_line("_ => None,");
        self.buffer.decrease_indent();
        self.buffer.push_line("}");
        self.buffer.decrease_indent();
        self.buffer.push_line("}");
        self.buffer.decrease_indent();
        self.buffer.push_line("}");
        self.buffer.new_line();
    }

    // ---------------------------------------------------------------------------------------------
    // Paths

    fn gen_paths(&mut self, routes: &Vec<spec::Route>) {
        let paths = spec::routes_to_paths(routes);
        for path in paths.iter() {
            let name = utils::camel_case(&path.name) + "PathParams";
            self.gen_path_struct(&path, &name);
            self.gen_path_impl(&path, &name);
        }
    }

    fn gen_path_struct(&mut self, path: &spec::Path, name: &str) {
        self.push_derive_line();
        self.buffer.push_line(&format!("pub struct {} {{", name));
        self.buffer.increase_indent();
        for segment in path.segments.iter() {
            match segment {
                spec::Segment::Exact(..) => {
                    // nothing to generate
                }
                spec::Segment::Str(name) => {
                    self.buffer.push_line(&format!("pub {}: String,", utils::snake_case(name)));
                }
            }
        }
        self.buffer.decrease_indent();
        self.buffer.push_line("}");
        self.buffer.new_line();
    }

    fn gen_path_impl(&mut self, path: &spec::Path, name: &str) {
        self.buffer.push_line(&format!("impl {} {{", name));
        self.buffer.increase_indent();

        // Constructor
        self.buffer.push_line("pub fn new (");
        self.buffer.increase_indent();
        for segment in path.segments.iter() {
            match segment {
                spec::Segment::Exact(..) => {
                    // nothing to generate
                }
                spec::Segment::Str(name) => {
                    self.buffer.push_line(&format!("{}: String,", utils::snake_case(&name)));
                }
            }
        }
        self.buffer.decrease_indent();
        self.buffer.push_line(") -> Self {");
        self.buffer.increase_indent();
        self.buffer.push_line("Self {");
        self.buffer.increase_indent();
        for segment in path.segments.iter() {
            match segment {
                spec::Segment::Exact(..) => {
                    // nothing to generate
                }
                spec::Segment::Str(name) => {
                    self.buffer.push_line(&format!("{},", utils::snake_case(&name)));
                }
            }
        }
        self.buffer.decrease_indent();
        self.buffer.push_line("}");
        self.buffer.decrease_indent();
        self.buffer.push_line("}");
        self.buffer.new_line();

        // Path construction
        self.buffer.push_line("pub fn to_path(&self) -> String {");
        self.buffer.increase_indent();
        self.buffer.push_line("String::new()");
        self.buffer.increase_indent();
        for segment in path.segments.iter() {
            match segment {
                spec::Segment::Exact(name) => {
                    self.buffer.push_line(&format!("+ \"/{}\"", utils::snake_case(&name)));
                }
                spec::Segment::Str(name) => {
                    self.buffer.push_line(&format!("+ \"/\" + &self.{}", utils::snake_case(&name)));
                }
            }
        }
        self.buffer.decrease_indent();
        self.buffer.decrease_indent();
        self.buffer.push_line("}");
        self.buffer.new_line();

        // Associated path construction
        self.buffer.push_line("pub fn get (");
        self.buffer.increase_indent();
        for segment in path.segments.iter() {
            match segment {
                spec::Segment::Exact(..) => {
                    // nothing to generate
                }
                spec::Segment::Str(name) => {
                    self.buffer.push_line(&format!("{}: &str,", utils::snake_case(&name)));
                }
            }
        }
        self.buffer.decrease_indent();
        self.buffer.push_line(") -> String {");
        self.buffer.increase_indent();
        self.buffer.push_line("String::new()");
        self.buffer.increase_indent();
        for segment in path.segments.iter() {
            match segment {
                spec::Segment::Exact(name) => {
                    self.buffer.push_line(&format!("+ \"/{}\"", utils::snake_case(&name)));
                }
                spec::Segment::Str(name) => {
                    self.buffer.push_line(&format!("+ \"/\" + {}", utils::snake_case(&name)));
                }
            }
        }
        self.buffer.decrease_indent();
        self.buffer.decrease_indent();
        self.buffer.push_line("}");

        // Close impl
        self.buffer.decrease_indent();
        self.buffer.push_line("}");
        self.buffer.new_line();
    }

    // ---------------------------------------------------------------------------------------------

    fn gen_reasons(&mut self, reasons: &Vec<spec::Reason>) {
        for reason in reasons.iter() {
            let name = utils::camel_case(&reason.name);
            self.gen_reason_enum(name.clone(), &reason.cases);
        }
    }

    fn gen_methods(&mut self, proto: &spec::Api) {
        for method in proto.methods.iter() {
            let prefix = utils::camel_case(&method.name);
            let request_name = prefix.clone() + "Request";
            let response_name = prefix + "Response";

            self.gen_request_struct(&method.request, &request_name);
            self.gen_request_impl(&method.request, &request_name, &proto.types);
            self.gen_response_enum(&method.response, &response_name);
            self.gen_response_impl(&method.response, &response_name, &proto.reasons);
            self.gen_method(&method);
        }
    }

    fn gen_request_struct(&mut self, request: &spec::Request, name: &str) {
        self.push_derive_line();
        self.buffer.push_line(&format!("pub struct {} {{", name));
        self.buffer.increase_indent();
        for arg in request.args.iter() {
            let (name, formated_type) = Self::format_member(&arg);
            self.buffer.push_line(&format!("pub {}: {},", name.snake_case(), formated_type));
        }
        self.buffer.decrease_indent();
        self.buffer.push_line("}");
        self.buffer.new_line();
    }

    fn gen_response_enum(&mut self, response: &spec::Response, name: &str) {
        let failure_camel = utils::camel_case(&response.failure_reason) + "Reason";
        let error_camel = utils::camel_case(&response.error_reason) + "Reason";
        self.push_derive_line();
        self.buffer.push_line(&format!("pub enum {} {{", name));
        self.buffer.increase_indent();
        self.buffer.push_line("#[serde(rename = \"success\")]");
        self.buffer.push_line("Success {");
        self.buffer.increase_indent();
        for arg in response.args.iter() {
            let (name, formated_type) = Self::format_member(&arg);
            self.buffer.push_line(&format!("{}: {},", name.snake_case(), formated_type));
        }
        self.buffer.decrease_indent();
        self.buffer.push_line("},");
        self.buffer.push_line(&format!("#[serde(rename = \"failure\")]"));
        self.buffer.push_line(&format!("Failure({}),", failure_camel));
        self.buffer.push_line(&format!("#[serde(rename = \"error\")]"));
        self.buffer.push_line(&format!("Error({}),", error_camel));
        self.buffer.decrease_indent();
        self.buffer.push_line("}");
        self.buffer.new_line();
    }

    fn gen_request_impl(
        &mut self,
        request: &spec::Request,
        name: &str,
        types: &Vec<spec::TypeDef>,
    ) {
        self.buffer.push_line(&format!("impl {} {{", name));
        self.buffer.increase_indent();

        // Constructor
        self.gen_constructor(&request.args);

        // Request
        if request.method == spec::HttpMethod::Get {
            self.buffer.push_line("pub fn from_query_string(query_str: &str)");
            self.buffer.push_line("-> Result<Self, serde_urlencoded::de::Error> {");
            self.buffer.increase_indent();
            self.buffer.push_line("serde_urlencoded::from_str(query_str)");
            self.buffer.decrease_indent();
            self.buffer.push_line("}");

            self.buffer.new_line();

            self.buffer.push_line("pub fn to_query_string(&self)");
            self.buffer.push_line("-> Result<String, serde_urlencoded::ser::Error> {");
            self.buffer.increase_indent();
            self.buffer.push_line("serde_urlencoded::to_string(self)");
            self.buffer.decrease_indent();
            self.buffer.push_line("}");
        } else {
            self.buffer.push_line("pub fn from_json_string(json_str: &str)");
            self.buffer.push_line("-> Result<Self, serde_json::Error> {");
            self.buffer.increase_indent();
            self.buffer.push_line("serde_json::from_str(json_str)");
            self.buffer.decrease_indent();
            self.buffer.push_line("}");

            self.buffer.new_line();

            self.buffer.push_line("pub fn to_json_string(&self)");
            self.buffer.push_line("-> Result<String, serde_json::Error> {");
            self.buffer.increase_indent();
            self.buffer.push_line("serde_json::to_string(self)");
            self.buffer.decrease_indent();
            self.buffer.push_line("}");
        }

        // Get Method
        self.buffer.push_line("pub fn get_method(&self) -> http::method::Method {");
        self.buffer.increase_indent();
        self.buffer.push_line(Self::format_method(&request.method));
        self.buffer.decrease_indent();
        self.buffer.push_line("}");

        self.buffer.push_line("pub fn get_method_name(&self) -> &'static str {");
        self.buffer.increase_indent();
        self.buffer.push_line(&format!("\"{}\"", request.method.to_str()));
        self.buffer.decrease_indent();
        self.buffer.push_line("}");

        // Into Message
        let path_name = utils::camel_case(&request.path) + "PathParams";
        self.buffer.push_line(&format!("pub fn to_message(&self, params: &{})", path_name));
        self.buffer.push_line("-> bluefire_twine::Message {");
        self.buffer.increase_indent();
        self.buffer.push_line("bluefire_twine::Message::new(");
        self.buffer.increase_indent();
        self.buffer.push_line("self.get_method_name(),");
        self.buffer.push_line("params.to_path(),");
        if request.method == spec::HttpMethod::Get {
            self.buffer.push_line("self.to_query_string().expect(\"Cast to query string\"),");
            self.buffer.push_line("String::new()");
        } else {
            self.buffer.push_line("String::new(),");
            self.buffer.push_line("self.to_json_string().expect(\"Cast to json string\")");
        }
        self.buffer.decrease_indent();
        self.buffer.push_line(")");
        self.buffer.decrease_indent();
        self.buffer.push_line("}");

        // Validation
        for member in request.args.iter() {
            match &member {
                spec::Member::Simple { .. } => {
                    // nothing to do
                }
                spec::Member::Contained { .. } => {
                    // nothing to do
                }
                spec::Member::Defined { name, tipe } => {
                    if let Some(tipe) = spec::find_type(tipe, types) {
                        match &tipe.container {
                            spec::TypeRepr::Simple { validation, .. } => {
                                if validation.is_some() {
                                    self.gen_validation(name, &tipe.name);
                                }
                            }
                            spec::TypeRepr::Struct { .. } => {
                                // nothing to do
                            }
                            spec::TypeRepr::Union { .. } => {
                                // nothing to do
                            }
                            spec::TypeRepr::Enum { .. } => {
                                // nothing to do
                            }
                        }
                    }
                }
            }
        }

        // End impl
        self.buffer.decrease_indent();
        self.buffer.push_line("}");
    }

    fn gen_validation(&mut self, member_name: &str, type_name: &str) {
        let member_name = utils::snake_case(member_name);
        let type_name = utils::Name::new(type_name);
        let snake = type_name.snake_case();
        let camel = type_name.camel_case();
        self.buffer.push_multiline(&format!(
            "
            pub fn validate_{}(&self) -> bluefire_twine::ValidationResult<{}ValidationResult> {{
                __validate_{}(&self.{})
            }}",
            member_name, camel, snake, member_name
        ));
        self.buffer.new_line();
    }

    fn gen_response_impl(
        &mut self,
        response: &spec::Response,
        name: &str,
        reasons: &Vec<spec::Reason>,
    ) {
        self.gen_success_constructor(name, &response);

        let reason = spec::find_reason(&response.failure_reason, reasons);
        self.gen_reason_constructors("failure", name, &reason);

        let reason = spec::find_reason(&response.error_reason, reasons);
        self.gen_reason_constructors("error", name, &reason);
    }

    fn gen_reason_enum(&mut self, name: String, cases: &Vec<spec::Case>) {
        self.push_derive_line();
        self.buffer.push_line(&format!("pub enum {}Reason {{", name));
        self.buffer.increase_indent();
        for case in cases.iter() {
            let name = utils::Name::new(&case.name);
            let (snake, camel) = (name.snake_case(), name.camel_case());
            self.buffer.push_line(&format!("#[serde(rename = \"{}\")]", snake));
            if case.args.len() > 0 {
                self.buffer.push_line(&format!("{} {{", camel));
                self.gen_args(&case.args);
                self.buffer.push_line("},");
            } else {
                self.buffer.push_line(&format!("{},", camel));
            }
        }
        self.buffer.decrease_indent();
        self.buffer.push_line("}");
        self.buffer.new_line();
    }

    fn gen_constructor(&mut self, members: &Vec<spec::Member>) {
        self.buffer.push_line("pub fn new (");
        self.buffer.increase_indent();
        for member in members.iter() {
            let (name, formated_type) = Self::format_member(&member);
            self.buffer.push_line(&format!("{}: {},", name.snake_case(), formated_type));
        }
        self.buffer.decrease_indent();
        self.buffer.push_line(") -> Self {");
        self.buffer.increase_indent();
        self.buffer.push_line("Self {");
        self.buffer.increase_indent();
        for member in members.iter() {
            let (name, _) = Self::format_member(&member);
            self.buffer.push_line(&format!("{},", name.snake_case()));
        }
        self.buffer.decrease_indent();
        self.buffer.push_line("}");
        self.buffer.decrease_indent();
        self.buffer.push_line("}");
    }

    fn gen_success_constructor(&mut self, name: &str, response: &spec::Response) {
        self.buffer.push_line(&format!("impl {} {{", name));
        self.buffer.increase_indent();

        self.buffer.push_line(&format!("pub fn success ("));
        self.buffer.increase_indent();
        for arg in response.args.iter() {
            let (name, formated_type) = Self::format_member(&arg);
            self.buffer.push_line(&format!("{}: {},", name.snake_case(), formated_type));
        }
        self.buffer.decrease_indent();
        self.buffer.push_line(&format!(") -> (http::StatusCode, {}) {{", name));
        self.buffer.increase_indent();

        let formatted_code = Self::format_code(&response.success_code);
        self.buffer.push_line(&format!("({}, {}::Success {{", formatted_code, name));
        self.buffer.increase_indent();
        for arg in response.args.iter() {
            let (name, _) = Self::format_member(&arg);
            self.buffer.push_line(&format!("{},", name.snake_case()));
        }
        self.buffer.decrease_indent();
        self.buffer.push_line("})");

        self.buffer.decrease_indent();
        self.buffer.push_line("}");
        self.buffer.new_line();

        self.buffer.decrease_indent();
        self.buffer.push_line("}");
        self.buffer.new_line();
    }

    fn gen_reason_constructors(&mut self, prefix: &str, query_name: &str, reason: &spec::Reason) {
        let reason_name = utils::camel_case(&reason.name) + "Reason";
        self.buffer.push_line(&format!("impl {} {{", query_name));
        self.buffer.increase_indent();
        for case in reason.cases.iter() {
            let name = utils::Name::new(&case.name);
            let (snake, camel) = (name.snake_case(), name.camel_case());

            self.buffer.push_indent();
            self.buffer.push(&format!("pub fn {}_{}(", prefix, snake));
            if case.args.len() != 0 {
                self.buffer.new_line();
                self.gen_args(&case.args);
                self.buffer.push_indent();
            }
            self.buffer.push(&format!(") -> (http::StatusCode, {}) {{", query_name));
            self.buffer.new_line();
            self.buffer.increase_indent();

            self.buffer.push_indent();
            let code = Self::format_code(&case.code);
            self.buffer.push(&format!(
                "({}, {}::{}({}::{}{{",
                code,
                query_name,
                utils::capitalize(prefix),
                reason_name,
                camel
            ));
            self.gen_args_call(&case.args);
            self.buffer.push("}))");

            self.buffer.decrease_indent();
            self.buffer.new_line();
            self.buffer.push_line("}");
            self.buffer.new_line();
        }

        self.buffer.decrease_indent();
        self.buffer.push_line("}");
        self.buffer.new_line();
    }

    fn gen_members(&mut self, members: &Vec<spec::Member>) {
        self.buffer.increase_indent();
        for member in members.iter() {
            let (name, formated_type) = Self::format_member(&member);
            self.buffer.push_line(&format!("pub {}: {},", name.snake_case(), formated_type));
        }
        self.buffer.decrease_indent();
    }

    fn gen_args(&mut self, args: &Vec<spec::Member>) {
        self.buffer.increase_indent();
        for arg in args.iter() {
            let (name, formated_type) = Self::format_member(&arg);
            self.buffer.push_line(&format!("{}: {},", name.snake_case(), formated_type));
        }
        self.buffer.decrease_indent();
    }

    fn gen_args_call(&mut self, args: &Vec<spec::Member>) {
        for (i, arg) in args.iter().enumerate() {
            let (name, _) = Self::format_member(&arg);
            self.buffer.push(&name.snake_case());
            if i != (args.len() - 1) {
                self.buffer.push(", ");
            }
        }
    }

    fn gen_method(&mut self, method: &spec::Method) {
        let prefix = utils::camel_case(&method.name);
        let method_name = prefix.clone() + "Method";
        let request_name = prefix.clone() + "Request";
        let response_name = prefix + "Response";
        let path_name = utils::camel_case(&method.request.path) + "PathParams";

        self.buffer.push_line(&format!("pub struct {};", method_name));
        self.buffer.push_line(&format!("impl bluefire_twine::Method for {} {{", method_name));
        self.buffer.increase_indent();
        self.buffer.push_line(&format!("type PathParams = {};", path_name));
        self.buffer.push_line(&format!("type Request = {};", request_name));
        self.buffer.push_line(&format!("type Response = {};", response_name));
        self.buffer.decrease_indent();
        self.buffer.push_line("}");
        self.buffer.new_line();
    }

    fn push_derive_line(&mut self) {
        self.buffer.push_line("#[derive(Clone, Debug, Serialize, Deserialize)]");
    }

    // ---------------------------------------------------------------------------------------------
    // Routes

    fn gen_routes(&mut self, routes: &spec::Routes) {
        self.buffer.push("bluefire_backend::router::Route::index()");
        self.buffer.new_line();
        self.buffer.increase_indent();
        if let Some(name) = &routes.name {
            let view_name = utils::camel_case(&name) + "View";
            self.buffer.push_line(&format!(".with_view(Box::new({}))", &view_name));
        }
        self.gen_segments(&routes.routes);
        self.buffer.decrease_indent();
    }

    fn gen_segments(&mut self, routes: &Vec<spec::Route>) {
        if !routes.is_empty() {
            self.buffer.push_line(".with_routes(vec![");
            self.buffer.increase_indent();
            for route in routes.iter() {
                self.buffer.push_indent();
                self.buffer.push("bluefire_backend::router::Route::");
                self.buffer.push(&match &route.segment {
                    spec::Segment::Exact(name) => {
                        ["exact(\"", &utils::snake_case(&name), "\")"].concat()
                    }
                    spec::Segment::Str(name) => {
                        ["param(\"", &utils::snake_case(&name), "\")"].concat()
                    }
                });
                self.buffer.new_line();
                self.buffer.increase_indent();
                if let Some(name) = &route.name {
                    let view_name = utils::camel_case(&name) + "View";
                    self.buffer.push_line(&[".with_view(Box::new(", &view_name, "))"].concat());
                }
                self.gen_segments(&route.routes);
                self.buffer.push_line(",");
                self.buffer.decrease_indent();
            }
            self.buffer.decrease_indent();
            self.buffer.push_line("])");
        }
    }

    // ---------------------------------------------------------------------------------------------
    // Spec elements formating

    fn format_member(member: &spec::Member) -> (utils::Name, String) {
        match member {
            spec::Member::Simple { name, tipe } => {
                let member_name = utils::Name::new(name);
                let formated_type = Self::format_simple_type(&tipe).to_owned();
                (member_name, formated_type)
            }
            spec::Member::Defined { name, tipe, .. } => {
                let member_name = utils::Name::new(name);
                let formated_type = utils::camel_case(&tipe);
                (member_name, formated_type)
            }
            spec::Member::Contained { name, tipe, container } => {
                let member_name = utils::Name::new(name);
                let formated_type = utils::camel_case(&tipe);
                let formated_type = match container {
                    spec::ContainerType::Vector => format!("Vec<{}>", formated_type),
                    spec::ContainerType::Optional => format!("Option<{}>", formated_type),
                };
                (member_name, formated_type)
            }
        }
    }

    fn format_code(code: &spec::HttpResponse) -> &'static str {
        match code {
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
