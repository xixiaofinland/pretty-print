use std::{fs, path::Path};

use pretty_printing::{pretty_print, NBuilder, NRef};
use tree_sitter::{Language, Node, Parser, Tree};

extern "C" {
    fn tree_sitter_apex() -> Language;
}

pub fn language() -> Language {
    unsafe { tree_sitter_apex() }
}

pub fn parse() -> Tree {
    let mut parser = Parser::new();
    parser
        .set_language(&language())
        .expect("Error loading Apex grammar");

    let source_code = fs::read_to_string(Path::new("examples/a.cls")).unwrap();

    let ast_tree = parser.parse(&source_code, None).unwrap();
    let root_node = &ast_tree.root_node();

    if root_node.has_error() {
        panic!("Parser encounters an error node in the tree.");
    }

    println!("{}", ast_tree.root_node().to_sexp());
    ast_tree
}

fn main() {
    parse();
}
