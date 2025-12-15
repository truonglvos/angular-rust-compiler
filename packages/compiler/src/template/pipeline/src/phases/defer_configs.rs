//! Defer Configs Phase
//!
//! Corresponds to packages/compiler/src/template/pipeline/src/phases/defer_configs.ts
//! Defer instructions take a configuration array, which should be collected into the component
//! consts. This phase finds the config options, and creates the corresponding const array.

use crate::template::pipeline::ir;
use crate::template::pipeline::ir::enums::OpKind;
use crate::template::pipeline::ir::ops::create::DeferOp;
use crate::template::pipeline::ir::expression::ConstCollectedExpr;
use crate::template::pipeline::src::compilation::{ComponentCompilationJob, CompilationUnit};
use crate::output::output_ast::Expression;

/// Defer instructions take a configuration array, which should be collected into the component
/// consts. This phase finds the config options, and creates the corresponding const array.
pub fn configure_defer_instructions(job: &mut ComponentCompilationJob) {
    // Process root view
    {
        let unit = &mut job.root;
        process_unit(unit);
    }
    
    // Process all other views
    for (_, unit) in job.views.iter_mut() {
        process_unit(unit);
    }
}

fn process_unit(unit: &mut crate::template::pipeline::src::compilation::ViewCompilationUnit) {
    for op in unit.create_mut().iter_mut() {
        if op.kind() != OpKind::Defer {
            continue;
        }
        
        unsafe {
            let op_ptr = op.as_mut() as *mut dyn ir::CreateOp;
            let defer_ptr = op_ptr as *mut DeferOp;
            let defer = &mut *defer_ptr;
            
            // Create placeholder config if placeholder_minimum_time is set
            if let Some(min_time) = defer.placeholder_minimum_time {
                let literal_array = create_literal_array_from_values(&[min_time]);
                defer.placeholder_config = Some(Expression::ConstCollected(ConstCollectedExpr::new(
                    Box::new(literal_array),
                )));
            }
            
            // Create loading config if loading_minimum_time or loading_after_time is set
            if defer.loading_minimum_time.is_some() || defer.loading_after_time.is_some() {
                let mut values = Vec::new();
                if let Some(min_time) = defer.loading_minimum_time {
                    values.push(min_time);
                }
                if let Some(after_time) = defer.loading_after_time {
                    values.push(after_time);
                }
                let literal_array = create_literal_array_from_values(&values);
                defer.loading_config = Some(Expression::ConstCollected(ConstCollectedExpr::new(
                    Box::new(literal_array),
                )));
            }
        }
    }
}

/// Create a literal array expression from an array of f64 values
fn create_literal_array_from_values(values: &[f64]) -> Expression {
    use crate::output::output_ast::{LiteralArrayExpr, LiteralExpr, LiteralValue};
    
    let entries: Vec<Expression> = values
        .iter()
        .map(|&val| {
            Expression::Literal(LiteralExpr {
                value: LiteralValue::Number(val),
                type_: None,
                source_span: None,
            })
        })
        .collect();
    
    Expression::LiteralArray(LiteralArrayExpr {
        entries,
        type_: None,
        source_span: None,
    })
}

