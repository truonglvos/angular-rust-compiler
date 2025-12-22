// Indexer API
//
// Types for indexed Angular declarations.

/// Indexed component.
#[derive(Debug, Clone)]
pub struct IndexedComponent {
    pub name: String,
    pub selector: Option<String>,
    pub template_file: Option<String>,
    pub style_files: Vec<String>,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

/// Indexed directive.
#[derive(Debug, Clone)]
pub struct IndexedDirective {
    pub name: String,
    pub selector: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

/// Indexed pipe.
#[derive(Debug, Clone)]
pub struct IndexedPipe {
    pub name: String,
    pub pipe_name: String,
}

/// Indexed NgModule.
#[derive(Debug, Clone)]
pub struct IndexedNgModule {
    pub name: String,
    pub declarations: Vec<String>,
    pub imports: Vec<String>,
    pub exports: Vec<String>,
    pub providers: Vec<String>,
}
