// Segment Marker
//
// Represents a position in a source map.

/// Segment marker for source map positions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SegmentMarker {
    /// Line number (0-indexed).
    pub line: u32,
    /// Column number (0-indexed).
    pub column: u32,
    /// Source index.
    pub source_index: Option<u32>,
    /// Name index.
    pub name_index: Option<u32>,
}

impl SegmentMarker {
    pub fn new(line: u32, column: u32) -> Self {
        Self {
            line,
            column,
            source_index: None,
            name_index: None,
        }
    }
    
    pub fn with_source(mut self, source_index: u32) -> Self {
        self.source_index = Some(source_index);
        self
    }
    
    pub fn with_name(mut self, name_index: u32) -> Self {
        self.name_index = Some(name_index);
        self
    }
}

/// Compare two segment markers.
pub fn compare_markers(a: &SegmentMarker, b: &SegmentMarker) -> std::cmp::Ordering {
    match a.line.cmp(&b.line) {
        std::cmp::Ordering::Equal => a.column.cmp(&b.column),
        other => other,
    }
}
