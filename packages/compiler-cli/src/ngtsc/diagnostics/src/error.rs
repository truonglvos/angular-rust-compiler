use super::error_code::ErrorCode;
use super::util::ng_error_code;
use std::fmt;
use ts::{
    make_diagnostic_chain as ts_make_diagnostic_chain, DiagnosticCategory, DiagnosticMessageChain,
    DiagnosticRelatedInformation, DiagnosticWithLocation, Node,
};

#[derive(Debug)]
pub struct FatalDiagnosticError {
    pub code: ErrorCode,
    pub node: Box<dyn Node>,
    pub diagnostic_message: DiagnosticMessageChain,
    pub related_information: Option<Vec<DiagnosticRelatedInformation>>,
}

impl FatalDiagnosticError {
    pub fn new(
        code: ErrorCode,
        node: Box<dyn Node>,
        diagnostic_message: impl Into<DiagnosticMessageChain>,
        related_information: Option<Vec<DiagnosticRelatedInformation>>,
    ) -> Self {
        Self {
            code,
            node,
            diagnostic_message: diagnostic_message.into(),
            related_information,
        }
    }

    pub fn to_diagnostic(&self) -> DiagnosticWithLocation {
        make_diagnostic(
            self.code,
            &*self.node,
            self.diagnostic_message.clone(),
            self.related_information.clone(),
            DiagnosticCategory::Error,
        )
    }
}

impl fmt::Display for FatalDiagnosticError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FatalDiagnosticError: Code: {:?}, Message: {}",
            self.code, self.diagnostic_message
        )
    }
}

impl std::error::Error for FatalDiagnosticError {}

pub fn make_diagnostic(
    code: ErrorCode,
    node: &dyn Node,
    message_text: DiagnosticMessageChain,
    related_information: Option<Vec<DiagnosticRelatedInformation>>,
    category: DiagnosticCategory,
) -> DiagnosticWithLocation {
    let source_file = node.get_source_file();
    DiagnosticWithLocation {
        category,
        code: ng_error_code(code),
        file: source_file.map(|sf| sf.file_name().to_string()),
        start: node.get_start(source_file),
        length: node.get_width(source_file),
        message_text,
        related_information,
    }
}

pub fn make_diagnostic_chain(
    message_text: String,
    next: Option<Vec<DiagnosticMessageChain>>,
) -> DiagnosticMessageChain {
    ts_make_diagnostic_chain(message_text, next)
}

pub fn make_related_information(
    node: &dyn Node,
    message_text: String,
) -> DiagnosticRelatedInformation {
    let source_file = node.get_source_file();
    DiagnosticRelatedInformation {
        category: DiagnosticCategory::Message,
        code: 0,
        file: source_file.map(|sf| sf.file_name().to_string()),
        start: Some(node.get_start(source_file)),
        length: Some(node.get_width(source_file)),
        message_text,
    }
}

pub fn add_diagnostic_chain(
    message_text: DiagnosticMessageChain,
    add: Vec<DiagnosticMessageChain>,
) -> DiagnosticMessageChain {
    ts::add_diagnostic_chain(message_text, add)
}

pub fn is_fatal_diagnostic_error(err: &(dyn std::error::Error + 'static)) -> bool {
    err.is::<FatalDiagnosticError>()
}

// Temporary implementation for dyn Error downcasting check
impl FatalDiagnosticError {
    pub fn is_fatal(err: &(dyn std::error::Error + 'static)) -> bool {
        err.downcast_ref::<FatalDiagnosticError>().is_some()
    }
}
