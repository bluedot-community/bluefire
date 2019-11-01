// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You can obtain one at http://mozilla.org/MPL/2.0/

//! Helpers for executing other programs needed for building `bluefire` appications.

fn handle_output(output: std::process::Output, args: &Vec<&str>) {
    use std::io::{self, Write};
    if !output.status.success() {
        io::stderr().write_all(&output.stderr).unwrap();
        panic!("Failed to run a command with args: {:?}", args);
    }
}

/// `wasm-bindgen` command wrapper.
///
/// See the documentation of `wasm-bindgen` for information about the arguments.
pub mod wasm_bindgen {
    #![allow(missing_docs)]

    pub enum Target {
        Web,
        Bundler,
        NodeJs,
        NoModules,
    }

    impl Target {
        fn to_str(&self) -> &'static str {
            match self {
                Target::Web => "web",
                Target::Bundler => "bundler",
                Target::NodeJs => "nodejs",
                Target::NoModules => "no-modules",
            }
        }
    }

    pub struct WasmBindgen {
        input: String,
        outdir: Option<String>,
        target: Option<Target>,
    }

    impl WasmBindgen {
        pub fn new(input: String) -> Self {
            Self { input, outdir: None, target: None }
        }

        pub fn outdir(mut self, outdir: String) -> Self {
            self.outdir = Some(outdir);
            self
        }

        pub fn target(mut self, target: Target) -> Self {
            self.target = Some(target);
            self
        }

        pub fn run(self) {
            let mut args: Vec<&str> = vec![&self.input];
            if let Some(outdir) = self.outdir.as_ref() {
                args.push("--out-dir");
                args.push(&outdir);
            }
            if let Some(target) = self.target {
                args.push("--target");
                args.push(target.to_str());
            }

            let output = std::process::Command::new("wasm-bindgen")
                .args(&args)
                .output()
                .expect("failed to execute `wasm-bindgen`");

            super::handle_output(output, &args);
        }
    }
}

/// `sass` command wrapper.
///
/// See the documentation of `sass` for information about the arguments.
pub mod sass {
    #![allow(missing_docs)]

    pub enum Style {
        Nested,
        Compact,
        Compressed,
        Expanded,
    }

    impl Style {
        fn to_str(&self) -> &'static str {
            match self {
                Style::Nested => "nested",
                Style::Compact => "compact",
                Style::Compressed => "compressed",
                Style::Expanded => "expanded",
            }
        }
    }

    pub enum Sourcemap {
        Auto,
        File,
        Inline,
        None,
    }

    impl Sourcemap {
        fn to_str(&self) -> &'static str {
            match self {
                Sourcemap::Auto => "auto",
                Sourcemap::File => "file",
                Sourcemap::Inline => "inline",
                Sourcemap::None => "none",
            }
        }
    }

    pub struct Sass {
        input: String,
        output: String,
        style: Option<Style>,
        sourcemap: Option<Sourcemap>,
        cache_location: Option<String>,
    }

    impl Sass {
        pub fn new(input: String, output: String) -> Self {
            Self { input, output, style: None, sourcemap: None, cache_location: None }
        }

        pub fn style(mut self, style: Style) -> Self {
            self.style = Some(style);
            self
        }

        pub fn sourcemap(mut self, sourcemap: Sourcemap) -> Self {
            self.sourcemap = Some(sourcemap);
            self
        }

        pub fn cache_location(mut self, cache_location: String) -> Self {
            self.cache_location = Some(cache_location);
            self
        }

        pub fn run(self) {
            let mut args: Vec<&str> = vec![&self.input, &self.output];

            if let Some(style) = self.style {
                args.push("--style");
                args.push(style.to_str());
            }

            let sourcemap_str;
            if let Some(sourcemap) = self.sourcemap {
                sourcemap_str = format!("--sourcemap={}", sourcemap.to_str());
                args.push(&sourcemap_str);
            }

            let cache_location;
            if let Some(location) = self.cache_location {
                cache_location = location.clone();
                args.push("--cache-location");
                args.push(&cache_location);
            }

            let output = std::process::Command::new("sass")
                .args(&args)
                .output()
                .expect("failed to execute `sass`");

            super::handle_output(output, &args);
        }
    }
}
