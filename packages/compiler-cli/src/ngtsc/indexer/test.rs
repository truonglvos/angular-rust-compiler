// Indexer Tests
//
// Tests for the indexer module.

#[cfg(test)]
mod tests {
    use crate::ngtsc::indexer::*;

    mod indexing_context_tests {
        use super::*;

        #[test]
        fn should_create_empty_context() {
            let ctx = IndexingContext::new();
            assert!(ctx.get_components("test.ts").is_none());
        }

        #[test]
        fn should_add_component() {
            let mut ctx = IndexingContext::new();

            ctx.add_component(
                "app.component.ts",
                IndexedComponent {
                    name: "AppComponent".to_string(),
                    selector: Some("app-root".to_string()),
                    template_file: Some("app.component.html".to_string()),
                    style_files: vec!["app.component.css".to_string()],
                    inputs: vec!["title".to_string()],
                    outputs: vec!["click".to_string()],
                },
            );

            let components = ctx.get_components("app.component.ts").unwrap();
            assert_eq!(components.len(), 1);
            assert_eq!(components[0].name, "AppComponent");
        }

        #[test]
        fn should_iterate_all_components() {
            let mut ctx = IndexingContext::new();

            ctx.add_component(
                "a.ts",
                IndexedComponent {
                    name: "A".to_string(),
                    selector: Some("a".to_string()),
                    template_file: None,
                    style_files: vec![],
                    inputs: vec![],
                    outputs: vec![],
                },
            );

            ctx.add_component(
                "b.ts",
                IndexedComponent {
                    name: "B".to_string(),
                    selector: Some("b".to_string()),
                    template_file: None,
                    style_files: vec![],
                    inputs: vec![],
                    outputs: vec![],
                },
            );

            let all: Vec<_> = ctx.all_components().collect();
            assert_eq!(all.len(), 2);
        }
    }

    mod indexer_tests {
        use super::*;

        #[test]
        fn should_create_indexer() {
            let indexer = Indexer::new();
            assert!(indexer.context().all_components().next().is_none());
        }

        #[test]
        fn should_get_mutable_context() {
            let mut indexer = Indexer::new();

            indexer.context_mut().add_component(
                "test.ts",
                IndexedComponent {
                    name: "TestComponent".to_string(),
                    selector: None,
                    template_file: None,
                    style_files: vec![],
                    inputs: vec![],
                    outputs: vec![],
                },
            );

            assert!(indexer.context().get_components("test.ts").is_some());
        }
    }
}
