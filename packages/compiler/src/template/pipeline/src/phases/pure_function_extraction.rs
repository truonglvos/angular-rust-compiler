//! Extract pure functions into shared constants.
//!
//! Corresponds to packages/compiler/src/template/pipeline/src/phases/pure_function_extraction.ts

use crate::constant_pool::{GenericKeyFn, SharedConstantDefinition};
use crate::output::output_ast as o;
use crate::template::pipeline::ir;
use crate::template::pipeline::src::compilation::{CompilationJob, ComponentCompilationJob, CompilationUnit};

pub fn extract_pure_functions(job: &mut dyn CompilationJob) {
    // Rust borrowck: split job borrow
    // We need access to job.pool (mutable) and job.units (mutable).
    // CompilationJob trait getters usually return refs.
    // We'll use the unsafe cast common pattern to get mutable component job.
    
    if let Some(component_job) = unsafe {
        let job_ptr = job as *mut dyn CompilationJob;
        if job.kind() == crate::template::pipeline::src::compilation::CompilationJobKind::Tmpl {
            Some(&mut *(job_ptr as *mut ComponentCompilationJob))
        } else {
            None
        }
    } {
        for unit in component_job.views.values_mut() {
            fn process_op(op: &mut dyn ir::Op) {
                 ir::transform_expressions_in_op(op, &mut |expr, _flags| {
                     if let Some(ir_expr) = ir::as_ir_expression(&expr) {
                        if let ir::IRExpression::PureFunctionParameter(param) = ir_expr {
                            return *o::variable(format!("a{}", param.index));
                        }
                     }
                     expr.clone()
                 }, ir::VisitorContextFlag::NONE);
            }

            for op in unit.create_mut().iter_mut() {
                process_op(op.as_mut());
            }
            for op in unit.update_mut().iter_mut() {
                process_op(op.as_mut());
            }
        }
    }
}

struct PureFunctionConstant {
    num_args: usize,
    base: GenericKeyFn,
}

impl PureFunctionConstant {
    fn new(num_args: usize) -> Self {
        PureFunctionConstant {
            num_args,
            base: GenericKeyFn,
        }
    }
}

impl SharedConstantDefinition for PureFunctionConstant {
    fn key_of(&self, expr: &o::Expression) -> String {
        if let Some(ir_expr) = ir::as_ir_expression(expr) {
            if let ir::IRExpression::PureFunctionParameter(param) = ir_expr {
                return format!("param({})", param.index);
            }
        }
        self.base.key_of(expr)
    }

    fn to_shared_constant_declaration(&self, decl_name: String, key_expr: o::Expression) -> o::Statement {
        let mut fn_params = Vec::new();
        for idx in 0..self.num_args {
            fn_params.push(o::FnParam { name: format!("a{}", idx), type_: None });
        }

        let return_expr = ir::transform_expressions_in_expression(
            key_expr,
            &mut |expr, _flags| {
                if let Some(ir_expr) = ir::as_ir_expression(&expr) {
                    if let ir::IRExpression::PureFunctionParameter(param) = ir_expr {
                        return *o::variable(format!("a{}", param.index));
                    }
                }
                expr.clone()
            },
            ir::VisitorContextFlag::NONE,
        );

        o::Statement::DeclareVar(o::DeclareVarStmt {
            name: decl_name,
            value: Some(o::arrow_fn(fn_params, o::ArrowFunctionBody::Expression(Box::new(return_expr)), None)),
            type_: None,
            modifiers: o::StmtModifier::None,
            source_span: None,
        })
    }
}
