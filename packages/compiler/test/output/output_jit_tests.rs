use angular_compiler::output::output_ast as o;
use angular_compiler::output::output_jit::{JitEmitterVisitor, ExternalReferenceResolver};
use angular_compiler::output::abstract_emitter::EmitterVisitorContext;

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::Any;

    // Mock resolver for tests
    struct MockResolver;

    impl ExternalReferenceResolver for MockResolver {
        fn resolve_external_reference(&self, _reference: &o::ExternalReference) -> Box<dyn Any> {
            Box::new(())
        }
    }

    // Helper to evaluate generated code
    // In Rust we can't easily eval JS. 
    // The TS test `output_jit_spec.ts` evaluates the generated code to verify logic.
    // e.g. `eval(ctx.toSource())(1, 2)`
    // Since we are generating JS string in Rust, we can only verify the generated STRING content.
    // We cannot execute it unless we use a JS runtime like embedding V8 or just asserting string output.
    // Given the constraints and typical Rust testing patterns, I will assert the generated source code string.
    
    // TS `output_jit_spec.ts` tests:
    // - run a statement
    // - run a function
    
    // I will convert evaluations to string assertions.

    #[test]
    fn should_support_literals() {
        let stmt = o::Statement::Return(o::ReturnStatement {
            value: o::literal(o::LiteralValue::Number(10.0)),
            source_span: None,
        });
        
        let mut ctx = EmitterVisitorContext::create_root();
        let resolver = MockResolver;
        let mut visitor = JitEmitterVisitor::new(&resolver);
        visitor.visit_all_statements(&[stmt], &mut ctx);
        
        assert_eq!(ctx.to_source().trim(), "return 10;");
    }

    #[test]
    fn should_support_binary_operators() {
        // return 1 + 2
        let stmt = o::Statement::Return(o::ReturnStatement {
            value: Box::new(o::Expression::BinaryOp(o::BinaryOperatorExpr {
                operator: o::BinaryOperator::Plus,
                lhs: o::literal(o::LiteralValue::Number(1.0)),
                rhs: o::literal(o::LiteralValue::Number(2.0)),
                type_: None,
                source_span: None,
            })),
            source_span: None,
        });
        
        let mut ctx = EmitterVisitorContext::create_root();
        let resolver = MockResolver;
        let mut visitor = JitEmitterVisitor::new(&resolver);
        visitor.visit_all_statements(&[stmt], &mut ctx);
        
        assert_eq!(ctx.to_source().trim(), "return 1 + 2;");
    }
    
    // Porting specific tests from output_jit_spec.ts
    // The TS tests define a `emitStmt` helper that wraps code in a function and evals it.
    // e.g. `expressions` test checks operator precedence.
    
    #[test]
    fn should_support_conditionals() {
        // return 1 >= 2 ? 1 : 2;
         let stmt = o::Statement::Return(o::ReturnStatement {
            value: Box::new(o::Expression::Conditional(o::ConditionalExpr {
                condition: Box::new(o::Expression::BinaryOp(o::BinaryOperatorExpr {
                    operator: o::BinaryOperator::BiggerEquals,
                    lhs: o::literal(o::LiteralValue::Number(1.0)),
                    rhs: o::literal(o::LiteralValue::Number(2.0)),
                    type_: None,
                    source_span: None,
                })),
                true_case: o::literal(o::LiteralValue::Number(1.0)),
                false_case: Some(o::literal(o::LiteralValue::Number(2.0))),
                type_: None,
                source_span: None,
            })),
            source_span: None,
        });
        
        let mut ctx = EmitterVisitorContext::create_root();
        let resolver = MockResolver;
        let mut visitor = JitEmitterVisitor::new(&resolver);
        visitor.visit_all_statements(&[stmt], &mut ctx);
        
        assert_eq!(ctx.to_source().trim(), "return (1 >= 2 ? 1 : 2);");
    }

    #[test]
    fn should_support_assignments() {
        // var a = 1; a = 2; return a;
        let stmts = vec![
            o::Statement::DeclareVar(o::DeclareVarStmt {
                name: "a".to_string(),
                value: Some(o::literal(o::LiteralValue::Number(1.0))),
                type_: None,
                modifiers: o::StmtModifier::None,
                source_span: None,
            }),
            o::Statement::Expression(o::ExpressionStatement {
                expr: Box::new(o::Expression::WriteVar(o::WriteVarExpr {
                    name: "a".to_string(),
                    value: o::literal(o::LiteralValue::Number(2.0)),
                    type_: None,
                    source_span: None,
                })),
                source_span: None,
            }),
             o::Statement::Return(o::ReturnStatement {
                value: Box::new(o::Expression::ReadVar(o::ReadVarExpr {
                    name: "a".to_string(),
                    type_: None,
                    source_span: None,
                })),
                source_span: None,
            }),
        ];

        let mut ctx = EmitterVisitorContext::create_root();
        let resolver = MockResolver;
        let mut visitor = JitEmitterVisitor::new(&resolver);
        visitor.visit_all_statements(&stmts, &mut ctx);

        assert_eq!(ctx.to_source().trim(), "var a = 1;\na = 2;\nreturn a;");
    }
}
