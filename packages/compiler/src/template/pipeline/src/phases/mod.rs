//! Pipeline Phases Module
//!
//! Corresponds to packages/compiler/src/template/pipeline/src/phases/
//! Contains 60+ transformation phases

pub mod any_cast;
pub mod apply_i18n_expressions;
pub mod assign_i18n_slot_dependencies;
pub mod attach_source_locations;
pub mod attribute_extraction;
pub mod binding_specialization;
pub mod chaining;
pub mod collapse_singleton_interpolations;
pub mod conditionals;
pub mod const_collection;
pub mod convert_animations;
pub mod defer_configs;
pub mod defer_resolve_targets;
pub mod resolve_defer_deps_fns;
pub mod remove_content_selectors;
pub mod remove_empty_bindings;
pub mod deduplicate_text_bindings;
pub mod ng_container;
pub mod namespace;
pub mod nonbindable;
pub mod empty_elements;
pub mod generate_local_let_references;
pub mod has_const_expression_collection;
pub mod remove_illegal_let_references;
pub mod expand_safe_reads;
pub mod store_let_optimization;
pub mod track_variables;
pub mod resolve_dollar_event;
pub mod resolve_contexts;
pub mod resolve_names;
pub mod resolve_sanitizers;
pub mod save_restore_view;
pub mod local_refs;
pub mod generate_advance;
pub mod generate_projection_def;
pub mod generate_variables;
pub mod host_style_property_parsing;
pub mod temporary_variables;
pub mod var_counting;
pub mod variable_optimization;

pub mod naming;
pub mod next_context_merging;
pub mod ordering;
pub mod parse_extracted_styles;
pub mod reify;
pub mod pipe_creation;
pub mod pipe_variadic;
pub mod pure_function_extraction;
pub mod pure_literal_structures;
pub mod slot_allocation;
pub mod style_binding_specialization;
pub mod strip_nonrequired_parentheses;
pub mod track_fn_optimization;
pub mod transform_two_way_binding_set;
pub mod wrap_icus;
pub mod propagate_i18n_blocks;
pub mod remove_unused_i18n_attrs;
pub mod remove_i18n_contexts;
pub mod resolve_i18n_expression_placeholders;
pub mod create_i18n_contexts;
pub mod convert_i18n_bindings;
pub mod i18n_text_extraction;
pub mod extract_i18n_messages;
pub mod resolve_i18n_element_placeholders;
pub mod i18n_const_collection;

