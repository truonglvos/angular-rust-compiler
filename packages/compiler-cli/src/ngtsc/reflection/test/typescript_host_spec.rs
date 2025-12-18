use crate::ngtsc::reflection::{
    ClassMemberKind, TypeScriptReflectionHost, ReflectionHost,
};
use oxc_allocator::Allocator;
use oxc_ast::ast;
use oxc_parser::Parser;
use oxc_span::SourceType;

struct TestProgram<'a> {
    allocator: &'a Allocator,
    program: ast::Program<'a>,
}

impl<'a> TestProgram<'a> {
    fn new(allocator: &'a Allocator, source: &'a str) -> Self {
        let source_type = SourceType::default().with_typescript(true).with_module(true);
        let ret = Parser::new(allocator, source, source_type).parse();
        
        if !ret.errors.is_empty() {
            panic!("Parse errors: {:?}", ret.errors);
        }

        Self {
            allocator,
            program: ret.program,
        }
    }
    
    fn find_class(&self, name: &str) -> Option<&ast::Class<'a>> {
        for stmt in &self.program.body {
            match stmt {
                ast::Statement::ClassDeclaration(class) => {
                    if let Some(id) = &class.id {
                        if id.name == name {
                            return Some(class);
                        }
                    }
                }
                ast::Statement::ExportNamedDeclaration(decl) => {
                    if let Some(ast::Declaration::ClassDeclaration(class)) = &decl.declaration {
                        if let Some(id) = &class.id {
                            if id.name == name {
                                return Some(class);
                            }
                        }
                    }
                }
                ast::Statement::ExportDefaultDeclaration(decl) => {
                     if let ast::ExportDefaultDeclarationKind::ClassDeclaration(class) = &decl.declaration {
                        if let Some(id) = &class.id {
                            if id.name == name {
                                return Some(class);
                            }
                        }
                     }
                }
                _ => {}
            }
        }
        None
    }

    fn find_function(&self, name: &str) -> Option<&ast::Function<'a>> {
         for stmt in &self.program.body {
            match stmt {
                ast::Statement::FunctionDeclaration(func) => {
                     if let Some(id) = &func.id {
                        if id.name == name {
                            return Some(func);
                        }
                    }
                }
                ast::Statement::ExportNamedDeclaration(decl) => {
                    if let Some(ast::Declaration::FunctionDeclaration(func)) = &decl.declaration {
                        if let Some(id) = &func.id {
                            if id.name == name {
                                return Some(func);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn find_variable(&self, name: &str) -> Option<&ast::VariableDeclarator<'a>> {
        for stmt in &self.program.body {
             match stmt {
                ast::Statement::VariableDeclaration(var_decl) => {
                     for decl in &var_decl.declarations {
                         if let ast::BindingPatternKind::BindingIdentifier(id) = &decl.id.kind {
                             if id.name == name {
                                 return Some(decl);
                             }
                         }
                    }
                }
                 ast::Statement::ExportNamedDeclaration(decl) => {
                    if let Some(ast::Declaration::VariableDeclaration(var_decl)) = &decl.declaration {
                         for decl in &var_decl.declarations {
                             if let ast::BindingPatternKind::BindingIdentifier(id) = &decl.id.kind {
                                 if id.name == name {
                                     return Some(decl);
                                 }
                             }
                        }
                    }
                }
                _ => {}
             }
        }
        None
    }
    
    fn find_declaration(&self, name: &str) -> Option<&ast::Declaration<'a>> {
        for stmt in &self.program.body {
            if let ast::Statement::ExportNamedDeclaration(decl) = stmt {
                if let Some(declaration) = &decl.declaration {
                    match declaration {
                        ast::Declaration::ClassDeclaration(class) => {
                            if let Some(id) = &class.id {
                                if id.name == name {
                                    return Some(declaration);
                                }
                            }
                        },
                        ast::Declaration::FunctionDeclaration(func) => {
                             if let Some(id) = &func.id {
                                if id.name == name {
                                    return Some(declaration);
                                }
                            }
                        },
                        ast::Declaration::VariableDeclaration(var_decl) => {
                             for d in &var_decl.declarations {
                                 if let ast::BindingPatternKind::BindingIdentifier(id) = &d.id.kind {
                                     if id.name == name {
                                         return Some(declaration);
                                     }
                                 }
                             }
                        }
                        _ => {}
                    }
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_constructor_parameters_basic() {
        let source = r#"
            class Bar {}
            class Foo {
              constructor(bar: Bar) {}
            }
        "#;
        let allocator = Allocator::default();
        let program = TestProgram::new(&allocator, source);
        let foo_class = program.find_class("Foo").expect("Class Foo not found");
        
        let host = TypeScriptReflectionHost::new();
        let args = host.get_constructor_parameters(foo_class).expect("Constructor params not found");
        
        assert_eq!(args.len(), 1);
        assert_eq!(args[0].name.as_deref(), Some("bar"));
    }

    #[test]
    fn test_get_constructor_parameters_decorated() {
        let source = r#"
            import {dec} from './dec';
            class Bar {}
            class Foo {
              constructor(@dec bar: Bar) {}
            }
        "#;
        let allocator = Allocator::default();
        let program = TestProgram::new(&allocator, source);
        let foo_class = program.find_class("Foo").expect("Class Foo not found");
        
        let host = TypeScriptReflectionHost::new();
        let args = host.get_constructor_parameters(foo_class).expect("Constructor params not found");
        
        assert_eq!(args.len(), 1);
        assert_eq!(args[0].name.as_deref(), Some("bar"));
        
        let decorators = args[0].decorators.as_ref().expect("Decorators should exist");
        assert_eq!(decorators.len(), 1);
        assert_eq!(decorators[0].name, "dec");
    }

    #[test]
    fn test_get_members_of_class_properties() {
        let source = r#"
            class Foo {
              'string-literal-property-member' = 'my value';
              regularProp = 123;
            }
        "#;
        let allocator = Allocator::default();
        let program = TestProgram::new(&allocator, source);
        let foo_class = program.find_class("Foo").expect("Class Foo not found");
        
        let host = TypeScriptReflectionHost::new();
        let members = host.get_members_of_class(foo_class);
        
        let literal_prop = members.iter().find(|m| m.name == "string-literal-property-member").expect("Literal prop not found");
        assert_eq!(literal_prop.kind, ClassMemberKind::Property);
        
        let regular_prop = members.iter().find(|m| m.name == "regularProp").expect("Regular prop not found");
        assert_eq!(regular_prop.kind, ClassMemberKind::Property);
    }

    #[test]
    fn test_get_members_of_class_methods() {
        let source = r#"
            class Foo {
              myMethod(): void {}
            }
        "#;
        let allocator = Allocator::default();
        let program = TestProgram::new(&allocator, source);
        let foo_class = program.find_class("Foo").expect("Class Foo not found");
        
        let host = TypeScriptReflectionHost::new();
        let members = host.get_members_of_class(foo_class);
        
        let method = members.iter().find(|m| m.name == "myMethod").expect("Method not found");
        assert_eq!(method.kind, ClassMemberKind::Method);
    }
    
    #[test]
    fn test_get_members_static() {
        let source = r#"
            class Foo {
              static staticMember = '';
            }
        "#;
        let allocator = Allocator::default();
        let program = TestProgram::new(&allocator, source);
        let foo_class = program.find_class("Foo").expect("Class Foo not found");
        
        let host = TypeScriptReflectionHost::new();
        let members = host.get_members_of_class(foo_class);
        
        let static_member = members.iter().find(|m| m.name == "staticMember").expect("Static member not found");
        assert!(static_member.is_static);
    }
    
    #[test]
    fn test_member_decorators() {
        let source = r#"
            class Foo {
              @Input()
              prop: string;
            }
        "#;
        let allocator = Allocator::default();
        let program = TestProgram::new(&allocator, source);
        let foo_class = program.find_class("Foo").expect("Class Foo not found");
        
        let host = TypeScriptReflectionHost::new();
        let members = host.get_members_of_class(foo_class);
        
        let prop = members.iter().find(|m| m.name == "prop").expect("Prop not found");
        let decorators = prop.decorators.as_ref().expect("Decorators not found");
        assert_eq!(decorators[0].name, "Input");
    }

    #[test]
    fn test_is_class() {
        let source = r#"
            export class A {}
            export function foo() {}
        "#;
        let allocator = Allocator::default();
        let program = TestProgram::new(&allocator, source);
        let host = TypeScriptReflectionHost::new();

        let class_decl = program.find_declaration("A").expect("Class A declaration not found");
        assert!(host.is_class(class_decl));

        let func_decl = program.find_declaration("foo").expect("Function foo declaration not found");
        assert!(!host.is_class(func_decl));
    }

    #[test]
    fn test_has_base_class() {
        let source = r#"
            class A {}
            class B extends A {}
        "#;
        let allocator = Allocator::default();
        let program = TestProgram::new(&allocator, source);
        let host = TypeScriptReflectionHost::new();

        let class_a = program.find_class("A").expect("Class A not found");
        assert!(!host.has_base_class(class_a));

        let class_b = program.find_class("B").expect("Class B not found");
        assert!(host.has_base_class(class_b));
        
        let base_expr = host.get_base_class_expression(class_b).expect("Base class expression should exist");
        // Verify base expression is identifier 'A'
        if let ast::Expression::Identifier(id) = base_expr {
            assert_eq!(id.name, "A");
        } else {
            panic!("Base class expression is not an identifier");
        }
    }

    #[test]
    fn test_generic_arity() {
        let source = r#"
            class A {}
            class B<T> {}
            class C<T, U> {}
        "#;
        let allocator = Allocator::default();
        let program = TestProgram::new(&allocator, source);
        let host = TypeScriptReflectionHost::new();

        let class_a = program.find_class("A").expect("Class A not found");
        assert_eq!(host.get_generic_arity_of_class(class_a), None);

        let class_b = program.find_class("B").expect("Class B not found");
        assert_eq!(host.get_generic_arity_of_class(class_b), Some(1));

        let class_c = program.find_class("C").expect("Class C not found");
        assert_eq!(host.get_generic_arity_of_class(class_c), Some(2));
    }

    #[test]
    fn test_reflection_functions() {
        let source = r#"
            function foo(a: string, b: number = 1) {}
        "#;
        let allocator = Allocator::default();
        let program = TestProgram::new(&allocator, source);
        let host = TypeScriptReflectionHost::new();

        let func = program.find_function("foo").expect("Function foo not found");
        let def = host.get_definition_of_function(func).expect("Function definition not found");
        
        assert_eq!(def.parameters.len(), 2);
        assert_eq!(def.parameters[0].name.as_deref(), Some("a"));
        assert_eq!(def.parameters[1].name.as_deref(), Some("b"));
    }

    #[test]
    fn test_reflection_variables() {
        let source = r#"
            export const a = 1;
        "#;
        let allocator = Allocator::default();
        let program = TestProgram::new(&allocator, source);
        let host = TypeScriptReflectionHost::new();

        let decl = program.find_variable("a").expect("Variable a not found");
        let value = host.get_variable_value(decl).expect("Variable value not found");
        
        // Check value is literal 1
        if let ast::Expression::NumericLiteral(lit) = value {
            assert_eq!(lit.value, 1.0);
        } else {
             panic!("Variable value is not a numeric literal");
        }
    }

    #[test]
    fn test_decorators_of_declaration() {
        let source = r#"
            @Dec()
            export class A {}
        "#;
        let allocator = Allocator::default();
        let program = TestProgram::new(&allocator, source);
        let host = TypeScriptReflectionHost::new();

        let decl = program.find_declaration("A").expect("Class A declaration not found");
        let decorators = host.get_decorators_of_declaration(decl);
        
        assert_eq!(decorators.len(), 1);
        assert_eq!(decorators[0].name, "Dec");
    }
}
