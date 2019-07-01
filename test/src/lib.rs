extern crate proc_macro;

use proc_macro::{
    Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree,
};

/// The different shapes of mazes for which to generate tests.
const SHAPES: &[&str] = &["hex", "quad", "tri"];

/// Marks a function as a test for a maze.
///
/// Adding this attribute macro will ensure that the function is run as a test
/// for all kinds of mazes.
///
/// The annotated function should take one argument, which is the maze
/// instance.
#[proc_macro_attribute]
pub fn maze_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Extract the interesting parts of the original function
    let (span, name, args, inner_body) = split(item);

    // Generate the body of the new function
    let body = {
        let mut body =
            function(span, Ident::new("inner", span), args, inner_body);

        // Iterate through known shapes for consistent ordering
        for shape in SHAPES {
            body.extend(
                format!(
                    "inner(&mut \"{}\".parse::<crate::Shape>()
                    .unwrap().create(10, 5));",
                    shape,
                )
                .parse::<TokenStream>()
                .unwrap(),
            );
        }
        body
    };

    let mut result = TokenStream::new();
    result.extend(test_attr(span));
    result.extend(function(
        span,
        name,
        Group::new(Delimiter::Parenthesis, TokenStream::new()),
        Group::new(Delimiter::Brace, body),
    ));
    result
}

/// Splits a token stream into the components we use.
///
/// This function expects a function definition. It does not validate the
/// function arguments.
///
/// # Arguments
/// *  `item` - The token stream to split.
///
/// # Panics
/// This function will panic if the token stream does not contain the expected
/// tokens.
fn split(item: TokenStream) -> (Span, Ident, Group, Group) {
    let mut items = item.into_iter();

    match (items.next(), items.next(), items.next(), items.next()) {
        (
            Some(TokenTree::Ident(head)),
            Some(TokenTree::Ident(name)),
            Some(TokenTree::Group(args)),
            Some(TokenTree::Group(body)),
        ) => {
            if head.to_string() != "fn" {
                panic!("Expected function")
            } else {
                (head.span(), name, args, body)
            }
        }
        _ => panic!("Expected function"),
    }
}

/// Generates a test attribute.
///
/// # Arguments
/// *  `span` - The span of the original function.
fn test_attr(span: Span) -> TokenStream {
    vec![
        TokenTree::Punct(Punct::new('#', Spacing::Alone)),
        TokenTree::Group(Group::new(
            Delimiter::Bracket,
            vec![TokenTree::Ident(Ident::new("test", span))]
                .into_iter()
                .collect(),
        )),
    ]
    .into_iter()
    .collect()
}

/// Generates a function.
///
/// # Arguments
/// *  `span` - The span of the original function.
/// *  `name` - The function name.
/// *  `args` - The function arguments.
/// *  `body` - The function body.
fn function(span: Span, name: Ident, args: Group, body: Group) -> TokenStream {
    vec![
        TokenTree::Ident(Ident::new("fn", span)),
        TokenTree::Ident(name),
        TokenTree::Group(args),
        TokenTree::Group(body),
    ]
    .into_iter()
    .collect::<TokenStream>()
}
