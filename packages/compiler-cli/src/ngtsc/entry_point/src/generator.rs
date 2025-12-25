// Generator
//
// Generates flat module entry points.

/// Generator for flat module entry points.
pub struct FlatModuleEntryPointGenerator {
    /// Output file name.
    output_name: String,
    /// Module name.
    module_name: String,
}

impl FlatModuleEntryPointGenerator {
    pub fn new(output_name: impl Into<String>, module_name: impl Into<String>) -> Self {
        Self {
            output_name: output_name.into(),
            module_name: module_name.into(),
        }
    }

    /// Generate flat module file content.
    pub fn generate(&self, exports: &[FlatModuleExport]) -> String {
        let mut output = String::new();
        output.push_str("/**\n * Generated flat module entry point\n */\n");

        for export in exports {
            output.push_str(&format!(
                "export {{ {} }} from '{}';\\n",
                export.symbols.join(", "),
                export.from
            ));
        }

        output
    }

    /// Get output file name.
    pub fn output_name(&self) -> &str {
        &self.output_name
    }

    /// Get module name.
    pub fn module_name(&self) -> &str {
        &self.module_name
    }
}

/// An export for flat module.
#[derive(Debug, Clone)]
pub struct FlatModuleExport {
    /// Symbols to export.
    pub symbols: Vec<String>,
    /// Source module.
    pub from: String,
}
