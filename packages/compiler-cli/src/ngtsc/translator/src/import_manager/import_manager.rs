use crate::ngtsc::translator::src::api::ast_factory::AstFactory;
use crate::ngtsc::translator::src::api::import_generator::{ImportGenerator, ImportRequest};
use crate::ngtsc::translator::src::import_manager::check_unique_identifier_name::{
    IdentifierScope, UniqueIdentifierGenerator,
};
use crate::ngtsc::translator::src::import_manager::reuse_generated_imports::{
    attempt_to_reuse_generated_imports, capture_generated_import, ReuseGeneratedImportsTracker,
};
use crate::ngtsc::translator::src::import_manager::reuse_source_file_imports::{
    attempt_to_reuse_existing_source_file_imports, ReuseExistingSourceFileImportsTracker,
    SourceFileImports,
};
use std::collections::{HashMap, HashSet};

// We define ModuleName as generic string for now.
pub type ModuleName = String;

pub struct ImportManagerConfig {
    pub namespace_import_prefix: String,
    pub disable_original_source_file_reuse: bool,
    pub force_generate_namespaces_for_new_imports: bool,
    // generateUniqueIdentifier is handled by UniqueIdentifierGenerator helper
}

pub struct ImportManager<'a, A: AstFactory, TFile> {
    config: ImportManagerConfig,
    ast_factory: &'a A,

    // Per file tracking
    new_imports: HashMap<TFile, NewImportsForFile>, // TFile must be Hash + Eq
    next_unique_index: usize,

    reuse_generated_imports_tracker: ReuseGeneratedImportsTracker<A::Expression>,
    reuse_source_file_imports_tracker: ReuseExistingSourceFileImportsTracker,
    unique_id_generator: UniqueIdentifierGenerator,
}

struct NewImportsForFile {
    namespace_imports: HashMap<ModuleName, String>, // Module -> Namespace Name
    named_imports: HashMap<ModuleName, Vec<ImportSpecifierInternal>>,
    side_effect_imports: HashSet<ModuleName>,
}

pub struct ImportSpecifierInternal {
    pub name: String,
    pub alias: Option<String>,
}

pub struct FinalizeResult<TFile, TDeclaration> {
    pub affected_files: HashSet<TFile>,
    pub new_imports: HashMap<TFile, Vec<NewImport>>, // TDeclaration usually Statement
    pub updated_imports: HashMap<TDeclaration, Vec<ImportSpecifierInternal>>, // ImportDeclaration -> specifiers
    pub reused_original_alias_declarations: HashSet<TDeclaration>,
}

pub struct NewImport {
    pub module_specifier: String,
    pub namespace_import: Option<String>,
    pub named_imports: Vec<ImportSpecifierInternal>,
    pub side_effect: bool,
}

impl<'a, A: AstFactory, TFile> ImportManager<'a, A, TFile>
where
    TFile: std::hash::Hash + Eq + Clone + IdentifierScope + SourceFileImports,
    A::Expression: Clone,
{
    pub fn new(ast_factory: &'a A, config: ImportManagerConfig) -> Self {
        Self {
            config,
            ast_factory,
            new_imports: HashMap::new(),
            next_unique_index: 0,
            reuse_generated_imports_tracker: ReuseGeneratedImportsTracker::new(),
            reuse_source_file_imports_tracker: ReuseExistingSourceFileImportsTracker::new(),
            unique_id_generator: UniqueIdentifierGenerator::new(),
        }
    }

    fn get_new_imports_tracker_for_file(&mut self, file: &TFile) -> &mut NewImportsForFile {
        self.new_imports
            .entry(file.clone())
            .or_insert(NewImportsForFile {
                namespace_imports: HashMap::new(),
                named_imports: HashMap::new(),
                side_effect_imports: HashSet::new(),
            })
    }

    pub fn finalize(&self) -> FinalizeResult<TFile, A::Statement> {
        // Collect new imports to be generated
        let mut new_imports = HashMap::new();
        let mut affected_files = HashSet::new();

        for (file, tracker) in &self.new_imports {
            affected_files.insert(file.clone());
            let file_new_imports = new_imports.entry(file.clone()).or_insert(Vec::new());

            // Process namespace imports
            for (module, alias) in &tracker.namespace_imports {
                file_new_imports.push(NewImport {
                    module_specifier: module.clone(),
                    namespace_import: Some(alias.clone()),
                    named_imports: Vec::new(),
                    side_effect: false,
                });
            }

            // Process named imports
            for (module, specifiers) in &tracker.named_imports {
                // Merge with existing named import if any? No, this is for *new* imports.
                if !specifiers.is_empty() {
                    // We need to clone specifiers.
                    let specifiers_cloned = specifiers
                        .iter()
                        .map(|s| ImportSpecifierInternal {
                            name: s.name.clone(),
                            alias: s.alias.clone(),
                        })
                        .collect();

                    file_new_imports.push(NewImport {
                        module_specifier: module.clone(),
                        namespace_import: None,
                        named_imports: specifiers_cloned,
                        side_effect: false,
                    });
                }
            }

            // Side effects
            for module in &tracker.side_effect_imports {
                file_new_imports.push(NewImport {
                    module_specifier: module.clone(),
                    namespace_import: None,
                    named_imports: Vec::new(),
                    side_effect: true,
                });
            }
        }

        // Updated imports would come from reuse tracker
        // But in reuse_source_file_imports implementation I used `ExistingImport` which is just data.
        // If we want to map back to AST nodes (A::Statement), we need to store them or be able to look valid ones up.
        // For now, I'll return empty maps for updated/reused until I link specific AST nodes.

        FinalizeResult {
            affected_files,
            new_imports,
            updated_imports: HashMap::new(), // TODO: Populate from reuse tracker
            reused_original_alias_declarations: HashSet::new(), // TODO: Populate
        }
    }
}

