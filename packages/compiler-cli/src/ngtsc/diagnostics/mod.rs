pub mod src {
    pub mod docs;
    pub mod error;
    pub mod error_code;
    pub mod error_details_base_url;
    pub mod extended_template_diagnostic_name;
    pub mod util;
}

pub use src::docs::*;
pub use src::error::*;
pub use src::error_code::*;
pub use src::error_details_base_url::*;
pub use src::extended_template_diagnostic_name::*;
pub use src::util::*;
pub use ts::{
    Diagnostic, DiagnosticCategory, DiagnosticMessageChain, DiagnosticRelatedInformation,
    DiagnosticWithLocation, Node,
};

#[cfg(test)]
mod test;
