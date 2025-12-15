//! Render3 HMR Compiler
//!
//! Corresponds to packages/compiler/src/render3/r3_hmr_compiler.ts
//! Contains Hot Module Replacement (HMR) compilation

use crate::output::output_ast::{
    Expression, Statement, ArrowFunctionExpr, ArrowFunctionBody, InvokeFunctionExpr, FnParam,
    LiteralExpr, LiteralValue, LiteralArrayExpr, DynamicImportExpr, ReadPropExpr, ReadVarExpr,
    DeclareVarStmt, DeclareFunctionStmt, ExternalExpr, ExternalReference, StmtModifier,
    BinaryOperatorExpr, BinaryOperator, WritePropExpr,
};
use crate::output::output_ast::dynamic_type;

use super::r3_identifiers::Identifiers as R3;
use super::util::dev_only_guarded_expression;

/// Helper to create external expression from ExternalReference
fn external_expr(reference: ExternalReference) -> Expression {
    Expression::External(ExternalExpr {
        value: reference,
        type_: None,
        source_span: None,
    })
}

/// Simple URI encoding helper (replaces urlencoding crate)
fn encode_uri_component(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '!' | '~' | '*' | '\'' | '(' | ')' => c.to_string(),
            c => c.encode_utf8(&mut [0; 4]).bytes()
                .map(|b| format!("%{:02X}", b))
                .collect::<String>(),
        })
        .collect()
}

/// Metadata necessary to compile HMR-related code
#[derive(Debug, Clone)]
pub struct R3HmrMetadata {
    /// Component class for which HMR is being enabled
    pub type_: Expression,
    /// Name of the component class
    pub class_name: String,
    /// File path of the component class
    pub file_path: String,
    /// Namespace dependencies (e.g. import * as i0 from '@angular/core')
    pub namespace_dependencies: Vec<R3HmrNamespaceDependency>,
    /// Local dependencies that need to be passed to the update callback
    pub local_dependencies: Vec<R3HmrLocalDependency>,
}

/// HMR dependency on a namespace import
#[derive(Debug, Clone)]
pub struct R3HmrNamespaceDependency {
    /// Module name of the import
    pub module_name: String,
    /// Name under which to refer to the namespace inside HMR-related code
    pub assigned_name: String,
}

/// Local dependency for HMR
#[derive(Debug, Clone)]
pub struct R3HmrLocalDependency {
    pub name: String,
    pub runtime_representation: Expression,
}