impl<'a, A: AstFactory, TFile> ImportGenerator<TFile, A::Expression> for ImportManager<'a, A, TFile>
where
    TFile: std::hash::Hash + Eq + Clone + IdentifierScope + SourceFileImports,
    A::Expression: Clone,
{
    fn add_import(&mut self, request: ImportRequest<TFile>) -> A::Expression {
        // Reuse generated
        if let Some(reused) =
            attempt_to_reuse_generated_imports(&self.reuse_generated_imports_tracker, &request)
        {
            // Need to handle if reused is a namespace import but we wanted a named import -> return PropertyAccess
            if request.export_symbol_name.is_some() {
                // If reused is Identifier(ns), we need ns.Symbol
                // reuse_generated_imports returns TExpression.
                // If TExpression is opaque, we rely on reuse logic returning the final expression?
                // But reuse logic returned just the cached one.
                // We should improve reuse_generated_imports to handle this or handle it here.
                // For now assuming direct reuse only.
                return reused;
            }
            return reused;
        }

        // Reuse source file
        if !self.config.disable_original_source_file_reuse {
            if let Some(reused) = attempt_to_reuse_existing_source_file_imports(
                &mut self.reuse_source_file_imports_tracker,
                &request.requested_file,
                &request,
                self.ast_factory,
            ) {
                return reused;
            }
        }

        let file = request.requested_file.clone();

        // Namespace Import
        if request.export_symbol_name.is_none()
            || self.config.force_generate_namespaces_for_new_imports
        {
            // Logic to generate namespace import
            let mut ns_name = format!(
                "{}{}",
                self.config.namespace_import_prefix, self.next_unique_index
            );
            self.next_unique_index += 1;

            // Check unique
            if let Some(unique) = self
                .unique_id_generator
                .generate_unique_identifier(&file, &ns_name)
            {
                ns_name = unique;
            }

            // Store in tracker
            let tracker = self
                .new_imports
                .entry(file.clone())
                .or_insert(NewImportsForFile {
                    namespace_imports: HashMap::new(),
                    named_imports: HashMap::new(),
                    side_effect_imports: HashSet::new(),
                });
            tracker
                .namespace_imports
                .insert(request.export_module_specifier.clone(), ns_name.clone());

            let ns_expr = self.ast_factory.create_identifier(&ns_name);
            capture_generated_import(
                &request,
                &mut self.reuse_generated_imports_tracker,
                ns_expr.clone(),
            );

            if let Some(symbol) = request.export_symbol_name {
                return self.ast_factory.create_property_access(ns_expr, &symbol);
            }
            return ns_expr;
        }

        // Named Import
        let symbol_name = request.export_symbol_name.as_ref().unwrap();

        // Generate unique alias if needed
        let unique_name = if let Some(alias) = &request.unsafe_alias_override {
            alias.clone()
        } else {
            self.unique_id_generator
                .generate_unique_identifier(&file, symbol_name)
                .unwrap_or(symbol_name.clone())
        };

        let needs_alias = &unique_name != symbol_name || request.unsafe_alias_override.is_some();
        let specifier_alias = if needs_alias {
            Some(unique_name.clone())
        } else {
            None
        };

        let tracker = self
            .new_imports
            .entry(file.clone())
            .or_insert(NewImportsForFile {
                namespace_imports: HashMap::new(),
                named_imports: HashMap::new(),
                side_effect_imports: HashSet::new(),
            });

        let exports = tracker
            .named_imports
            .entry(request.export_module_specifier.clone())
            .or_insert(Vec::new());

        exports.push(ImportSpecifierInternal {
            name: symbol_name.clone(),
            alias: specifier_alias,
        });

        let expr = self.ast_factory.create_identifier(&unique_name);
        capture_generated_import(
            &request,
            &mut self.reuse_generated_imports_tracker,
            expr.clone(),
        );
        expr
    }
}

// =========================================================================================
// EmitterImportManager
// A simplified ImportManager for use specifically with AbstractJsEmitter where no AST context exists.
// =========================================================================================

pub struct EmitterImportManager {
    /// Map of module name -> alias (e.g. "@angular/core" -> "i0")
    imports: HashMap<String, String>,
    /// Counter for generating unique aliases
    next_id: usize,
}

impl EmitterImportManager {
    pub fn new() -> Self {
        Self {
            imports: HashMap::new(),
            next_id: 0,
        }
    }

    /// Get the alias for a module, generating one if it doesn't exist.
    pub fn get_or_generate_alias(&mut self, module_name: &str) -> String {
        if let Some(alias) = self.imports.get(module_name) {
            return alias.clone();
        }

        let alias = format!("i{}", self.next_id);
        self.next_id += 1;
        self.imports.insert(module_name.to_string(), alias.clone());
        alias
    }

    /// Get the current map of imports to aliases
    pub fn get_imports_map(&self) -> HashMap<String, String> {
        self.imports.clone()
    }

    /// Generate the import statements to be prepended to the file
    pub fn generate_import_statements(&self) -> String {
        let mut statements = String::new();
        // Sort imports to ensure deterministic output
        let mut sorted_imports: Vec<_> = self.imports.iter().collect();
        sorted_imports.sort_by_key(|(module, _)| *module);

        for (module, alias) in sorted_imports {
            statements.push_str(&format!("import * as {} from '{}';\n", alias, module));
        }
        statements
    }
}
