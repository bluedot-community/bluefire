// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! This macro generates a constructor initializing a structure composed only of `&'static str`
//! fields using the field names. Useful for defining serializable bundles of constants.

// -------------------------------------------------------------------------------------------------

enum Format {
    Snake,
    Kebab,
}

impl Format {
    fn from_str(string: &str) -> Option<Self> {
        match string {
            "snake" => Some(Format::Snake),
            "kebab" => Some(Format::Kebab),
            _ => None,
        }
    }
}

struct Config {
    format: Format,
}

impl Default for Config {
    fn default() -> Self {
        Self { format: Format::Snake }
    }
}

struct Code {
    struct_name: proc_macro2::Ident,
    fields: Vec<syn::Ident>,
}

// -------------------------------------------------------------------------------------------------

fn parse_attibutes(attributes: proc_macro2::TokenStream) -> Config {
    let mut config = Config::default();
    if !attributes.is_empty() {
        let meta: syn::Meta = syn::parse2(attributes).expect("failed to parse attributes");
        match meta {
            syn::Meta::NameValue(value) => {
                let name = value.ident.to_string();
                match name.as_ref() {
                    "format" => match value.lit {
                        syn::Lit::Str(ref lit_str) => {
                            if let Some(format) = Format::from_str(&lit_str.value()) {
                                config.format = format;
                            } else {
                                panic!("Unacceptable format '{}'", lit_str.value());
                            }
                        }
                        _ => panic!("Argument '{}' must be a string", name),
                    },
                    _ => panic!("Unacceptable attribute name '{}'", name),
                }
            }
            _ => panic!("Unacceptable attribute type"),
        }
    }
    config
}

fn parse_input(input: proc_macro2::TokenStream) -> Code {
    let item: syn::Item = syn::parse2(input).expect("failed to parse input");
    match item {
        syn::Item::Struct(item_struct) => {
            let mut fields = Vec::new();
            let struct_name = item_struct.ident.clone();
            match item_struct.fields {
                syn::Fields::Named(named_fields) => {
                    for field in named_fields.named.iter() {
                        if let Some(ref field_ident) = field.ident {
                            fields.push(field_ident.clone());
                        } else {
                            panic!("A field does not have a name");
                        }
                    }
                }
                _ => panic!("This macro can be applied only to structures with names fields"),
            }
            Code { struct_name, fields }
        }
        _ => panic!("This macro can be applied only to structures"),
    }
}

fn make_fields_code(config: &Config, code: &Code) -> Vec<proc_macro2::TokenStream> {
    let mut result = Vec::new();
    for field in code.fields.iter() {
        let span = field.span().clone();
        let field_name = match config.format {
            Format::Snake => field.to_string(),
            Format::Kebab => field.to_string().split("_").collect::<Vec<&str>>().join("-"),
        };
        result.push(quote::quote_spanned!(span=> #field: #field_name,));
    }
    result
}

// -------------------------------------------------------------------------------------------------

pub fn new_constant(
    attributes: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attibutes = proc_macro2::TokenStream::from(attributes);
    let input = proc_macro2::TokenStream::from(input);
    let config = parse_attibutes(attibutes);
    let code = parse_input(input.clone());
    let struct_name = &code.struct_name;
    let fields = make_fields_code(&config, &code);

    let gen = quote::quote! {
        #input

        impl #struct_name {
            /// Constructs new instance using field names as field values.
            pub const fn new_constant() -> Self {
                Self {
                    #( #fields )*
                }
            }
        }
    };
    gen.into()
}