/// Compiles the expression that initializes HMR for a class
pub fn compile_hmr_initializer(meta: &R3HmrMetadata) -> Expression {
    let module_name = "m";
    let data_name = "d";
    let timestamp_name = "t";
    let id_name = "id";
    let import_callback_name = format!("{}_HmrLoad", meta.class_name);
    
    let namespaces: Vec<Expression> = meta.namespace_dependencies
        .iter()
        .map(|dep| {
            Expression::External(ExternalExpr {
                value: ExternalReference {
                    module_name: Some(dep.module_name.clone()),
                    name: None,
                    runtime: None,
                },
                type_: None,
                source_span: None,
            })
        })
        .collect();

    // m.default
    let default_read = Expression::ReadProp(ReadPropExpr {
        receiver: Box::new(Expression::ReadVar(ReadVarExpr {
            name: module_name.to_string(),
            type_: None,
            source_span: None,
        })),
        name: "default".to_string(),
        type_: None,
        source_span: None,
    });

    // Build locals array
    let locals_arr: Vec<Expression> = meta.local_dependencies
        .iter()
        .map(|l| l.runtime_representation.clone())
        .collect();

    // ɵɵreplaceMetadata(Comp, m.default, [...namespaces], [...locals], import.meta, id)
    let replace_metadata_ref = R3::replace_metadata();
    let replace_metadata_expr = external_expr(replace_metadata_ref);
    
    let replace_call = Expression::InvokeFn(InvokeFunctionExpr {
        fn_: Box::new(replace_metadata_expr),
        args: vec![
            meta.type_.clone(),
            default_read.clone(),
            Expression::LiteralArray(LiteralArrayExpr {
                entries: namespaces,
                type_: None,
                source_span: None,
            }),
            Expression::LiteralArray(LiteralArrayExpr {
                entries: locals_arr,
                type_: None,
                source_span: None,
            }),
            Expression::ReadProp(ReadPropExpr {
                receiver: Box::new(Expression::ReadVar(ReadVarExpr {
                    name: "import".to_string(),
                    type_: None,
                    source_span: None,
                })),
                name: "meta".to_string(),
                type_: None,
                source_span: None,
            }),
            Expression::ReadVar(ReadVarExpr {
                name: id_name.to_string(),
                type_: None,
                source_span: None,
            }),
        ],
        type_: None,
        source_span: None,
        pure: false,
    });

    // (m) => m.default && ɵɵreplaceMetadata(...)
    let replace_callback = Expression::ArrowFn(ArrowFunctionExpr {
        params: vec![FnParam {
            name: module_name.to_string(),
            type_: None,
        }],
        body: ArrowFunctionBody::Expression(Box::new(
            Expression::BinaryOp(BinaryOperatorExpr {
                operator: BinaryOperator::And,
                lhs: Box::new(default_read.clone()),
                rhs: Box::new(replace_call),
                type_: None,
                source_span: None,
            })
        )),
        type_: None,
        source_span: None,
    });

    // getReplaceMetadataURL(id, timestamp, import.meta.url)
    let get_replace_metadata_url_ref = R3::get_replace_metadata_url();
    let get_replace_metadata_url_expr = external_expr(get_replace_metadata_url_ref);
    
    let _url_expr = Expression::InvokeFn(InvokeFunctionExpr {
        fn_: Box::new(get_replace_metadata_url_expr),
        args: vec![
            Expression::ReadVar(ReadVarExpr {
                name: id_name.to_string(),
                type_: None,
                source_span: None,
            }),
            Expression::ReadVar(ReadVarExpr {
                name: timestamp_name.to_string(),
                type_: None,
                source_span: None,
            }),
            Expression::ReadProp(ReadPropExpr {
                receiver: Box::new(Expression::ReadProp(ReadPropExpr {
                    receiver: Box::new(Expression::ReadVar(ReadVarExpr {
                        name: "import".to_string(),
                        type_: None,
                        source_span: None,
                    })),
                    name: "meta".to_string(),
                    type_: None,
                    source_span: None,
                })),
                name: "url".to_string(),
                type_: None,
                source_span: None,
            }),
        ],
        type_: None,
        source_span: None,
        pure: false,
    });

    // import(url).then(replaceCallback)
    // Note: DynamicImportExpr in Rust expects a String, but TypeScript accepts an Expression
    // We need to use the url_expr as a template string or evaluate it
    // For now, we'll create a DynamicImportExpr with a placeholder and handle the expression separately
    // The comment '@vite-ignore' would be handled by the emitter
    let dynamic_import = Expression::DynamicImport(DynamicImportExpr {
        url: "/* @vite-ignore */ import(url)".to_string(), // This will need proper stringification of url_expr
        source_span: None,
    });
    
    let import_then = Expression::InvokeFn(InvokeFunctionExpr {
        fn_: Box::new(Expression::ReadProp(ReadPropExpr {
            receiver: Box::new(dynamic_import),
            name: "then".to_string(),
            type_: None,
            source_span: None,
        })),
        args: vec![replace_callback],
        type_: None,
        source_span: None,
        pure: false,
    });

    // function Cmp_HmrLoad(t) { import(...).then(...); }
    let import_callback = Statement::DeclareFn(DeclareFunctionStmt {
        name: import_callback_name.clone(),
        params: vec![FnParam {
            name: timestamp_name.to_string(),
            type_: None,
        }],
        statements: vec![import_then.to_stmt()],
        type_: None,
        modifiers: StmtModifier::Final,
        source_span: None,
    });

    // (d) => d.id === id && Cmp_HmrLoad(d.timestamp)
    let d_id = Expression::ReadProp(ReadPropExpr {
        receiver: Box::new(Expression::ReadVar(ReadVarExpr {
            name: data_name.to_string(),
            type_: None,
            source_span: None,
        })),
        name: "id".to_string(),
        type_: None,
        source_span: None,
    });
    let d_timestamp = Expression::ReadProp(ReadPropExpr {
        receiver: Box::new(Expression::ReadVar(ReadVarExpr {
            name: data_name.to_string(),
            type_: None,
            source_span: None,
        })),
        name: "timestamp".to_string(),
        type_: None,
        source_span: None,
    });
    let hmr_load_call = Expression::InvokeFn(InvokeFunctionExpr {
        fn_: Box::new(Expression::ReadVar(ReadVarExpr {
            name: import_callback_name.clone(),
            type_: None,
            source_span: None,
        })),
        args: vec![d_timestamp],
        type_: None,
        source_span: None,
        pure: false,
    });
    
    let update_callback = Expression::ArrowFn(ArrowFunctionExpr {
        params: vec![FnParam {
            name: data_name.to_string(),
            type_: None,
        }],
        body: ArrowFunctionBody::Expression(Box::new(
            Expression::BinaryOp(BinaryOperatorExpr {
                operator: BinaryOperator::And,
                lhs: Box::new(Expression::BinaryOp(BinaryOperatorExpr {
                    operator: BinaryOperator::Identical,
                    lhs: Box::new(d_id),
                    rhs: Box::new(Expression::ReadVar(ReadVarExpr {
                        name: id_name.to_string(),
                        type_: None,
                        source_span: None,
                    })),
                    type_: None,
                    source_span: None,
                })),
                rhs: Box::new(hmr_load_call),
                type_: None,
                source_span: None,
            })
        )),
        type_: None,
        source_span: None,
    });

    // Cmp_HmrLoad(Date.now())
    let date_now = Expression::InvokeFn(InvokeFunctionExpr {
        fn_: Box::new(Expression::ReadProp(ReadPropExpr {
            receiver: Box::new(Expression::ReadVar(ReadVarExpr {
                name: "Date".to_string(),
                type_: None,
                source_span: None,
            })),
            name: "now".to_string(),
            type_: None,
            source_span: None,
        })),
        args: vec![],
        type_: None,
        source_span: None,
        pure: false,
    });
    let initial_call = Expression::InvokeFn(InvokeFunctionExpr {
        fn_: Box::new(Expression::ReadVar(ReadVarExpr {
            name: import_callback_name.clone(),
            type_: None,
            source_span: None,
        })),
        args: vec![date_now],
        type_: None,
        source_span: None,
        pure: false,
    });

    // import.meta.hot
    let hot_read = Expression::ReadProp(ReadPropExpr {
        receiver: Box::new(Expression::ReadProp(ReadPropExpr {
            receiver: Box::new(Expression::ReadVar(ReadVarExpr {
                name: "import".to_string(),
                type_: None,
                source_span: None,
            })),
            name: "meta".to_string(),
            type_: None,
            source_span: None,
        })),
        name: "hot".to_string(),
        type_: None,
        source_span: None,
    });

    // import.meta.hot.on('angular:component-update', updateCallback)
    let hot_listener = Expression::InvokeFn(InvokeFunctionExpr {
        fn_: Box::new(Expression::ReadProp(ReadPropExpr {
            receiver: Box::new(hot_read.clone()),
            name: "on".to_string(),
            type_: None,
            source_span: None,
        })),
        args: vec![
            Expression::Literal(LiteralExpr {
                value: LiteralValue::String("angular:component-update".to_string()),
                type_: None,
                source_span: None,
            }),
            update_callback,
        ],
        type_: None,
        source_span: None,
        pure: false,
    });

    // Encode ID
    let encoded_id = encode_uri_component(&format!("{}@{}", meta.file_path, meta.class_name));

    // Build the IIFE
    let iife_body: Vec<Statement> = vec![
        // const id = <encoded_id>
        Statement::DeclareVar(DeclareVarStmt {
            name: id_name.to_string(),
            value: Some(Box::new(Expression::Literal(LiteralExpr {
                value: LiteralValue::String(encoded_id),
                type_: None,
                source_span: None,
            }))),
            type_: None,
            modifiers: StmtModifier::Final,
            source_span: None,
        }),
        // function Cmp_HmrLoad() {...}
        import_callback,
        // ngDevMode && Cmp_HmrLoad(Date.now())
        dev_only_guarded_expression(initial_call).to_stmt(),
        // ngDevMode && import.meta.hot && import.meta.hot.on(...)
        dev_only_guarded_expression(
            Expression::BinaryOp(BinaryOperatorExpr {
                operator: BinaryOperator::And,
                lhs: Box::new(hot_read),
                rhs: Box::new(hot_listener),
                type_: None,
                source_span: None,
            })
        ).to_stmt(),
    ];

    let iife = Expression::ArrowFn(ArrowFunctionExpr {
        params: vec![],
        body: ArrowFunctionBody::Statements(iife_body),
        type_: None,
        source_span: None,
    });

    Expression::InvokeFn(InvokeFunctionExpr {
        fn_: Box::new(iife),
        args: vec![],
        type_: None,
        source_span: None,
        pure: false,
    })
}

