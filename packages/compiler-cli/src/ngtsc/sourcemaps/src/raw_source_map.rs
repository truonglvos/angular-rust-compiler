// Raw Source Map
//
// Raw source map data structure.

/// Raw source map (version 3).
#[derive(Debug, Clone)]
pub struct SourceMap {
    pub version: u32,
    pub file: String,
    pub source_root: Option<String>,
    pub sources: Vec<String>,
    pub sources_content: Option<Vec<Option<String>>>,
    pub names: Vec<String>,
    pub mappings: String,
}

impl SourceMap {
    pub fn new(file: impl Into<String>) -> Self {
        Self {
            version: 3,
            file: file.into(),
            source_root: None,
            sources: Vec::new(),
            sources_content: None,
            names: Vec::new(),
            mappings: String::new(),
        }
    }
}

/// VLQ encoding for source maps.
pub fn encode_vlq(mut value: i32) -> String {
    const BASE64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut result = String::new();
    let negative = value < 0;

    if negative {
        value = -value;
    }

    // Add sign bit to first 5 bits
    let mut first_digit = (value & 0xF) << 1;
    if negative {
        first_digit |= 1;
    }
    value >>= 4;

    if value > 0 {
        first_digit |= 0x20; // continuation bit
    }
    result.push(BASE64_CHARS[first_digit as usize] as char);

    while value > 0 {
        let mut digit = value & 0x1F;
        value >>= 5;
        if value > 0 {
            digit |= 0x20;
        }
        result.push(BASE64_CHARS[digit as usize] as char);
    }

    result
}
