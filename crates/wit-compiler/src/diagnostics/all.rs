use super::*;
/// Information about all know [`Diagnostic`] types.
pub fn all_diagnostics() -> Vec<DiagnosticInfo> {
    Vec::from([
        DiagnosticInfo::for_type::<DuplicateName>(),
        DiagnosticInfo::for_type::<MultipleConstructors>(),
        DiagnosticInfo::for_type::<SyntaxError>(),
        DiagnosticInfo::for_type::<UnknownName>(),
        DiagnosticInfo::for_type::<UnknownPackage>(),
        DiagnosticInfo::for_type::<Bug>(),
        DiagnosticInfo::for_type::<MismatchedPackageDeclaration>(),
        DiagnosticInfo::for_type::<MultiplePackageDocs>(),
        DiagnosticInfo::for_type::<Deprecated>(),
    ])
}
