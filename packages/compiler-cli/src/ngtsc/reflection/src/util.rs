use oxc_ast::ast;

use super::host::ClassMemberAccessLevel;

pub fn is_named_class_declaration<'a>(node: &'a ast::Declaration<'a>) -> bool {
    if let ast::Declaration::ClassDeclaration(class_decl) = node {
        return class_decl.id.is_some();
    }
    false
}

pub fn is_named_function_declaration<'a>(node: &'a ast::Declaration<'a>) -> bool {
    if let ast::Declaration::FunctionDeclaration(func_decl) = node {
        return func_decl.id.is_some();
    }
    false
}

pub fn is_named_variable_declaration<'a>(node: &'a ast::Declaration<'a>) -> bool {
    if let ast::Declaration::VariableDeclaration(_) = node {
        // VariableDeclaration has declarations: Vec<VariableDeclarator>
        // TS logic is slightly different as it treats each declarator as a declaration often.
        // For Oxc, this check might check if *any* declarator is named (which they must be).
        return true; 
    }
    false
}

// Helper to check for identifier
pub fn is_identifier<'a>(node: Option<&'a ast::BindingIdentifier<'a>>) -> bool {
    node.is_some()
}

pub fn class_member_access_level_to_string(level: ClassMemberAccessLevel) -> &'static str {
    match level {
        ClassMemberAccessLevel::EcmaScriptPrivate => "ES private",
        ClassMemberAccessLevel::Private => "private",
        ClassMemberAccessLevel::Protected => "protected",
        ClassMemberAccessLevel::PublicReadonly => "public readonly",
        ClassMemberAccessLevel::PublicWritable => "public",
    }
}
