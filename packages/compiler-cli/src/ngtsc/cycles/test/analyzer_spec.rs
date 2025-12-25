use crate::ngtsc::cycles::src::analyzer::CycleAnalyzer;
use crate::ngtsc::cycles::src::imports::ImportGraph;
use crate::ngtsc::cycles::test::util::{
    create_fs_from_graph, import_path_to_string, MockSourceFile,
}; // Import MockSourceFile

#[test]
fn test_caches_results() {
    let fs = create_fs_from_graph("a:b;b:c;c");
    let graph = ImportGraph::new(&fs);
    let analyzer = CycleAnalyzer::new(&graph);

    let a_sf = MockSourceFile {
        file_name: "/a.ts".to_string(),
        text: "".to_string(),
    };
    let b_sf = MockSourceFile {
        file_name: "/b.ts".to_string(),
        text: "".to_string(),
    };

    assert!(analyzer.would_create_cycle(&a_sf, &b_sf).is_none());
    assert!(analyzer.would_create_cycle(&a_sf, &b_sf).is_none());
}

#[test]
fn test_detects_simple_cycle() {
    let fs = create_fs_from_graph("a:b;b");
    let graph = ImportGraph::new(&fs);
    let analyzer = CycleAnalyzer::new(&graph);

    let a_sf = MockSourceFile {
        file_name: "/a.ts".to_string(),
        text: "".to_string(),
    };
    let b_sf = MockSourceFile {
        file_name: "/b.ts".to_string(),
        text: "".to_string(),
    };

    // a -> b exists in graph.
    // check if adding b -> a creates cycle.
    let cycle = analyzer.would_create_cycle(&b_sf, &a_sf);
    assert!(cycle.is_some());
    let cycle = cycle.unwrap();
    assert_eq!(import_path_to_string(&fs, cycle.get_path()), "b,a,b");
}

#[test]
fn test_detects_transitive_cycle() {
    let fs = create_fs_from_graph("a:b;b:c;c");
    let graph = ImportGraph::new(&fs);
    let analyzer = CycleAnalyzer::new(&graph);

    let a_sf = MockSourceFile {
        file_name: "/a.ts".to_string(),
        text: "".to_string(),
    };
    let c_sf = MockSourceFile {
        file_name: "/c.ts".to_string(),
        text: "".to_string(),
    };

    // a -> b -> c
    // check if c -> a creates cycle
    let cycle = analyzer.would_create_cycle(&c_sf, &a_sf);
    assert!(cycle.is_some());
    let cycle = cycle.unwrap();
    assert_eq!(import_path_to_string(&fs, cycle.get_path()), "c,a,b,c");
}

#[test]
fn test_handles_synthetic_imports() {
    let fs = create_fs_from_graph("a;b");
    let graph = ImportGraph::new(&fs);
    let analyzer = CycleAnalyzer::new(&graph);

    let a_sf = MockSourceFile {
        file_name: "/a.ts".to_string(),
        text: "".to_string(),
    };
    let b_sf = MockSourceFile {
        file_name: "/b.ts".to_string(),
        text: "".to_string(),
    };

    assert!(analyzer.would_create_cycle(&b_sf, &a_sf).is_none());

    analyzer.record_synthetic_import(&a_sf, &b_sf);

    let cycle = analyzer.would_create_cycle(&b_sf, &a_sf);
    assert!(cycle.is_some());
    assert_eq!(
        import_path_to_string(&fs, cycle.unwrap().get_path()),
        "b,a,b"
    );
}

#[test]
fn test_synthetic_edge_cycle() {
    let fs = create_fs_from_graph("a:b,c;b;c");
    let graph = ImportGraph::new(&fs);
    let analyzer = CycleAnalyzer::new(&graph);

    let b_sf = MockSourceFile {
        file_name: "/b.ts".to_string(),
        text: "".to_string(),
    };
    let c_sf = MockSourceFile {
        file_name: "/c.ts".to_string(),
        text: "".to_string(),
    };

    assert!(analyzer.would_create_cycle(&b_sf, &c_sf).is_none());

    analyzer.record_synthetic_import(&c_sf, &b_sf);
    let cycle = analyzer.would_create_cycle(&b_sf, &c_sf);
    assert!(cycle.is_some());
    assert_eq!(
        import_path_to_string(&fs, cycle.unwrap().get_path()),
        "b,c,b"
    );
}

#[test]
fn test_more_complex_cycle() {
    // a:*b,*c;b:*e,*f;c:*g,*h;e:f;f:c;g;h:g
    let fs = create_fs_from_graph("a:*b,*c;b:*e,*f;c:*g,*h;e:f;f:c;g;h:g");
    let graph = ImportGraph::new(&fs);
    let analyzer = CycleAnalyzer::new(&graph);

    let b_sf = MockSourceFile {
        file_name: "/b.ts".to_string(),
        text: "".to_string(),
    };
    let g_sf = MockSourceFile {
        file_name: "/g.ts".to_string(),
        text: "".to_string(),
    };

    // Check b -> g (no cycle)
    assert!(analyzer.would_create_cycle(&b_sf, &g_sf).is_none());

    // Check g -> b (cycle: g -> b -> f -> c -> g)
    let cycle = analyzer.would_create_cycle(&g_sf, &b_sf);
    assert!(cycle.is_some());

    assert_eq!(
        import_path_to_string(&fs, cycle.unwrap().get_path()),
        "g,b,f,c,g"
    );
}
