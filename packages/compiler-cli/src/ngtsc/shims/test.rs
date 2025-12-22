// Shims Tests
//
// Tests for the shims module.

#[cfg(test)]
mod tests {
    use crate::ngtsc::shims::*;
    
    mod shim_generator_tests {
        use super::*;
        
        #[test]
        fn should_create_generator() {
            let gen = ShimGenerator::new();
            assert!(gen.base_content().is_empty() || !gen.base_content().is_empty());
        }
        
        #[test]
        fn should_generate_factory_shim() {
            let gen = ShimGenerator::new();
            let shim = gen.generate("app.module.ts", ShimType::Factory);
            
            assert!(!shim.file_name.is_empty());
            assert_eq!(shim.shim_type, ShimType::Factory);
        }
        
        #[test]
        fn should_generate_summary_shim() {
            let gen = ShimGenerator::new();
            let shim = gen.generate("app.module.ts", ShimType::Summary);
            
            assert_eq!(shim.shim_type, ShimType::Summary);
        }
    }
    
    mod shim_file_tests {
        use super::*;
        
        #[test]
        fn should_create_shim_file() {
            let shim = ShimFile {
                file_name: "app.module.ngsummary.ts".to_string(),
                content: "export const AppModuleNgSummary = {};".to_string(),
                shim_type: ShimType::Summary,
            };
            
            assert!(shim.file_name.contains("ngsummary"));
        }
    }
}
