//! Helper functions for host binding ingestion
//!
//! These functions are used to process host bindings (properties, attributes, events)
//! for components and directives.

use crate::core::SecurityContext;
use crate::expression_parser::ast::{ParsedEvent, ParsedProperty};
use crate::output::output_ast::Expression;
use crate::parse_util::ParseSourceSpan;
use crate::template::pipeline::ir as ir;
use crate::template::pipeline::ir::ops::update::BindingExpression;
use crate::template::pipeline::src::compilation::HostBindingCompilationJob;
use crate::template::pipeline::src::conversion::convert_ast;

/// Ingest a DOM property binding for host bindings
pub fn ingest_dom_property(
    job: &mut HostBindingCompilationJob,
    property: ParsedProperty,
    binding_kind: ir::BindingKind,
    security_contexts: Vec<SecurityContext>,
) {
    use crate::expression_parser::ast::AST as ExprAST;
    
    // Convert expression - handle interpolation if present
    let expression = match &property.expression.ast {
        ExprAST::Interpolation(interp) => {
            // Convert interpolation to IR Interpolation
            let exprs: Vec<Expression> = interp.expressions.iter().map(|expr| {
                convert_ast(expr, job, Some(&property.source_span))
            }).collect();
            
            // Create Interpolation expression
            BindingExpression::Interpolation(crate::template::pipeline::ir::Interpolation {
                strings: interp.strings.clone(),
                expressions: exprs,
                placeholders: vec![], // Host bindings don't have i18n placeholders
            })
        }
        ast => {
            // Convert regular AST to Expression
            BindingExpression::Expression(convert_ast(ast, job, Some(&property.source_span)))
        }
    };
    
    // Create binding op
    let binding_op = ir::ops::update::create_binding_op(
        job.root().xref(),
        binding_kind,
        property.name,
        expression,
        property.unit,
        security_contexts,
        false, // is_text_attr
        false, // is_structural_template_attribute
        None,  // template_kind
        None,  // i18n_message - host bindings don't handle i18n
        property.source_span,
    );
    
    job.root.create_mut().push(Box::new(ir::ops::update::UpdateBindingOp::from(binding_op)));
}

/// Ingest a host attribute binding
pub fn ingest_host_attribute(
    job: &mut HostBindingCompilationJob,
    name: String,
    value: Expression,
    security_contexts: Vec<SecurityContext>,
) {
    // Host attributes should always be extracted to const hostAttrs
    // Create binding op with is_text_attr = true
    let binding_op = ir::ops::update::create_binding_op(
        job.root().xref(),
        ir::BindingKind::Attribute,
        name,
        BindingExpression::Expression(value.clone()),
        None, // unit
        security_contexts,
        true, // is_text_attr - always true for host attributes
        false, // is_structural_template_attribute
        None,  // template_kind
        None,  // i18n_message
        value.source_span().cloned().unwrap_or_else(|| ParseSourceSpan::unknown()),
    );
    
    job.root.create_mut().push(Box::new(ir::ops::update::UpdateBindingOp::from(binding_op)));
}

/// Ingest a host event binding
pub fn ingest_host_event(
    job: &mut HostBindingCompilationJob,
    event: ParsedEvent,
) {
    use crate::expression_parser::ast::ParsedEventType;
    use crate::template::pipeline::ir::handle::SlotHandle;
    
    // Create handler ops
    let handler_ops = make_listener_handler_ops(job, &event.handler, &event.handler_span);
    
    match event.type_ {
        ParsedEventType::Animation => {
            // Determine animation kind based on event name
            let animation_kind = if event.name.ends_with("enter") {
                ir::enums::AnimationKind::Enter
            } else {
                ir::enums::AnimationKind::Leave
            };
            
            // Create animation listener op
            let animation_listener_op = ir::ops::create::create_animation_listener_op(
                job.root().xref(),
                SlotHandle::default(),
                event.name,
                None, // tag - host listeners don't have tags
                handler_ops,
                animation_kind,
                event.target, // event_target
                true, // host_listener
                event.source_span,
            );
            job.root.create_mut().push(animation_listener_op);
        }
        ParsedEventType::Regular => {
            // Create regular listener op
            let listener_op = ir::ops::create::create_listener_op(
                job.root().xref(),
                SlotHandle::default(),
                event.name,
                None, // tag
                handler_ops,
                None, // legacy_animation_phase
                event.target, // event_target
                true, // host_listener
                event.source_span,
            );
            job.root.create_mut().push(listener_op);
        }
    }
}

/// Helper function to convert event handler AST into UpdateOps for host bindings
fn make_listener_handler_ops(
    job: &mut HostBindingCompilationJob,
    handler: &crate::expression_parser::ast::AST,
    handler_span: &ParseSourceSpan,
) -> crate::template::pipeline::ir::operations::OpList<Box<dyn crate::template::pipeline::ir::operations::UpdateOp + Send + Sync>> {
    use crate::template::pipeline::ir::ops::shared::create_statement_op;
    use crate::output::output_ast::{Statement, ExpressionStatement, ReturnStatement};
    
    let mut handler_ops = crate::template::pipeline::ir::operations::OpList::new();
    
    // Convert handler AST to Expression
    let handler_expr = convert_ast(handler, job, Some(handler_span));
    
    // Extract return expression if present
    // For simplicity, treat the handler as a return statement
    let return_stmt = ReturnStatement {
        value: Box::new(handler_expr),
        source_span: Some(handler_span.clone()),
    };
    let stmt = Statement::Return(return_stmt);
    let stmt_op = create_statement_op::<Box<dyn crate::template::pipeline::ir::operations::UpdateOp + Send + Sync>>(Box::new(stmt));
    handler_ops.push(Box::new(stmt_op));
    
    handler_ops
}

