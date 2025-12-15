//! Render3 Partial Compilation Utilities
//!
//! Corresponds to packages/compiler/src/render3/partial/util.ts
//! Contains utility functions for partial compilation

use crate::output::output_ast::{
    Expression, LiteralExpr, LiteralValue, LiteralArrayExpr, LiteralMapExpr, LiteralMapEntry,
};
use crate::render3::r3_factory::R3DependencyMetadata;
use crate::render3::view::util::DefinitionMap;

use super::api::R3DeclareDependencyMetadata;

/// Helper to create literal expression
fn literal(value: LiteralValue) -> Expression {
    Expression::Literal(LiteralExpr {
        value,
        type_: None,
        source_span: None,
    })
}

/// Creates an array literal expression from the given array, mapping all values to an expression.
/// Returns `None` if the array is empty or null.
pub fn to_optional_literal_array<T, F>(values: Option<&[T]>, mapper: F) -> Option<LiteralArrayExpr>
where
    F: Fn(&T) -> Expression,
{
    match values {
        None => None,
        Some(vals) if vals.is_empty() => None,
        Some(vals) => {
            let exprs: Vec<Expression> = vals.iter().map(mapper).collect();
            Some(LiteralArrayExpr {
                entries: exprs,
                type_: None,
                source_span: None,
            })
        }
    }
}

/// Creates an object literal expression from the given hashmap, mapping all values to an expression.
/// Returns `None` if the map is empty.
pub fn to_optional_literal_map<T, F>(
    object: &std::collections::HashMap<String, T>,
    mapper: F,
) -> Option<LiteralMapExpr>
where
    F: Fn(&T) -> Expression,
{
    if object.is_empty() {
        return None;
    }

    let entries: Vec<LiteralMapEntry> = object
        .iter()
        .map(|(key, value)| LiteralMapEntry {
            key: key.clone(),
            value: Box::new(mapper(value)),
            quoted: true,
        })
        .collect();

    Some(LiteralMapExpr {
        entries,
        type_: None,
        source_span: None,
    })
}

/// Compile dependencies for partial declarations
pub fn compile_dependencies(deps: &DepsValue) -> Expression {
    match deps {
        DepsValue::Invalid => literal(LiteralValue::String("invalid".to_string())),
        DepsValue::None => literal(LiteralValue::Null),
        DepsValue::Valid(deps) => {
            let exprs: Vec<Expression> = deps.iter().map(compile_dependency).collect();
            Expression::LiteralArray(LiteralArrayExpr {
                entries: exprs,
                type_: None,
                source_span: None,
            })
        }
    }
}

/// Dependencies value
#[derive(Debug, Clone)]
pub enum DepsValue {
    Valid(Vec<R3DependencyMetadata>),
    Invalid,
    None,
}

/// Compile a single dependency
pub fn compile_dependency(dep: &R3DependencyMetadata) -> Expression {
    let mut def_map = DefinitionMap::new();

    if let Some(ref token) = dep.token {
        def_map.set("token", Some(token.clone()));
    }

    if dep.attribute_name_type.is_some() {
        def_map.set("attribute", Some(literal(LiteralValue::Bool(true))));
    }

    if dep.host {
        def_map.set("host", Some(literal(LiteralValue::Bool(true))));
    }

    if dep.optional {
        def_map.set("optional", Some(literal(LiteralValue::Bool(true))));
    }

    if dep.self_ {
        def_map.set("self", Some(literal(LiteralValue::Bool(true))));
    }

    if dep.skip_self {
        def_map.set("skipSelf", Some(literal(LiteralValue::Bool(true))));
    }

    Expression::LiteralMap(def_map.to_literal_map())
}

/// Convert R3DeclareDependencyMetadata to literal map
pub fn compile_declare_dependency(dep: &R3DeclareDependencyMetadata) -> Expression {
    let mut def_map = DefinitionMap::new();

    if let Some(ref token) = dep.token {
        def_map.set("token", Some(token.clone()));
    }

    if dep.attribute {
        def_map.set("attribute", Some(literal(LiteralValue::Bool(true))));
    }

    if dep.host {
        def_map.set("host", Some(literal(LiteralValue::Bool(true))));
    }

    if dep.optional {
        def_map.set("optional", Some(literal(LiteralValue::Bool(true))));
    }

    if dep.self_ {
        def_map.set("self", Some(literal(LiteralValue::Bool(true))));
    }

    if dep.skip_self {
        def_map.set("skipSelf", Some(literal(LiteralValue::Bool(true))));
    }

    Expression::LiteralMap(def_map.to_literal_map())
}
