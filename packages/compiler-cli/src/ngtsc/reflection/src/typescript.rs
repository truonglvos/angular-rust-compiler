use super::host::*;
use oxc_ast::ast as oxc;
use std::collections::HashMap;
use std::marker::PhantomData;

pub struct TypeScriptReflectionHost<'a> {
    phantom: PhantomData<&'a ()>,
}

impl<'a> TypeScriptReflectionHost<'a> {
    pub fn new() -> Self {
        Self { phantom: PhantomData }
    }

    fn convert_decorators(&self, oxc_decorators: &'a [oxc::Decorator<'a>]) -> Option<Vec<Decorator<'a>>> {
        if oxc_decorators.is_empty() {
             return None;
        }

        let mut decorators = Vec::new();
        for decorator in oxc_decorators {
            if let oxc::Expression::CallExpression(call_expr) = &decorator.expression {
                 let identifier = if let oxc::Expression::Identifier(ident) = &call_expr.callee {
                     Some(DecoratorIdentifier {
                         name: ident.name.to_string(),
                         module_name: None, 
                     })
                 } else {
                     None
                 };
                 
                 let name = identifier.as_ref().map(|id| id.name.clone()).unwrap_or_default();
                 
                 // Extract args
                 let args = call_expr.arguments.iter().filter_map(|arg| {
                     arg.as_expression()
                 }).collect::<Vec<_>>();

                 decorators.push(Decorator {
                     name,
                     identifier,
                     import: None, // Import resolution requires full TypeChecker
                     node: decorator,
                     args: Some(args),
                 });
            } else if let oxc::Expression::Identifier(ident) = &decorator.expression {
                 // @Decorator without parens
                 decorators.push(Decorator {
                     name: ident.name.to_string(),
                     identifier: Some(DecoratorIdentifier {
                         name: ident.name.to_string(),
                         module_name: None, 
                     }),
                     import: None,
                     node: decorator,
                     args: None,
                 });
            }
        }
        
        if decorators.is_empty() {
            None
        } else {
            Some(decorators)
        }
    }
}

