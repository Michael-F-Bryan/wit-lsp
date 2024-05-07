use std::borrow::Cow;

use crate::{BlockComment, Builtins, HasSource, Identifier, Semver, UserDefinedType};

macro_rules! literal_types {
    ($($ty:ident),* $(,)?) => {
        $(
            impl $ty<'_> {
                pub fn value(self, src: &str) -> &str {
                    self.utf8_text(src)
                }
            }
        )*
    };
}

literal_types!(Identifier, Builtins, UserDefinedType, Semver);

impl BlockComment<'_> {
    /// The actual comment text.
    pub fn text(self, src: &str) -> Cow<'_, str> {
        let src = self.utf8_text(src);
        // let src = src.trim();

        // Remove the initial /* and final */ from the string
        let content = &src[2..src.len() - 2].trim();

        if !content.contains('\n') {
            return Cow::Borrowed(content);
        }

        // Split the content into lines, stripping the leading and trailing whitespace
        let lines: Vec<&str> = content.lines().map(str::trim).collect();

        // Calculate the smallest number of leading spaces in lines that are not only asterisks
        let min_spaces = lines
            .iter()
            .filter(|&line| !line.trim().is_empty() && !line.trim().chars().all(|c| c == '*'))
            .map(|line| line.chars().take_while(|&c| c == ' ').count())
            .min()
            .unwrap_or(0);

        // Concatenate all lines, removing the calculated leading spaces and leading asterisks
        let mut result = String::new();
        for (i, line) in lines.iter().enumerate() {
            if i > 0 {
                result.push('\n');
            }
            let trimmed_line = if line.len() > min_spaces {
                line[min_spaces..].trim_start_matches('*').trim_start()
            } else {
                ""
            };
            result.push_str(trimmed_line);
        }

        Cow::Owned(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::AstNode;

    use super::*;

    fn block_comment_value(src: &str) -> Cow<'_, str> {
        let tree = tree_sitter_wit::parse(src);
        let mut cursor = tree.walk();
        let comment = tree
            .root_node()
            .children(&mut cursor)
            .find_map(BlockComment::cast)
            .unwrap();

        comment.text(src)
    }

    #[test]
    fn single_line_block_comment() {
        let src = "/* word */";

        let got = block_comment_value(src);

        assert_eq!(got, "word");
    }

    #[test]
    fn multi_line_block_comment() {
        let src = r#"
        /* this
            is
            a
            comment */
    "#;

        let got = block_comment_value(src);

        assert_eq!(got, "this\nis\na\ncomment");
    }

    #[test]
    fn multi_line_block_comment_with_asterisks() {
        let src = r#"
        /*
            * this
            * is
            * a
            * comment
            */
    "#;

        let got = block_comment_value(src);

        assert_eq!(got, "this\nis\na\ncomment");
    }

    #[test]
    #[ignore = "FIXME"]
    fn multi_line_block_comment_with_indentation() {
        let src = r#"
        /*
            this
            is
                an indented
            comment
         */
    "#;

        let got = block_comment_value(src);

        assert_eq!(got, "this\nis\n     an indented\ncomment");
    }
}
