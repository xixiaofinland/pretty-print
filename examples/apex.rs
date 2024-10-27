use colored::Colorize;
use pretty_printing::{pretty_print, NBuilder, NRef};
use std::{fs, path::Path};
use tree_sitter::{Language, Node, Parser, Tree};

fn main() {
    let (tree, source_code) = parse();
    let n = tree.root_node().first_c();

    let notation_builder = NBuilder::new();
    let builder = TreeBuilder(&notation_builder);
    let notation = builder.rewrite_class(n, &source_code);

    let output = pretty_print(notation, 40);
    println!("{}", output);
}

#[derive(Clone, Copy)]
struct TreeBuilder<'a>(&'a NBuilder<'a>);

impl<'a> TreeBuilder<'a> {
    fn rewrite_class(self, n: Node, source_code: &str) -> NRef<'a> {
        let v = n.v(source_code);
        self.0.txt(v)
    }
}

//fn rewrite_class<'a>(node: &Node, source_code: &str) -> NRef<'a> {
//    //try_add_pref_and_offset(&mut result, shape, context);
//    let b = NBuilder::new();
//
//    if let Some(ref a) = node.try_c_by_k("modifiers") {
//        result.push_str(&rewrite::<Modifiers>(a, shape, context));
//
//        if let Some(_) = a.try_c_by_k("modifier") {
//            result.push(' ');
//        }
//    }
//
//    result.push_str("class ");
//    result.push_str(node.cv_by_n("name", source_code));
//
//    if let Some(ref c) = node.try_c_by_n("type_parameters") {
//        result.push_str(&rewrite_shape::<TypeParameters>(c, shape, false, context));
//    }
//
//    if let Some(ref c) = node.try_c_by_n("superclass") {
//        result.push_str(&rewrite_shape::<SuperClass>(c, shape, false, context));
//    }
//
//    if let Some(ref c) = node.try_c_by_n("interfaces") {
//        result.push_str(&rewrite_shape::<Interfaces>(c, shape, false, context));
//    }
//
//    result.push_str(" {\n");
//
//    let body_node = node.c_by_n("body");
//    result.push_str(&body_node.apply_to_standalone_children(
//        shape,
//        context,
//        |c, c_shape, c_context| c._visit(c_shape, c_context),
//    ));
//
//    result.push_str(&format!("{}}}", shape.indent.as_string(context.config)));
//    //try_add_standalone_suffix_no_semicolumn(node, &mut result, shape, source_code);
//    result
//}

fn parse() -> (Tree, String) {
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

    (ast_tree, source_code)
}

#[allow(dead_code)]
pub trait Accessor<'t> {
    fn v<'a>(&self, source_code: &'a str) -> &'a str;
    fn children_vec(&self) -> Vec<Node<'t>>;
    fn all_children_vec(&self) -> Vec<Node<'t>>;

    fn try_c_by_n(&self, kind: &str) -> Option<Node<'t>>;
    fn try_c_by_k(&self, kind: &str) -> Option<Node<'t>>;
    fn try_cv_by_n<'a>(&self, name: &str, source_code: &'a str) -> Option<&'a str>;
    fn try_cv_by_k<'a>(&self, kind: &str, source_code: &'a str) -> Option<&'a str>;
    fn try_cs_by_k(&self, kind: &str) -> Vec<Node<'t>>;
    fn try_csv_by_k<'a>(&self, kind: &str, source_code: &'a str) -> Vec<&'a str>;

    fn c_by_n(&self, name: &str) -> Node<'t>;
    fn c_by_k(&self, kind: &str) -> Node<'t>;
    fn first_c(&self) -> Node<'t>;
    fn cv_by_k<'a>(&self, name: &str, source_code: &'a str) -> &'a str;
    fn cv_by_n<'a>(&self, name: &str, source_code: &'a str) -> &'a str;
    fn cs_by_k(&self, kind: &str) -> Vec<Node<'t>>;
    fn cs_by_n(&self, name: &str) -> Vec<Node<'t>>;

    fn is_comment(&self) -> bool;
}

