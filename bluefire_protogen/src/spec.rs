// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Specification of the API file format.

use serde_derive::{Deserialize, Serialize};
use serde_yaml;

use crate::utils;

// -------------------------------------------------------------------------------------------------
// Common definitions

/// Represents an HTTP method.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum HttpMethod {
    /// `GET` method.
    #[serde(rename = "get")]
    Get,

    /// `POST` method.
    #[serde(rename = "post")]
    Post,

    /// `PUT` method.
    #[serde(rename = "put")]
    Put,

    /// `PATCH` method.
    #[serde(rename = "patch")]
    Patch,

    /// `DELETE` method.
    #[serde(rename = "delete")]
    Delete,
}

/// Represents an HTTP response code.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum HttpResponse {
    /// Code: 200 Ok.
    #[serde(rename = "200-ok")]
    Ok,

    /// Code: 201 Created.
    #[serde(rename = "201-created")]
    Created,

    /// Code: 400 Bad Request.
    #[serde(rename = "400-bad-request")]
    BadRequest,

    /// Code: 401 Unauthorized.
    #[serde(rename = "401-unauthorized")]
    Unauthorized,

    /// Code: 403 Forbidden.
    #[serde(rename = "403-forbidden")]
    Forbidden,

    /// Code: 404 Not Found.
    #[serde(rename = "404-not-found")]
    NotFound,

    /// Code: 409 Conflict.
    #[serde(rename = "409-conflict")]
    Conflict,

    /// Code: 500 Internal Server Error.
    #[serde(rename = "500-internal-server-error")]
    InternalServerError,
}

// -------------------------------------------------------------------------------------------------
// Validation

/// Represents as predefined check if a value is correct.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Check {
    /// A string value should be a valid e-mail.
    #[serde(rename = "email")]
    Email,
}

/// Represents a parametrized condition to validate a value.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Condition {
    /// The value is a number and must be lesser or equal to this one.
    #[serde(rename = "le")]
    Le(f32),

    /// The value is a number and must be greater or equal to this one.
    #[serde(rename = "ge")]
    Ge(f32),

    /// The value is a string or vector and its length must be exactly equal to this one.
    #[serde(rename = "len_eq")]
    LenEq(u32),

    /// The value is a string or vector and its length must be lesser or equal to this one.
    #[serde(rename = "len_le")]
    LenLe(u32),

    /// The value is a string or vector and its length must be greater or equal to this one.
    #[serde(rename = "len_ge")]
    LenGe(u32),
}

/// Represents a list of checks and conditions to verify if given value is valid.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Validation {
    /// A list of `Check`s.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub checks: Vec<Check>,

    /// A list of `Condition`s.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub conditions: Vec<Condition>,
}

// -------------------------------------------------------------------------------------------------
// Types

/// Represents a simple type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum SimpleType {
    /// A one-byte unsigned integer.
    #[serde(rename = "u8")]
    U8,

    /// A four-byte unsigned integer.
    #[serde(rename = "u32")]
    U32,

    /// A four-byte signed integer.
    #[serde(rename = "i32")]
    I32,

    /// A four-byte floating-point number.
    #[serde(rename = "f32")]
    F32,

    /// An eight-byte floating-point number.
    #[serde(rename = "f64")]
    F64,

    /// A string.
    #[serde(rename = "string")]
    Str,

    /// An object ID.
    #[serde(rename = "id")]
    Id,
}

/// Represents a alternative way of how to modify the members type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ContainerType {
    /// Store as a vector.
    #[serde(rename = "vector")]
    Vector,

    /// Serialize/deserialize optionally.
    #[serde(rename = "optional")]
    Optional,
}

/// Represents and argument of request or return value of response.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Member {
    /// Simple (predefined) member.
    Simple {
        /// Member name.
        name: String,

        /// Members type.
        #[serde(rename = "type")]
        tipe: SimpleType,
    },

    /// A members inside of a predefined container.
    Contained {
        /// Members name.
        name: String,

        /// Members type.
        #[serde(rename = "type")]
        tipe: String,

        /// Members container.
        container: ContainerType,
    },

    /// A member type defined in API specification.
    Defined {
        /// Members name.
        name: String,

        /// Members type (this type must be defined in the API specification).
        #[serde(rename = "type")]
        tipe: String,
    },
}

/// Defines how a type should be represented in the API protocol (JSON).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "repr")]
pub enum TypeRepr {
    /// A simple type.
    #[serde(rename = "simple")]
    Simple {
        /// The type.
        #[serde(rename = "type")]
        simple_type: SimpleType,

        /// Validation.
        #[serde(default)]
        validation: Option<Validation>,
    },

    /// A structure with members.
    #[serde(rename = "struct")]
    Struct {
        /// A list of members.
        members: Vec<Member>,
    },

    /// A union (only one member preset at a time).
    #[serde(rename = "union")]
    Union {
        /// A list of possible members.
        members: Vec<Member>,
    },

