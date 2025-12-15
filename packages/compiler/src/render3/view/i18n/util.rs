//! i18n Utilities
//!
//! Corresponds to packages/compiler/src/render3/view/i18n/util.ts
//! Contains i18n utility functions

use std::collections::HashMap;

use crate::i18n::i18n_ast as i18n;
use crate::i18n::serializers::xmb::to_public_name;
use crate::ml_parser::ast as html;
use crate::output::output_ast::{Expression, LiteralExpr, LiteralValue};

/// Name of the i18n attributes
pub const I18N_ATTR: &str = "i18n";
pub const I18N_ATTR_PREFIX: &str = "i18n-";

/// Prefix of var expressions used in ICUs
pub const I18N_ICU_VAR_PREFIX: &str = "VAR_";

/// Check if an attribute name is an i18n attribute
pub fn is_i18n_attribute(name: &str) -> bool {
    name == I18N_ATTR || name.starts_with(I18N_ATTR_PREFIX)
}

/// Check if an element has i18n attributes
pub fn has_i18n_attrs(node: &html::Element) -> bool {
    node.attrs.iter().any(|attr| is_i18n_attribute(&attr.name))
}

/// Get ICU from i18n message
pub fn icu_from_i18n_message(message: &i18n::Message) -> Option<&i18n::IcuPlaceholder> {
    if let Some(i18n::Node::IcuPlaceholder(icu)) = message.nodes.first() {
        Some(icu)
    } else {
        None
    }
}

/// Convert placeholders map to params
pub fn placeholders_to_params(placeholders: &HashMap<String, Vec<String>>) -> HashMap<String, Expression> {
    let mut params = HashMap::new();
    for (key, values) in placeholders {
        let value = if values.len() > 1 {
            format!("[{}]", values.join("|"))
        } else {
            values.first().cloned().unwrap_or_default()
        };
        params.insert(
            key.clone(),
            Expression::Literal(LiteralExpr {
                value: LiteralValue::String(value),
                type_: None,
                source_span: None,
            }),
        );
    }
    params
}

/// Format i18n placeholder names in a map.
///
/// The placeholder names are converted from "internal" format (e.g. `START_TAG_DIV_1`)
/// to "external" format (e.g. `startTagDiv_1`).
pub fn format_i18n_placeholder_names_in_map(
    params: &HashMap<String, Expression>,
    use_camel_case: bool,
) -> HashMap<String, Expression> {
    let mut result = HashMap::new();
    for (key, value) in params {
        result.insert(format_i18n_placeholder_name(key, use_camel_case), value.clone());
    }
    result
}

/// Converts internal placeholder names to public-facing format.
///
/// Example: `START_TAG_DIV_1` is converted to `startTagDiv_1`.
pub fn format_i18n_placeholder_name(name: &str, use_camel_case: bool) -> String {
    let public_name = to_public_name(name);
    
    if !use_camel_case {
        return public_name;
    }
    
    let chunks: Vec<&str> = public_name.split('_').collect();
    if chunks.len() == 1 {
        // if no "_" found - just lowercase the value
        return name.to_lowercase();
    }
    
    let mut chunks = chunks.into_iter().map(String::from).collect::<Vec<_>>();
    
    // eject last element if it's a number
    let postfix = if chunks.last().map_or(false, |s| s.chars().all(|c| c.is_ascii_digit())) {
        chunks.pop()
    } else {
        None
    };
    
    let mut raw = chunks.remove(0).to_lowercase();
    
    if !chunks.is_empty() {
        for chunk in chunks {
            if let Some(first_char) = chunk.chars().next() {
                let rest: String = chunk.chars().skip(1).collect();
                raw.push_str(&format!(
                    "{}{}",
                    first_char.to_uppercase(),
                    rest.to_lowercase()
                ));
            }
        }
    }
    
    if let Some(p) = postfix {
        format!("{}_{}", raw, p)
    } else {
        raw
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_i18n_placeholder_name() {
        assert_eq!(format_i18n_placeholder_name("START_TAG_DIV", true), "startTagDiv");
        assert_eq!(format_i18n_placeholder_name("START_TAG_DIV_1", true), "startTagDiv_1");
        assert_eq!(format_i18n_placeholder_name("INTERPOLATION", true), "interpolation");
        assert_eq!(format_i18n_placeholder_name("CLOSE_TAG_SPAN", true), "closeTagSpan");
    }

    #[test]
    fn test_is_i18n_attribute() {
        assert!(is_i18n_attribute("i18n"));
        assert!(is_i18n_attribute("i18n-title"));
        assert!(is_i18n_attribute("i18n-placeholder"));
        assert!(!is_i18n_attribute("class"));
        assert!(!is_i18n_attribute("i18"));
    }
}

