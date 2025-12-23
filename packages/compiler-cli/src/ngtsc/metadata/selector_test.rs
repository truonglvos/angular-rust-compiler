#[cfg(test)]
mod tests {
    
use std::path::PathBuf;

use crate::ngtsc::metadata::{OxcMetadataReader, MetadataReader, DecoratorMetadata};
use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_span::SourceType;

#[test]
fn test_extract_selector() {
    let source = r#"
        import { Component } from '@angular/core';

        @Component({
            selector: 'app-root',
            template: '<div></div>',
            standalone: true,
            inputs: ['foo', 'bar: baz'],
            outputs: ['click'],
            exportAs: 'myApp'
        })
        export class AppComponent {}
    "#;

    let allocator = Allocator::default();
    let source_type = SourceType::default().with_typescript(true).with_module(true);
    let parser = Parser::new(&allocator, source, source_type);
    let ret = parser.parse();

    assert!(ret.errors.is_empty(), "Parser errors: {:?}", ret.errors);

    let reader = OxcMetadataReader;
    let directives = reader.get_directive_metadata(&ret.program, &PathBuf::from("test.ts"));

    assert_eq!(directives.len(), 1);
    
    if let DecoratorMetadata::Directive(meta) = &directives[0] {
        assert_eq!(meta.t2.name, "AppComponent");
        assert_eq!(meta.t2.selector, Some("app-root".to_string()));
        assert!(meta.t2.is_component);
        assert!(meta.is_standalone);
        assert_eq!(meta.t2.export_as, Some(vec!["myApp".to_string()]));
        
        // Check inputs
        let foo_input = meta.t2.inputs.get_by_class_property_name("foo");
        assert!(foo_input.is_some());
        assert_eq!(foo_input.unwrap().binding_property_name, "foo");

        let bar_input = meta.t2.inputs.get_by_class_property_name("bar");
        assert!(bar_input.is_some());
        assert_eq!(bar_input.unwrap().binding_property_name, "baz");

        // Check outputs
        let click_output = meta.t2.outputs.get_by_class_property_name("click");
        assert!(click_output.is_some());
        assert_eq!(click_output.unwrap().binding_property_name, "click");
        
        // Check template
        assert_eq!(meta.component.as_ref().and_then(|c| c.template.as_ref()).map(|s| s.as_str()), Some("<div></div>"));
    } else {
        panic!("Expected Directive metadata");
    }
}

#[test]
fn test_extract_component_assets() {
    let source = r#"
        import { Component } from '@angular/core';

        @Component({
            selector: 'app-assets',
            templateUrl: './assets.component.html',
            styleUrls: ['./assets.component.css', './other.css'],
            styles: ['div { color: red; }']
        })
        export class AssetsComponent {}
    "#;

    let allocator = Allocator::default();
    let source_type = SourceType::default().with_typescript(true).with_module(true);
    let parser = Parser::new(&allocator, source, source_type);
    let ret = parser.parse();

    assert!(ret.errors.is_empty(), "Parser errors: {:?}", ret.errors);

    let reader = OxcMetadataReader;
    let directives = reader.get_directive_metadata(&ret.program, &PathBuf::from("test.ts"));

    assert_eq!(directives.len(), 1);
    
    if let DecoratorMetadata::Directive(meta) = &directives[0] {
        assert_eq!(meta.t2.name, "AssetsComponent");
        assert_eq!(meta.component.as_ref().and_then(|c| c.template_url.as_ref()), Some(&"./assets.component.html".to_string()));
        assert_eq!(meta.component.as_ref().and_then(|c| c.styles.as_ref()), Some(&vec!["div { color: red; }".to_string()]));
        assert_eq!(meta.component.as_ref().and_then(|c| c.style_urls.as_ref()), Some(&vec!["./assets.component.css".to_string(), "./other.css".to_string()]));
    } else {
        panic!("Expected Directive metadata");
    }
}

#[test]
fn test_extract_directive_selector() {
    let source = r#"
        import { Directive } from '@angular/core';

        @Directive({
            selector: 'app-test',
            inputs: { 'val': 'value' },
            outputs: { 'change': 'onChange' },
            standalone: false
        })
        export class TestDirective {}
    "#;

    let allocator = Allocator::default();
    let source_type = SourceType::default().with_typescript(true).with_module(true);
    let parser = Parser::new(&allocator, source, source_type);
    let ret = parser.parse();

    assert!(ret.errors.is_empty(), "Parser errors: {:?}", ret.errors);

    let reader = OxcMetadataReader;
    let directives = reader.get_directive_metadata(&ret.program, &PathBuf::from("test.ts"));

    assert_eq!(directives.len(), 1);
    
    if let DecoratorMetadata::Directive(meta) = &directives[0] {
        assert_eq!(meta.t2.name, "TestDirective");
        assert_eq!(meta.t2.selector, Some("app-test".to_string()));
        assert!(!meta.t2.is_component);
        assert!(!meta.is_standalone);

        // Check inputs object syntax
        let val_input = meta.t2.inputs.get_by_class_property_name("val");
        assert!(val_input.is_some());
        assert_eq!(val_input.unwrap().binding_property_name, "value");

        // Check outputs object syntax
        let change_output = meta.t2.outputs.get_by_class_property_name("change");
        assert!(change_output.is_some());
        assert_eq!(change_output.unwrap().binding_property_name, "onChange");
    } else {
        panic!("Expected Directive metadata");
    }
}
}

