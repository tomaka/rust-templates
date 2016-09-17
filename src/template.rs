use std::io::Read;
use std::mem;

#[derive(Debug, Clone)]
pub enum Element {
    Template(String),
    RustExpr(String),
}

pub fn parse<R: Read>(input: R) -> Vec<Element> {
    let mut elements = Vec::new();
    let mut current_element = Element::Template(String::new());
    
    let mut previous_char = None;
    for chr in input.chars().filter_map(|c| c.ok()) {
        if previous_char == Some(chr) && (chr == '{' || chr == '}') {
            match current_element {
                Element::Template(ref mut s) => s.pop(),
                Element::RustExpr(ref mut s) => s.pop(),
            };

            let new_element = match current_element {
                Element::Template(_) => Element::RustExpr(String::new()),
                Element::RustExpr(_) => Element::Template(String::new()),
            };

            elements.push(mem::replace(&mut current_element, new_element));

        } else {
            let s = match current_element {
                Element::Template(ref mut s) => s,
                Element::RustExpr(ref mut s) => s,
            };

            s.push(chr);
        }

        previous_char = Some(chr);
    }

    elements.push(current_element);
    elements
}
