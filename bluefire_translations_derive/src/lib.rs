// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Macros for including translations read from translation files into your application.

// TODO: Uncomment after https://github.com/rust-lang/rust/issues/42008 is fixed
// #![warn(missing_docs)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

use std::collections::HashMap;
use std::path::PathBuf;

use serde_derive::{Deserialize, Serialize};

const ATTRIBUTE_NAME: &str = "translations";
const DEFAULT_PATH: &str = "translations";
const DEFAULT_LANG: &str = "en";

// -------------------------------------------------------------------------------------------------

struct Config {
    translations_path: PathBuf,
    default_lang: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            translations_path: as_cargo_absolute_path(DEFAULT_PATH),
            default_lang: DEFAULT_LANG.to_string(),
        }
    }
}

struct Code {
    struct_name: proc_macro2::Ident,
    keys: Vec<syn::Ident>,
}

struct Info {
    config: Config,
    code: Code,
}

// -------------------------------------------------------------------------------------------------

fn as_cargo_absolute_path(relative_path: &str) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(std::env::var("CARGO_MANIFEST_DIR").expect("Cargo manifest directory not provided"));
    path.push(relative_path);
    path
}

// -------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
struct TranslationEntry {
    comment: Option<String>,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TranslationFile {
    lang_code: String,
    translations: HashMap<String, TranslationEntry>,
}

impl TranslationFile {
    fn into_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for (key, entry) in self.translations.iter() {
            map.insert(key.clone(), entry.text.clone());
        }
        map
    }
}

fn read_translations(config: &Config) -> HashMap<String, HashMap<String, String>> {
    let mut result = HashMap::new();
    match std::fs::read_dir(&config.translations_path) {
        Ok(directory) => {
            let mut processed_files = 0;
            for entry in directory {
                let path = match entry {
                    Ok(entry) => entry.path(),
                    Err(err) => panic!("Failed to read translation directory: {}", err),
                };

                if path.is_file() && path.extension().filter(|e| *e == "yaml").is_some() {
                    let string = match std::fs::read_to_string(&path) {
                        Ok(string) => string,
                        Err(err) => panic!("Failed to read file ({:?}): {}", path, err),
                    };
                    let trans: TranslationFile = match serde_yaml::from_str(&string) {
                        Ok(trans) => trans,
                        Err(err) => panic!("Parse translation file ({:?}): {}", path, err),
                    };
                    result.insert(trans.lang_code.clone(), trans.into_map());
                    processed_files += 1;
                }
            }

            if processed_files == 0 {
                panic!("No translations were provided in {:?}", config.translations_path);
            }
        }
        Err(err) => {
            panic!("Failed to read directory ({:?}): {}", config.translations_path, err);
        }
    }
    result
}

// -------------------------------------------------------------------------------------------------

fn prepare_match_arms(info: &Info) -> proc_macro2::TokenStream {
    let struct_name = &info.code.struct_name;
    let translations = read_translations(&info.config);

    let langs: Vec<&String> = translations.keys().collect();
    if !langs.contains(&&info.config.default_lang) {
        panic!(
            "Default language '{}' not provided among translations '{:?}'",
            info.config.default_lang, langs
        );
    }

    let mut arms = Vec::new();
    for (lang_code, lang_translations) in translations.iter() {
        let lang_code_lit = syn::LitStr::new(lang_code, proc_macro2::Span::call_site());
        let keys = &info.code.keys;
        let mut values = Vec::new();
        for key in info.code.keys.iter() {
            if let Some(value) = lang_translations.get(&key.to_string()) {
                values.push(syn::LitStr::new(value, proc_macro2::Span::call_site()));
            } else {
                panic!("Translation not found for key '{}' for lang '{}'", key, lang_code);
            }
        }
        let arm = quote::quote! {
            #lang_code_lit => {
                Some(#struct_name {
                    #( #keys: #values.into() ),*
                })
            }
        };
        arms.push(arm);
    }

    quote::quote! { #(#arms)* }
}

// -------------------------------------------------------------------------------------------------

