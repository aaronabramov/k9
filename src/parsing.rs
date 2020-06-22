use ra_syntax::SyntaxNode;

// Transform line!()and column!() macro returns into a byte offset in the source file content `&str`
// This byte offset should be exactly the same as the result of `ra_syntax` tokenizer return, since
// we'll be looking through the AST using ra parser.
pub fn line_and_column_to_offset(
    file_content: &str,
    line_num: u32,
    column_num: u32,
) -> Result<u32, String> {
    let mut bytes_before = 0;

    for (i, line) in file_content.lines().enumerate() {
        if i as u32 + 1 == line_num {
            let line_length = line.len() as u32;
            if line_length < column_num {
                return Err(format!(
                    "line `{}` have `{}` characters and doesn't have `{}` column",
                    line_num, line_length, column_num
                ));
            }
            bytes_before += column_num - 1;
            break;
        } else {
            bytes_before += line.len() as u32 + 1; // line + 1 byte for \n character
        }
    }
    Ok(bytes_before)
}

pub fn ast_dfs_find_node<F>(node: SyntaxNode, f: F) -> Result<Option<SyntaxNode>, String>
where
    F: Fn(&SyntaxNode) -> Result<bool, String>,
{
    let mut stack = vec![node];

    while let Some(next) = stack.pop() {
        let is_needle = f(&next).unwrap();
        if is_needle {
            return Ok(Some(next));
        } else {
            for child in next.children() {
                stack.push(child);
            }
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    const CODE: &str = r#"fn main() {
    assert_equal!((1, 2), (2, 4));
    println!("{}", "hello world");
}

pub fn test() {
    assert_greater_than!(1, 2);
    assert_matches_inline_snapshot!("hello", "howdy");
}
"#;

    use super::*;
    use crate::*;
    use anyhow::Result;
    use ra_syntax::{AstNode, File, SyntaxKind};

    fn first_index(s: &str, substring: &str) -> u32 {
        s.match_indices(substring)
            .collect::<Vec<_>>()
            .pop()
            .unwrap()
            .0 as u32
    }

    #[test]
    fn test_line_and_column_to_offset() -> Result<()> {
        assert_equal!(line_and_column_to_offset(CODE, 1, 1), Ok(0));
        assert_equal!(
            line_and_column_to_offset(CODE, 7, 5),
            Ok(first_index(CODE, "assert_greater_than"))
        );
        Ok(())
    }

    #[test]
    #[cfg(feature = "regex")]
    fn test_ast_finder() -> Result<()> {
        let file = File::parse(CODE);
        let root = file.ast().syntax().owned();

        let macro_call_node = ast_dfs_find_node(root, |node| match node.kind() {
            SyntaxKind::MACRO_CALL => Ok(true),
            _ => Ok(false),
        })
        .expect("errored while searching for node")
        .expect("failed to find MACRO_CALL node in the tree");

        assert_equal!(macro_call_node.kind(), SyntaxKind::MACRO_CALL);

        // since it's popping from the stack, it'll DFS in reverse children mode
        assert_matches_regex!(
            &macro_call_node.text().to_string(),
            "assert_matches_inline.*"
        );

        let root = file.ast().syntax().owned();
        let macro_call_node = ast_dfs_find_node(root, |node| match node.kind() {
            SyntaxKind::MACRO_CALL => {
                let p = node
                    .first_child()
                    .ok_or_else(|| "macro call must have a first child in AST".to_string())?;

                Ok(&p.text().to_string() == "println")
            }
            _ => Ok(false),
        })
        .expect("errored while searching for node")
        .expect("failed to find MACRO_CALL node in the tree");

        assert_equal!(macro_call_node.kind(), SyntaxKind::MACRO_CALL);

        assert_matches_regex!(&macro_call_node.text().to_string(), "println.*");
        Ok(())
    }
}