/// Definition for HMR update callback
#[derive(Debug, Clone)]
pub struct HmrDefinition {
    pub name: String,
    pub initializer: Option<Expression>,
    pub statements: Vec<Statement>,
}

/// Compiles the HMR update callback for a class
pub fn compile_hmr_update_callback(
    definitions: &[HmrDefinition],
    constant_statements: &[Statement],
    meta: &R3HmrMetadata,
) -> DeclareFunctionStmt {
    let namespaces = "ɵɵnamespaces";
    let mut params = vec![
        FnParam {
            name: meta.class_name.clone(),
            type_: Some(dynamic_type()),
        },
        FnParam {
            name: namespaces.to_string(),
            type_: Some(dynamic_type()),
        },
    ];

    for local in &meta.local_dependencies {
        params.push(FnParam {
            name: local.name.clone(),
            type_: None,
        });
    }

    let mut body: Vec<Statement> = vec![];

    // Declare variables that read out the individual namespaces
    for (i, dep) in meta.namespace_dependencies.iter().enumerate() {
        body.push(Statement::DeclareVar(DeclareVarStmt {
            name: dep.assigned_name.clone(),
            value: Some(Box::new(Expression::ReadProp(ReadPropExpr {
                receiver: Box::new(Expression::ReadVar(ReadVarExpr {
                    name: namespaces.to_string(),
                    type_: None,
                    source_span: None,
                })),
                name: i.to_string(),
                type_: None,
                source_span: None,
            }))),
            type_: Some(dynamic_type()),
            modifiers: StmtModifier::Final,
            source_span: None,
        }));
    }

    body.extend(constant_statements.iter().cloned());

    for field in definitions {
        if let Some(ref initializer) = field.initializer {
            // Comp.fieldName = initializer
            let assignment = Expression::WriteProp(WritePropExpr {
                receiver: Box::new(Expression::ReadVar(ReadVarExpr {
                    name: meta.class_name.clone(),
                    type_: None,
                    source_span: None,
                })),
                name: field.name.clone(),
                value: Box::new(initializer.clone()),
                type_: None,
                source_span: None,
            });
            body.push(assignment.to_stmt());

            for stmt in &field.statements {
                body.push(stmt.clone());
            }
        }
    }

    DeclareFunctionStmt {
        name: format!("{}_UpdateMetadata", meta.class_name),
        params,
        statements: body,
        type_: None,
        modifiers: StmtModifier::Final,
        source_span: None,
    }
}
