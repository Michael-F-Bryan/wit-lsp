use codespan_reporting::diagnostic::Severity;
use tree_sitter::{Query, QueryCursor};

use crate::diagnostics::{Diag, Diagnostic, IntoDiagnostic, LintContext, LintPass, Location};

pub(crate) const DEPRECATED: LintPass = LintPass {
    name: "deprecated",
    default_severity: Severity::Warning,
    run: lint_deprecated,
};

const QUERY: &str = r#"
    ["float32", "float64"] @deprecated
"#;

fn lint_deprecated(ctx: LintContext<'_>) {
    let lang = tree_sitter_wit::language();
    let query = Query::new(&lang, QUERY).unwrap();

    let mut cursor = QueryCursor::new();

    let matches = cursor
        .matches(&query, ctx.root_node(), ctx.src().as_bytes())
        .flat_map(|m| m.captures)
        .map(|m| m.node);

    for node in matches {
        let location = ctx.location(node);
        ctx.emit(Deprecated { location });
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
