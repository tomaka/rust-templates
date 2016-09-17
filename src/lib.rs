#![feature(rustc_macro, rustc_macro_lib, io)]

extern crate regex;
extern crate rustc_macro;
extern crate syntex_syntax;

use std::fs::File;
use std::path::Path;

use regex::Regex;
use rustc_macro::TokenStream;

mod exprs;
mod template;

#[rustc_macro_derive(Template)]
pub fn derive_template(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();

    let (path, struct_name) = {
        // Yes, we're parsing Rust code with a regex. Problem?
        let regex = Regex::new(r#"^#\[path\s*=\s*"(.*?)"\]\s*struct (.*?);\s*"#).unwrap();

        let captures = regex.captures_iter(&input_str).next().unwrap();
        let path = captures.at(1).unwrap();
        let struct_name = captures.at(2).unwrap();
        (path, struct_name)
    };

    let file = File::open(Path::new(path)).unwrap();
    let elements = template::parse(file);
    let out = exprs::analyze(struct_name, &elements);

    out.parse().unwrap()
}
