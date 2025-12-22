use crate::ngtsc::cycles::src::imports::ImportGraph;
use crate::ngtsc::file_system::{AbsoluteFsPath, ReadonlyFileSystem};
use crate::ngtsc::cycles::test::util::{create_fs_from_graph, import_path_to_string, MockSourceFile};

#[test]
fn test_scan_imports_simple() {
    let fs = create_fs_from_graph("a:b,c;b;c");
    let graph = ImportGraph::new(&fs);
    
    // read_file returns String in MockFileSystem/ReadonlyFileSystem
    let text = fs.read_file(&AbsoluteFsPath::from("/a.ts")).unwrap();
    let a_sf = MockSourceFile { 
        file_name: "/a.ts".to_string(), 
        text,
    };

    let imports = graph.imports_of(&a_sf);
    assert_eq!(imports.len(), 2);
    assert!(imports.contains(&AbsoluteFsPath::from("/b.ts")));
    assert!(imports.contains(&AbsoluteFsPath::from("/c.ts")));
}

#[test]
fn test_find_path() {
    // a -> b -> c
    // e -> f
    let fs = create_fs_from_graph("a:b;b:c;c;e:f;f");
    let graph = ImportGraph::new(&fs);
    
    let a_sf = MockSourceFile { file_name: "/a.ts".to_string(), text: "".to_string() };
    let c_sf = MockSourceFile { file_name: "/c.ts".to_string(), text: "".to_string() };
    let e_sf = MockSourceFile { file_name: "/e.ts".to_string(), text: "".to_string() };
    let b_sf = MockSourceFile { file_name: "/b.ts".to_string(), text: "".to_string() };

    // a -> c
    let path = graph.find_path(&a_sf, &c_sf);
    assert!(path.is_some());
    let path = path.unwrap();
    assert_eq!(import_path_to_string(&fs, &path), "a,b,c");
    
    // e -> c (no path)
    assert!(graph.find_path(&e_sf, &c_sf).is_none());
    
    // b -> c (path exists)
    let path = graph.find_path(&b_sf, &c_sf);
    assert!(path.is_some());
    assert_eq!(import_path_to_string(&fs, &path.unwrap()), "b,c");
}

#[test]
fn test_find_path_circular() {
    // a -> b -> c -> d
    // ^----/    |
    // ^---------/
    let fs = create_fs_from_graph("a:b;b:a,c;c:a,d;d");
    let graph = ImportGraph::new(&fs);
    
    let a_sf = MockSourceFile { file_name: "/a.ts".to_string(), text: "".to_string() };
    let d_sf = MockSourceFile { file_name: "/d.ts".to_string(), text: "".to_string() };
    
    let path = graph.find_path(&a_sf, &d_sf);
    assert!(path.is_some());
    assert_eq!(import_path_to_string(&fs, &path.unwrap()), "a,b,c,d");
}

#[test]
fn test_synthetic_import() {
    let fs = create_fs_from_graph("a;b");
    let graph = ImportGraph::new(&fs);
    
    let a_sf = MockSourceFile { file_name: "/a.ts".to_string(), text: fs.read_file(&AbsoluteFsPath::from("/a.ts")).unwrap() };
    let b_sf = MockSourceFile { file_name: "/b.ts".to_string(), text: "".to_string() };

    assert!(graph.imports_of(&a_sf).is_empty());
    
    graph.add_synthetic_import(&a_sf, &b_sf);
    
    let imports = graph.imports_of(&a_sf);
    assert!(imports.contains(&AbsoluteFsPath::from("/b.ts")));
}
