// JSDoc Extractor
//
// Extracts JSDoc comments and tags.

use super::entities::*;

/// Extracts JSDoc documentation.
pub struct JsDocExtractor;

impl JsDocExtractor {
    /// Parse JSDoc comment text.
    pub fn parse(comment: &str) -> (String, Vec<JsDocTag>) {
        let mut description = String::new();
        let mut tags = Vec::new();

        let lines: Vec<&str> = comment.lines().collect();
        let mut in_description = true;
        let mut current_tag: Option<(String, String)> = None;

        for line in lines {
            let trimmed = line.trim().trim_start_matches('*').trim();

            if trimmed.starts_with('@') {
                // Save previous tag if any
                if let Some((name, text)) = current_tag.take() {
                    tags.push(JsDocTag { name, text });
                }

                in_description = false;

                // Parse new tag
                if let Some(space_pos) = trimmed.find(char::is_whitespace) {
                    let tag_name = trimmed[1..space_pos].to_string();
                    let tag_text = trimmed[space_pos..].trim().to_string();
                    current_tag = Some((tag_name, tag_text));
                } else {
                    let tag_name = trimmed[1..].to_string();
                    current_tag = Some((tag_name, String::new()));
                }
            } else if in_description {
                if !description.is_empty() && !trimmed.is_empty() {
                    description.push(' ');
                }
                description.push_str(trimmed);
            } else if let Some((_, ref mut text)) = current_tag {
                if !trimmed.is_empty() {
                    if !text.is_empty() {
                        text.push(' ');
                    }
                    text.push_str(trimmed);
                }
            }
        }

        // Save last tag
        if let Some((name, text)) = current_tag {
            tags.push(JsDocTag { name, text });
        }

        (description, tags)
    }

    /// Get tag value by name.
    pub fn get_tag<'a>(tags: &'a [JsDocTag], name: &str) -> Option<&'a str> {
        tags.iter()
            .find(|t| t.name == name)
            .map(|t| t.text.as_str())
    }

    /// Check if tags include a specific tag.
    pub fn has_tag(tags: &[JsDocTag], name: &str) -> bool {
        tags.iter().any(|t| t.name == name)
    }

    /// Parse @param tag.
    pub fn parse_param_tag(text: &str) -> Option<(String, String)> {
        let mut parts = text.splitn(2, char::is_whitespace);
        let name = parts.next()?.to_string();
        let description = parts.next().unwrap_or("").to_string();
        Some((name, description))
    }
}