    /// Enum (one of predefined string values).
    #[serde(rename = "enum")]
    Enum {
        /// A list of values.
        values: Vec<String>,
    },
}

/// Represents a definition of type.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TypeDef {
    /// Name of the new type.
    pub name: String,

    /// The way the type should be represented.
    pub container: TypeRepr,
}

// -------------------------------------------------------------------------------------------------
// Yields

/// Represents a successful result.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Yield {
    /// Name of the yield.
    pub name: String,

    /// HTTP code used in this response.
    pub code: HttpResponse,

    /// Values sent in response.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub args: Vec<Member>,
}

// -------------------------------------------------------------------------------------------------
// Reasons

/// Represents a reason of failure or error.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Reason {
    /// Name of the failure. Used to generate the representation name and to specify the reason in
    /// the `Method`.
    pub name: String,

    /// List of possible cases.
    pub cases: Vec<Case>,
}

/// Represents a possible case of failure reason.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Case {
    /// Name of the case. Used to generate the name of the constructor.
    pub name: String,

    /// HTTP code used in this response.
    pub code: HttpResponse,

    /// Values sent in response.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub args: Vec<Member>,
}

// -------------------------------------------------------------------------------------------------
// Paths

/// Represents a segment of a path.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Segment {
    /// Only strings exactly equal this one will match with this segment.
    #[serde(rename = "exact")]
    Exact(String),

    /// Any string will match with this segment. (Can be used to parametrize the path.)
    #[serde(rename = "string")]
    Str(String),
}

/// Represents a route (in a tree-like manner).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Route {
    /// Name of the path represented by this `Route` node.
    /// Used to generate a viewer struct name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The type of segment.
    #[serde(flatten)]
    pub segment: Segment,

    /// A list of sub-routes.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub routes: Vec<Route>,
}

// -------------------------------------------------------------------------------------------------
// Methods

/// Represents a request in an API call.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Request {
    /// HTTP method.
    pub method: HttpMethod,

    /// Path part of the URL.
    pub path: String,

    /// Arguments of the call (serialized either to JSON or query part of the URL).
    pub args: Vec<Member>,
}

/// Represents a response to an API call.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    /// A name of yield type used in case of success.
    pub success: String,

    /// A name of reason type used in case of failure.
    pub failure: String,

    /// A name of reason type used in case of error.
    pub error: String,
}

/// Represents an API call method.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Method {
    /// Name of the method. Used to generate the request and response representation names.
    pub name: String,

    /// The definition of a request.
    pub request: Request,

    /// The definition of a response.
    pub response: Response,
}

// -------------------------------------------------------------------------------------------------
// Protocol

/// Represents a top-level (index) route.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Routes {
    /// Name of the path represented by this `Routes` node.
    /// Used to generate a viewer struct name.
    pub name: Option<String>,

    /// A list of sub-routes.
    pub routes: Vec<Route>,
}

/// Represents a definition of an API.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Api {
    /// List of definitions of data structures used in the API.
    pub types: Vec<TypeDef>,

    /// A list of tree-like structures representing API routes.
    pub routes: Vec<Route>,

    /// A list of possible success results.
    pub yields: Vec<Yield>,

    /// A list of possible failure reasons.
    pub reasons: Vec<Reason>,

    /// List of possible API calls (request and corresponding paths and responses).
    pub methods: Vec<Method>,
}

// -------------------------------------------------------------------------------------------------
// Helper implementations

impl HttpMethod {
    /// Returns a name of the HTTP method.
    pub fn to_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Delete => "DELETE",
        }
    }
}

impl Check {
    /// Returns a name of the checks error.
    pub fn get_error_name(&self) -> utils::Name {
        match self {
            Check::Email => utils::Name::from_parts(vec!["email"]),
        }
    }
}

impl Condition {
    /// Returns a name of the conditions error.
    pub fn get_error_name(&self) -> utils::Name {
        match self {
            Condition::Le(..) => utils::Name::from_parts(vec!["too", "big"]),
            Condition::Ge(..) => utils::Name::from_parts(vec!["too", "small"]),
            Condition::LenEq(..) => utils::Name::from_parts(vec!["wrong", "length"]),
            Condition::LenLe(..) => utils::Name::from_parts(vec!["too", "long"]),
            Condition::LenGe(..) => utils::Name::from_parts(vec!["too", "short"]),
        }
    }
}

/// A helper method for representing routes as vectors of segments.
pub struct Path {
    /// The name of the path.
    pub name: String,

    /// A list of segments constituting the path.
    pub segments: Vec<Segment>,
}

impl Routes {
    /// Constructs `Routes` structure from API file content.
    pub fn from_str(spec_str: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str::<Self>(spec_str)
    }
}

impl Api {
    /// Constructs `Api` structure from API file content.
    pub fn from_str(spec_str: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str::<Self>(spec_str)
    }
}

// -------------------------------------------------------------------------------------------------
// Helper functions