fn parse_attibute_args(args: &syn::MetaList) -> Config {
    let mut config = Config::default();
    for arg in args.nested.iter() {
        match arg {
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::NameValue(value) => {
                    let a = value.path.get_ident().expect("Get ident").to_string();
                    match a.as_ref() {
                        "path" => match value.lit {
                            syn::Lit::Str(ref lit_str) => {
                                let path = as_cargo_absolute_path(&lit_str.value());
                                config.translations_path = path;
                            }
                            _ => panic!("Argument '{}' must be a string", a),
                        },
                        "default_language" => match value.lit {
                            syn::Lit::Str(ref lit_str) => {
                                config.default_lang = lit_str.value();
                            }
                            _ => panic!("Argument '{}' must be a string", a),
                        },
                        _ => panic!("Unknown argument '{}'", a),
                    }
                }
                _ => panic!("All arguments are expectedt to be name-value"),
            },
            _ => panic!("'translations' attribute is expected to have nested arguments"),
        }
    }
    config
}

fn parse_attributes(attrs: &Vec<syn::Attribute>) -> Config {
    for attr in attrs.iter() {
        match attr.parse_meta() {
            Ok(meta) => {
                let name = meta.path().get_ident().expect("Get ident").to_string();
                if name == ATTRIBUTE_NAME {
                    match meta {
                        syn::Meta::List(meta_list) => {
                            return parse_attibute_args(&meta_list);
                        }
                        _ => panic!("'translations' attribute is expected to be a list"),
                    }
                }
            }
            Err(err) => {
                panic!("Failed to parse attribute metadata: {}", err);
            }
        }
    }
    Config::default()
}

fn parse_fields(fields: &syn::Fields) -> Vec<syn::Ident> {
    match fields {
        syn::Fields::Named(named_fields) => {
            let mut keys = Vec::new();
            for field in named_fields.named.iter() {
                if let Some(ref ident) = field.ident {
                    keys.push(ident.clone());
                } else {
                    panic!("All fields should be named");
                }
            }
            keys
        }
        _ => panic!("This macro can be applied only to structures with named members"),
    }
}

fn parse_item(stream: proc_macro::TokenStream) -> Info {
    let ast: syn::DeriveInput = syn::parse(stream).expect("Failed to parse token stream");
    match ast.data {
        syn::Data::Struct(data_struct) => {
            let config = parse_attributes(&ast.attrs);
            let keys = parse_fields(&data_struct.fields);
            Info { config: config, code: Code { struct_name: ast.ident, keys: keys } }
        }
        _ => panic!("This macro can be applied only to structures"),
    }
}

// -------------------------------------------------------------------------------------------------

/// Implements `bluefire_translations::TranslationProvider`.
///
/// ## Attributes
///
/// `path` - path to the translation file relatively from the Cargo manifest directory
/// `default_lang` - the code of the default language (if not provided, "en" is used).
///
/// ## Example
///
/// Let's say we have the following file in "translations/en.yaml"
/// ``` text
/// lang_code: en
/// translations:
///  message_1:
///   text: "Message 1"
///  message_2:
///   text: "Message 2"
/// ```
///
/// Then the following will generate implementation of `bluefire_translations::TranslationProvider`:
/// ``` ignore
/// #[derive(Translations)]
/// #[translations(path = "tests/translations", default_language = "es")]
/// struct Messages {
///     msg_1: &'static str,
///     msg_2: &'static str,
/// }
/// ```
///
/// Note that the member names in the structure and entry names in the translatioon file must be
/// the same.
#[proc_macro_derive(Translations, attributes(translations))]
pub fn derive_translations(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let info = parse_item(stream);
    let struct_name = &info.code.struct_name;
    let default_lang = syn::LitStr::new(&info.config.default_lang, proc_macro2::Span::call_site());
    let match_arms = prepare_match_arms(&info);

    let gen = quote::quote! {
        impl bluefire_translations::TranslationProvider for #struct_name {
            fn provide(lang_code: &str) -> Option<Self> {
                match lang_code {
                    #match_arms
                    _ => None,
                }
            }

            fn provide_default() -> Self {
                #struct_name::provide(#default_lang)
                    .expect("BlueFire: default language is not present")
            }
        }
    };
    gen.into()
}
