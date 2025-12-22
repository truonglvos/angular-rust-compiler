// Content Origin
//
// Tracks the origin of source map content.

/// Content origin type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentOrigin {
    /// Content from file.
    File(String),
    /// Inline content.
    Inline,
    /// Unknown origin.
    Unknown,
}