/// Searches for a `Reason` with given name.
pub fn find_reason(name: &str, reasons: &Vec<Reason>) -> Reason {
    for reason in reasons.iter() {
        if name == reason.name {
            return reason.clone();
        }
    }

    panic!("No reason '{}' found", name);
}

/// Searches for a `Yield` with given name.
pub fn find_yield(name: &str, yields: &Vec<Yield>) -> Yield {
    for yeeld in yields.iter() {
        if name == yeeld.name {
            return yeeld.clone();
        }
    }

    panic!("No yield '{}' found", name);
}

/// Searches for a `TypeDef` with given name.
pub fn find_type(name: &str, types: &Vec<TypeDef>) -> Option<TypeDef> {
    for tipe in types.iter() {
        if name == tipe.name {
            return Some(tipe.clone());
        }
    }
    None
}

/// Transforms routes representation from tree-like structure to a vector of vectors of path
/// segments.
pub fn routes_to_paths(routes: &Vec<Route>) -> Vec<Path> {
    fn iter_segments(routes: &Vec<Route>, paths: &mut Vec<Path>, segments: &mut Vec<Segment>) {
        for route in routes.iter() {
            segments.push(route.segment.clone());
            if let Some(name) = &route.name {
                paths.push(Path { name: name.clone(), segments: segments.clone() });
            }
            iter_segments(&route.routes, paths, segments);
            segments.pop();
        }
    }

    let mut paths = Vec::new();
    let mut segments = Vec::new();
    iter_segments(&routes, &mut paths, &mut segments);
    paths
}

// -------------------------------------------------------------------------------------------------
// Tests

#[cfg(test)]
mod tests {
    use super::{ContainerType, Member, Route, Segment, SimpleType, TypeDef, TypeRepr};
    use serde_yaml;

    #[test]
    fn test_typedef_serialization() {
        let member1 = Member::Defined { name: "abcd".to_owned(), tipe: "custom".to_owned() };
        let member2 = Member::Defined { name: "edfg".to_owned(), tipe: "custom".to_owned() };
        let members = vec![member1, member2];
        assert_eq!(
            serde_yaml::to_string(&TypeDef {
                name: "name".to_string(),
                container: TypeRepr::Struct { members: members.clone() }
            })
            .unwrap(),
            "---\nname: name\ncontainer:\n  repr: struct\n  members:\n    - name: abcd\
             \n      type: custom\n    - name: edfg\n      type: custom"
        );
    }

    #[test]
    fn test_route_serialization() {
        assert_eq!(
            serde_yaml::to_string(&Route {
                name: Some("Name1".to_owned()),
                segment: Segment::Exact("segment_name_1".to_owned()),
                routes: Vec::new(),
            })
            .unwrap(),
            "---\nname: Name1\nexact: segment_name_1"
        );
        assert_eq!(
            serde_yaml::to_string(&Route {
                name: Some("Name2".to_owned()),
                segment: Segment::Str("segment_name_2".to_owned()),
                routes: Vec::new(),
            })
            .unwrap(),
            "---\nname: Name2\nstring: segment_name_2"
        );
    }

    #[test]
    fn test_member_serialization() {
        assert_eq!(
            serde_yaml::to_string(&Member::Contained {
                name: "abc".to_owned(),
                tipe: "custom".to_owned(),
                container: ContainerType::Vector,
            })
            .unwrap(),
            "---\nname: abc\ntype: custom\ncontainer: vector"
        );
    }

    #[test]
    fn test_member_deserialization() {
        let d1 = "---\nname: abc\ntype: u8".to_owned();
        let d2 = "---\nname: abc\ntype: string".to_owned();
        let d3 = "---\nname: abc\ntype: custom".to_owned();
        let d4 = "---\nname: abc\ntype: string 2".to_owned();
        let d5 = "---\nname: abc\ntype: custom\ncontainer: vector".to_owned();
        let s1 = serde_yaml::from_str::<Member>(&d1).unwrap();
        let s2 = serde_yaml::from_str::<Member>(&d2).unwrap();
        let s3 = serde_yaml::from_str::<Member>(&d3).unwrap();
        let s4 = serde_yaml::from_str::<Member>(&d4).unwrap();
        let s5 = serde_yaml::from_str::<Member>(&d5).unwrap();
        assert_eq!(s1, Member::Simple { name: "abc".to_owned(), tipe: SimpleType::U8 });
        assert_eq!(s2, Member::Simple { name: "abc".to_owned(), tipe: SimpleType::Str });
        assert_eq!(s3, Member::Defined { name: "abc".to_owned(), tipe: "custom".to_owned() });
        assert_eq!(s4, Member::Defined { name: "abc".to_owned(), tipe: "string 2".to_owned() });
        assert_eq!(
            s5,
            Member::Contained {
                name: "abc".to_owned(),
                tipe: "custom".to_owned(),
                container: ContainerType::Vector,
            }
        );
    }
}
