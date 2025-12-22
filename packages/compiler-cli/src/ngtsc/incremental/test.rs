// Incremental Tests
//
// Tests for the incremental compilation module.

#[cfg(test)]
mod tests {
    use super::src::*;
    
    mod incremental_driver_tests {
        use super::*;
        
        #[test]
        fn should_create_fresh_state() {
            let state = IncrementalBuildState::new();
            assert!(state.is_empty());
        }
        
        #[test]
        fn should_track_analyzed_file() {
            let mut state = IncrementalBuildState::new();
            state.record_analyzed_file("test.ts".to_string());
            
            assert!(state.was_file_analyzed("test.ts"));
            assert!(!state.was_file_analyzed("other.ts"));
        }
        
        #[test]
        fn should_track_file_dependencies() {
            let mut tracker = DependencyTracker::new();
            tracker.add_dependency("component.ts", "service.ts");
            tracker.add_dependency("component.ts", "utils.ts");
            
            let deps = tracker.get_dependencies("component.ts");
            assert_eq!(deps.len(), 2);
            assert!(deps.contains(&"service.ts".to_string()));
            assert!(deps.contains(&"utils.ts".to_string()));
        }
        
        #[test]
        fn should_detect_affected_files() {
            let mut tracker = DependencyTracker::new();
            tracker.add_dependency("a.ts", "b.ts");
            tracker.add_dependency("b.ts", "c.ts");
            
            let affected = tracker.get_dependents("c.ts");
            assert!(affected.contains(&"b.ts".to_string()));
        }
    }
    
    mod incremental_strategy_tests {
        use super::*;
        
        #[test]
        fn should_use_noop_strategy_for_fresh_build() {
            let strategy = NoopIncrementalStrategy::new();
            assert!(strategy.get_analysis("anything").is_none());
            assert!(!strategy.requires_fresh_emit("anything"));
        }
        
        #[test]
        fn should_tracked_strategy_return_prior_state() {
            let state = IncrementalBuildState::new();
            let strategy = TrackedIncrementalStrategy::new(Some(state));
            
            // Fresh tracked strategy should have prior state
            assert!(strategy.get_prior_state().is_some());
        }
    }
    
    mod semantic_graph_tests {
        use super::super::semantic_graph::src::*;
        
        #[test]
        fn should_create_empty_graph() {
            let graph = SemanticDepGraph::new();
            assert!(graph.get_symbol("NonExistent").is_none());
        }
        
        #[test]
        fn should_register_symbol() {
            let mut graph = SemanticDepGraph::new();
            let symbol = SemanticSymbol {
                name: "TestSymbol".to_string(),
                identifier: "test_symbol".to_string(),
                path: "test.ts".to_string(),
            };
            graph.register_symbol(symbol);
            
            assert!(graph.get_symbol("TestSymbol").is_some());
        }
        
        #[test]
        fn should_track_symbol_dependencies() {
            let mut graph = SemanticDepGraph::new();
            
            let symbol1 = SemanticSymbol {
                name: "Service".to_string(),
                identifier: "service".to_string(),
                path: "service.ts".to_string(),
            };
            let symbol2 = SemanticSymbol {
                name: "Component".to_string(),
                identifier: "component".to_string(),
                path: "component.ts".to_string(),
            };
            
            graph.register_symbol(symbol1);
            graph.register_symbol(symbol2);
            graph.add_dependency("Component", "Service");
            
            let deps = graph.get_dependencies("Component");
            assert!(deps.contains(&"Service".to_string()));
        }
    }
}