impl<'t> Accessor<'t> for Node<'t> {
    fn v<'a>(&self, source_code: &'a str) -> &'a str {
        self.utf8_text(source_code.as_bytes())
            .unwrap_or_else(|_| panic!("{}: get_value failed.", self.kind().red()))
    }

    fn children_vec(&self) -> Vec<Node<'t>> {
        let mut cursor = self.walk();
        self.named_children(&mut cursor).collect()
    }

    fn all_children_vec(&self) -> Vec<Node<'t>> {
        let mut cursor = self.walk();
        self.children(&mut cursor).collect()
    }

    fn try_c_by_k(&self, kind: &str) -> Option<Node<'t>> {
        let mut cursor = self.walk();
        let child = self.named_children(&mut cursor).find(|c| c.kind() == kind);
        child
    }

    fn try_cs_by_k(&self, kind: &str) -> Vec<Node<'t>> {
        let mut cursor = self.walk();
        self.named_children(&mut cursor)
            .filter(|c| c.kind() == kind)
            .collect()
    }

    fn try_c_by_n(&self, name: &str) -> Option<Node<'t>> {
        self.child_by_field_name(name)
    }

    fn try_cv_by_n<'a>(&self, name: &str, source_code: &'a str) -> Option<&'a str> {
        self.child_by_field_name(name).map(|n| n.v(source_code))
    }

    fn c_by_k(&self, kind: &str) -> Node<'t> {
        self.try_c_by_k(kind).unwrap_or_else(|| {
            panic!(
                "{}: missing mandatory kind child: {}.",
                self.kind().red(),
                kind.red()
            )
        })
    }

    fn first_c(&self) -> Node<'t> {
        self.named_child(0)
            .unwrap_or_else(|| panic!("{}: missing a mandatory child.", self.kind().red()))
    }

    fn cv_by_k<'a>(&self, name: &str, source_code: &'a str) -> &'a str {
        let child_node = self.c_by_k(name);
        child_node.v(source_code)
    }

    fn cv_by_n<'a>(&self, name: &str, source_code: &'a str) -> &'a str {
        let node = self.child_by_field_name(name).unwrap_or_else(|| {
            panic!(
                "{}: missing mandatory name child: {}.",
                self.kind().red(),
                name.red()
            )
        });
        node.v(source_code)
    }

    fn c_by_n(&self, name: &str) -> Node<'t> {
        self.child_by_field_name(name).unwrap_or_else(|| {
            panic!(
                "{}: missing mandatory name child: {}.",
                self.kind().red(),
                name.red()
            )
        })
    }

    fn cs_by_n(&self, name: &str) -> Vec<Node<'t>> {
        let mut cursor = self.walk();
        let children: Vec<Node<'t>> = self.children_by_field_name(name, &mut cursor).collect();
        if children.is_empty() {
            panic!(
                "{}: missing mandatory name child: {}.",
                self.kind().red(),
                name.red()
            );
        }
        children
    }

    fn cs_by_k(&self, kind: &str) -> Vec<Node<'t>> {
        let children = self.try_cs_by_k(kind);
        if children.is_empty() {
            panic!(
                "{}: missing mandatory kind children: {}.",
                self.kind().red(),
                kind.red()
            );
        }
        children
    }

    fn try_csv_by_k<'a>(&self, kind: &str, source_code: &'a str) -> Vec<&'a str> {
        self.try_cs_by_k(kind)
            .iter()
            .map(|n| n.v(source_code))
            .collect::<Vec<&str>>()
    }

    fn try_cv_by_k<'a>(&self, kind: &str, source_code: &'a str) -> Option<&'a str> {
        self.try_c_by_k(kind).map(|child| child.v(source_code))
    }

    fn is_comment(&self) -> bool {
        matches!(self.kind(), "line_comment" | "block_comment")
    }
}

extern "C" {
    fn tree_sitter_apex() -> Language;
}

pub fn language() -> Language {
    unsafe { tree_sitter_apex() }
}
