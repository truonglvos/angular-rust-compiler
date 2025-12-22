use super::*;
use crate::ngtsc::diagnostics::{ErrorCode, FatalDiagnosticError, Node, ng_error_code, replace_ts_with_ng_in_errors};
use ts::{SourceFile, SyntaxKind, NodeFlags, LanguageVariant, ScriptTarget};

#[derive(Debug)]
struct MockSourceFile {
    file_name: String,
    text: String,
}

impl Node for MockSourceFile {
    fn kind(&self) -> SyntaxKind { SyntaxKind::SourceFile }
    fn flags(&self) -> NodeFlags { NodeFlags::None }
    fn pos(&self) -> usize { 0 }
    fn end(&self) -> usize { self.text.len() }
    fn get_start(&self, _source_file: Option<&dyn SourceFile>) -> usize { 0 }
    fn get_width(&self, _source_file: Option<&dyn SourceFile>) -> usize { self.text.len() }
    fn get_source_file(&self) -> Option<&dyn SourceFile> { Some(self) }
    fn parent(&self) -> Option<&dyn Node> { None }
}

impl SourceFile for MockSourceFile {
    fn text(&self) -> &str { &self.text }
    fn file_name(&self) -> &str { &self.file_name }
    fn language_variant(&self) -> LanguageVariant { LanguageVariant::Standard }
    fn is_declaration_file(&self) -> bool { false }
    fn has_no_default_lib(&self) -> bool { false }
    fn language_version(&self) -> ScriptTarget { ScriptTarget::ES2015 }
}

#[derive(Debug)]
struct MockNode {
    start: usize,
    width: usize,
    source_file: Option<Box<MockSourceFile>>,
}

impl Node for MockNode {
    fn kind(&self) -> SyntaxKind { SyntaxKind::Unknown }
    fn flags(&self) -> NodeFlags { NodeFlags::None }
    fn pos(&self) -> usize { self.start }
    fn end(&self) -> usize { self.start + self.width }
    fn get_start(&self, _source_file: Option<&dyn SourceFile>) -> usize { self.start }
    fn get_width(&self, _source_file: Option<&dyn SourceFile>) -> usize { self.width }
    fn get_source_file(&self) -> Option<&dyn SourceFile> { 
        self.source_file.as_ref().map(|sf| sf.as_ref() as &dyn SourceFile)
    }
    fn parent(&self) -> Option<&dyn Node> { None } 
}

#[test]
fn test_error_code_mapping() {
    assert_eq!(ng_error_code(ErrorCode::DecoratorArgNotLiteral), -991001);
    assert_eq!(ng_error_code(ErrorCode::ComponentMissingTemplate), -992001);
}

#[test]
fn test_replace_ts_with_ng() {
    let input = "\u{001b}[31mTS-991001: \u{001b}[0mError message";
    let expected = "\u{001b}[31mNG1001: \u{001b}[0mError message";
    assert_eq!(replace_ts_with_ng_in_errors(input), expected);
}

#[test]
fn test_fatal_diagnostic_error() {
    let source_file = MockSourceFile {
        file_name: "test.ts".to_string(),
        text: "".to_string(),
    };
    let node = MockNode { 
        start: 10, 
        width: 20, 
        source_file: Some(Box::new(source_file)) 
    };
    let err = FatalDiagnosticError::new(
        ErrorCode::DecoratorArgNotLiteral,
        Box::new(node),
        "Something went wrong",
        None
    );
    
    let diag = err.to_diagnostic();
    assert_eq!(diag.code, -991001);
    assert_eq!(diag.start, 10);
    assert_eq!(diag.length, 20);
    assert_eq!(diag.file.as_deref(), Some("test.ts"));
    
    let display = format!("{}", err);
    assert!(display.contains("FatalDiagnosticError"));
    assert!(display.contains("Code: DecoratorArgNotLiteral"));
    assert!(display.contains("Message: Something went wrong"));
}