impl<'a> ReflectionHost<'a> for TypeScriptReflectionHost<'a> {
    fn get_decorators_of_declaration(&self, declaration: &'a oxc::Declaration<'a>) -> Vec<Decorator<'a>> {
        let oxc_decorators = match declaration {
            oxc::Declaration::ClassDeclaration(class_decl) => &class_decl.decorators,
            // Functions and Variables do not support decorators in this context
            _ => return Vec::new(),
        };
        
        self.convert_decorators(oxc_decorators).unwrap_or_default()
    }
    
    fn get_members_of_class(&self, clazz: &'a ClassDeclaration<'a>) -> Vec<ClassMember<'a>> {
        let mut members = Vec::new();
        for element in &clazz.body.body {
            match element {
                oxc::ClassElement::MethodDefinition(method) => {
                    let name = match &method.key {
                        oxc::PropertyKey::StaticIdentifier(id) => id.name.to_string(),
                        oxc::PropertyKey::Identifier(id) => id.name.to_string(),
                        oxc::PropertyKey::PrivateIdentifier(id) => id.name.to_string(),
                        oxc::PropertyKey::StringLiteral(lit) => lit.value.to_string(),
                        _ => "unknown".to_string()
                    };
                    
                    let kind = match method.kind {
                        oxc::MethodDefinitionKind::Constructor => ClassMemberKind::Constructor,
                        oxc::MethodDefinitionKind::Method => ClassMemberKind::Method,
                        oxc::MethodDefinitionKind::Get => ClassMemberKind::Getter,
                        oxc::MethodDefinitionKind::Set => ClassMemberKind::Setter,
                    };

                    members.push(ClassMember {
                        node: Some(element),
                        kind,
                        access_level: ClassMemberAccessLevel::PublicWritable, // TODO: Check modifiers
                        type_node: None, // Method definition declaration doesn't list ret type directly here
                        name,
                        name_node: Some(&method.key),
                        value: None,
                        is_static: method.r#static,
                        decorators: self.convert_decorators(&method.decorators),
                    });
                },
                oxc::ClassElement::PropertyDefinition(prop) => {
                     let name = match &prop.key {
                        oxc::PropertyKey::StaticIdentifier(id) => id.name.to_string(),
                        oxc::PropertyKey::Identifier(id) => id.name.to_string(),
                        oxc::PropertyKey::PrivateIdentifier(id) => id.name.to_string(),
                        oxc::PropertyKey::StringLiteral(lit) => lit.value.to_string(),
                        _ => "unknown".to_string()
                    };
                    
                    members.push(ClassMember {
                        node: Some(element),
                        kind: ClassMemberKind::Property,
                        access_level: ClassMemberAccessLevel::PublicWritable, // TODO
                        type_node: prop.type_annotation.as_ref().map(|t| &t.type_annotation),
                        name,
                        name_node: Some(&prop.key),
                        value: prop.value.as_ref(),
                        is_static: prop.r#static,
                        decorators: self.convert_decorators(&prop.decorators),
                    });
                },
                _ => {}
            }
        }
        members
    }
    
    fn get_constructor_parameters(&self, clazz: &'a ClassDeclaration<'a>) -> Option<Vec<CtorParameter<'a>>> {
        for element in &clazz.body.body {
             if let oxc::ClassElement::MethodDefinition(method) = element {
                 if method.kind == oxc::MethodDefinitionKind::Constructor {
                     let mut params = Vec::new();
                     for param in &method.value.params.items {
                         let name = match &param.pattern.kind {
                             oxc::BindingPatternKind::BindingIdentifier(id) => Some(id.name.to_string()),
                             _ => None
                         };
                         
                         // type_value_reference needs to be resolved. For now Unavailable.
                         let type_value_reference = super::host::TypeValueReference::Unavailable(
                             super::host::UnavailableTypeValueReference {
                                 kind: super::host::TypeValueReferenceKind::Unavailable,
                                 reason: super::host::UnavailableValue::MissingType,
                             }
                         );

                         params.push(CtorParameter {
                             name,
                             name_node: &param.pattern,
                             type_value_reference,
                             type_node: param.pattern.type_annotation.as_ref().map(|t| &t.type_annotation),
                             decorators: self.convert_decorators(&param.decorators),
                         });
                     }
                     return Some(params);
                 }
             }
        }
        None
    }
    
    fn get_definition_of_function(&self, fn_node: &'a oxc::Function<'a>) -> Option<FunctionDefinition<'a>> {
        let mut params = Vec::new();
         for param in &fn_node.params.items {
             let name = match &param.pattern.kind {
                 oxc::BindingPatternKind::BindingIdentifier(id) => Some(id.name.to_string()),
                 oxc::BindingPatternKind::AssignmentPattern(assign) => {
                     match &assign.left.kind {
                        oxc::BindingPatternKind::BindingIdentifier(id) => Some(id.name.to_string()),
                        _ => None
                     }
                 },
                 _ => None
             };
             
             params.push(Parameter {
                 name,
                 node: param,
                 initializer: None, // Needs check logic
                 type_node: param.pattern.type_annotation.as_ref().map(|t| &t.type_annotation),
             });
         }
        
        Some(FunctionDefinition {
            node: fn_node,
            body: fn_node.body.as_ref().map(|b| &**b),
            parameters: params,
            type_parameters: fn_node.type_parameters.as_ref().map(|b| &**b),
            signature_count: 1,
        })
    }
    
    fn get_import_of_identifier(&self, _id: &'a oxc::IdentifierReference<'a>) -> Option<Import<'a>> {
        None
    }
    
    fn get_declaration_of_identifier(&self, _id: &'a oxc::IdentifierReference<'a>) -> Option<Declaration<'a>> {
        None
    }
    
    fn get_exports_of_module(&self, _module: &'a oxc::Program<'a>) -> Option<HashMap<String, Declaration<'a>>> {
        None
    }
    
    fn is_class(&self, node: &'a oxc::Declaration<'a>) -> bool {
         matches!(node, oxc::Declaration::ClassDeclaration(_))
    }
    
    fn has_base_class(&self, clazz: &'a ClassDeclaration<'a>) -> bool {
        clazz.super_class.is_some()
    }
    
    fn get_base_class_expression(&self, clazz: &'a ClassDeclaration<'a>) -> Option<&'a oxc::Expression<'a>> {
        clazz.super_class.as_ref()
    }
    
    fn get_generic_arity_of_class(&self, clazz: &'a ClassDeclaration<'a>) -> Option<usize> {
        clazz.type_parameters.as_ref().map(|p| p.params.len())
    }
    
    fn get_variable_value(&self, declaration: &'a oxc::VariableDeclarator<'a>) -> Option<&'a oxc::Expression<'a>> {
        declaration.init.as_ref()
    }
    
    fn is_statically_exported(&self, _decl: &'a oxc::Declaration<'a>) -> bool {
        false
    }
}
