#![feature(rustc_macro, type_ascription)]

#[macro_use]
extern crate rust_template;

fn main() {
    #[derive(Template)]
    #[path = "src/template.html"]
    struct Template;

    let out = Template::build(TemplateMain {
        test: "hello",
        num: 2,
    });

    println!("{}", out);
}
