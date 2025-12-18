pub mod src;

// Re-export from src to maintain convenience access if needed, or mirroring TS index.ts
pub use src::host::{ReflectionHost, Decorator, ClassDeclaration, Declaration, ClassMember, CtorParameter, FunctionDefinition, Import, ClassMemberKind};

pub use src::typescript::TypeScriptReflectionHost;

#[cfg(test)]
pub mod test;
// Exporting OxcReflectionHost alias as TypeScriptReflectionHost is what we likely want if we renamed it?
// The file is renamed to typescript.rs, so the struct inside should arguably be TypeScriptReflectionHost 
// (or OxcReflectionHost if we keep the name). Checking file content next.
