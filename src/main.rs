#![deny(unsafe_code)]
#![warn(
    clippy::cognitive_complexity,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_link_with_quotes,
    clippy::doc_markdown,
    clippy::empty_line_after_outer_attr,
    clippy::empty_structs_with_brackets,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::float_equality_without_abs,
    keyword_idents,
    clippy::missing_const_for_fn,
    missing_copy_implementations,
    missing_debug_implementations,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    non_ascii_idents,
    noop_method_call,
    clippy::option_if_let_else,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::semicolon_if_nothing_returned,
    clippy::unseparated_literal_suffix,
    clippy::shadow_unrelated,
    clippy::similar_names,
    clippy::suspicious_operation_groupings,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    clippy::unused_self,
    clippy::use_debug,
    clippy::used_underscore_binding,
    clippy::useless_let_if_seq,
    clippy::wildcard_dependencies,
    clippy::wildcard_imports
)]

mod combinators;
mod forest;
mod parser;
mod primitives;

use combinators::choice::Choice;
use parser::parse_str;
use primitives::char::Char;

use crate::combinators::then::Then;

#[allow(clippy::print_stdout, clippy::use_debug)]
fn main() {
    let parser: Choice<_> = Choice::new(vec![
        Box::new(Then::new(Char('('), Char(')'))),
        Box::new(Then::new(Char('a'), Char('b'))),
    ]);

    println!("{:?}", parse_str(&parser, "ab"));
}
