use super::source_code::{extract_range, LineColumn, Range};
use anyhow::{Context, Result};
use proc_macro2::{Span, TokenStream, TokenTree};
use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::{Macro, PathSegment};

#[derive(Debug)]
struct MacroVisitor {
    found: Option<(TokenStream, Macro)>,
    line: usize,
    macro_name: String,
}

impl<'ast> Visit<'ast> for MacroVisitor {
    fn visit_macro(&mut self, m: &'ast Macro) {
        let last_path_segment = m.path.segments.last();
        if let Some(PathSegment { ident, .. }) = last_path_segment {
            if ident.to_string().as_str() == self.macro_name
                && ident.span().start().line == self.line
            {
                self.found.replace((m.tokens.to_owned(), m.to_owned()));
            }
        }
    }
}

/// Find the Range containing the space where snapshot literal needs to be written.
///
/// If a literal already exists, it will return the range of the existing string literal.
///
/// some_macro!(blah);
///                ^
///                |
///        0 length range of the position where
///        new snapshot needs to be written
///            
/// Otherwise, if the snapshot literal is there, return its range
///
/// some_macro(blah, "hello")
///                  ^     ^
///                  |     |
///                   range
pub fn find_snapshot_literal_range<S: Into<String>>(
    file_content: &str,
    macro_name: S,
    line_num: usize,
    literal_exists: bool,
) -> Result<Range> {
    let syntax = syn::parse_file(file_content)
        .context("Unable to parse file using syn::parse_file")?;

    let macro_name = macro_name.into();

    let mut macro_visitor = MacroVisitor {
        found: None,
        line: line_num,
        macro_name: macro_name.clone(),
    };

    macro_visitor.visit_file(&syntax);

    let (tt, macro_node) = macro_visitor.found.with_context(|| {
        format!(
            "Failed to find a macro call AST node with macro name `{}!()`.\nThis macro was called on line `{}`\n\n",
            &macro_name, line_num
        )
    })?;

    if literal_exists {
        let literal = tt.into_iter().last();

        if let Some(TokenTree::Literal(literal)) = literal {
            Ok(Range {
                start: LineColumn {
                    line: literal.span().start().line,
                    // columns might be 0 based? i'm not sure
                    column: literal.span().start().column + 1,
                },
                end: LineColumn {
                    line: literal.span().end().line,
                    column: literal.span().end().column + 1,
                },
            })
        } else {
            let macro_range = syn_span_to_range(macro_node.span());
            let macro_code = extract_range(file_content, &macro_range);
            anyhow::bail!(
                r#"
Failed to extract a snapshot literal from a snapshot macro call.
Snapshot literal must be the last argument to a macro call and must be a string literal. e.g.

assert_matches_inline_snapshot!(12345, "12345");
                                       ^     ^
                                       |     |
                                   snapshot literal

Given macro call:

```
{}
```
"#,
                macro_code,
            )
        }
    } else {
        let last = tt
            .into_iter()
            .last()
            .ok_or(anyhow!("must have last tokentree"))?;

        

        let span = last.span();

        Ok(Range {
            start: LineColumn {
                line: span.end().line,
                column: span.end().column + 1,
            },
            end: LineColumn {
                line: span.end().line,
                column: span.end().column + 1,
            },
        })
    }
}

/// Convert proc_macro2 Span struct to local Range struct, which indexes
/// for Lines and Columns starting from 1 and not 0
fn syn_span_to_range(span: Span) -> Range {
    Range {
        start: LineColumn {
            line: span.start().line,
            column: span.start().column + 1,
        },
        end: LineColumn {
            line: span.end().line,
            column: span.end().column + 1,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: &str = r##"                       // 1
fn main() {                                         // 2
    let hello = "world";                            // 3
    random_macro!(hello);                           // 4 
    hello_macro!(stuff, "literal");                 // 5
    wrong_macro!(stuff, not_a_literal);             // 6
}
"##;
    #[test]
    fn no_literal() -> Result<()> {
        let range = find_snapshot_literal_range(SOURCE, "random_macro", 4, false)?;
        k9_stable::assert_equal!(&range.start, &range.end);

        k9_stable::assert_matches_inline_snapshot!(
            format!("{:?}", range),
            r##"Range { start: LineColumn { line: 4, column: 24 }, end: LineColumn { line: 4, column: 24 } }"##
        );
        Ok(())
    }

    #[test]
    fn literal() -> Result<()> {
        let range = find_snapshot_literal_range(SOURCE, "hello_macro", 5, true)?;

        k9_stable::assert_matches_inline_snapshot!(
            format!("{:?}", range),
            r##"Range { start: LineColumn { line: 5, column: 25 }, end: LineColumn { line: 5, column: 34 } }"##
        );
        Ok(())
    }

    #[test]
    fn not_a_literal_error() {
        let err = find_snapshot_literal_range(SOURCE, "wrong_macro", 6, true).unwrap_err();

        k9_stable::assert_matches_inline_snapshot!(
            format!("{:?}", err),
            r##"
Failed to extract a snapshot literal from a snapshot macro call.
Snapshot literal must be the last argument to a macro call and must be a string literal. e.g.

assert_matches_inline_snapshot!(12345, "12345");
                                       ^     ^
                                       |     |
                                   snapshot literal

Given macro call:

```
wrong_macro!(stuff, not_a_literal)
```
"##
        );
    }
}
