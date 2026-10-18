use codespan_reporting::diagnostic::Severity;
use tree_sitter::{Query, QueryCursor, StreamingIterator};

use crate::diagnostics::{Diag, Diagnostic, IntoDiagnostic, LintContext, LintPass, Location};

pub(crate) const DEPRECATED: LintPass = LintPass {
    name: "deprecated",
    default_severity: Severity::Warning,
    run: lint_deprecated,
};

const QUERY: &str = r#"
    ["float32" "float64"] @deprecated
"#;

fn lint_deprecated(ctx: LintContext<'_>) {
    let lang = tree_sitter_wit::language();
    let query = Query::new(&lang, QUERY).unwrap();

    let mut cursor = QueryCursor::new();

    let mut matches = cursor.matches(&query, ctx.root_node(), ctx.src().as_bytes());
    while let Some(m) = matches.next() {
        for capture in m.captures() {
            let location = ctx.location(capture.node);
            ctx.emit(Deprecated { location });
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Deprecated {
    pub location: Location,
}

impl IntoDiagnostic for Deprecated {
    const CODE: &'static str = "W001";
    const MESSAGE: &'static str = "This type is deprecated";
    const VERBOSE_DESCRIPTION: &'static str = include_str!("W001-deprecated.md");
    const SEVERITY: Severity = Severity::Warning;

    fn update_diag(&self, diag: Diag) -> Diag {
        diag.with_labels(vec![self.location.label()])
    }
}

impl From<Deprecated> for Diagnostic {
    fn from(value: Deprecated) -> Self {
        Diagnostic::Deprecated(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Compiler,
        diagnostics::check_all,
        queries::{FilePath, SourceFile, Workspace},
    };

    use super::*;

    #[test]
    fn deprecated_types_report_each_capture_and_refresh_after_edits() {
        let source = "// float32 and float64 in a comment are not types\n\
                      interface floats {\n\
                        type first = float32;\n\
                        type second = float64;\n\
                        type third = float32;\n\
                        type current = f32;\n\
                      }";
        let mut db = Compiler::default();
        let path = FilePath::new(&db, "floats.wit".into());
        let file = SourceFile::new(&db, path, source.into());
        let ws = Workspace::new(&db, [(path, file)].into_iter().collect());

        let diagnostics = check_all(&db, ws);
        let ranges: Vec<_> = diagnostics
            .iter()
            .map(|diag| {
                let Diagnostic::Deprecated(diag) = diag else {
                    panic!("unexpected diagnostic: {diag:?}");
                };
                assert_eq!(diag.location.filename, path);
                diag.location.range.start_byte..diag.location.range.end_byte
            })
            .collect();
        let expected: Vec<_> = source
            .match_indices("= float")
            .map(|(start, _)| start + 2..start + 9)
            .collect();
        assert_eq!(ranges, expected);
        assert_eq!(ranges.len(), 3);

        ws.update(
            &mut db,
            "floats.wit",
            source.replace("float32", "f32").replace("float64", "f64"),
        );
        assert!(check_all(&db, ws).is_empty());
    }
}
