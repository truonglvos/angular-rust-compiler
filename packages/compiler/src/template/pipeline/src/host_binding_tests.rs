
#[cfg(test)]
mod tests {
    use crate::constant_pool::ConstantPool;
    use crate::output::output_ast::Expression;
    use crate::output::output_ast::{LiteralExpr, LiteralValue};
    use crate::template::pipeline::src::ingest::{ingest_host_binding, HostBindingInput};
    use crate::template::pipeline::src::phases::run_host;
    use std::collections::HashMap;

    #[test]
    fn test_extract_static_host_attributes() {
        // Reproduce the issue:
        // host: { class: 'foo', style: 'color: red' }
        // expected: hostAttrs: ['class', 'foo', 'style', 'color: red'] (or similar structure in consts)
        // actual (before fix): hostBindings with classProp/styleProp instructions

        let mut attributes = HashMap::new();
        attributes.insert(
            "class".to_string(),
            Expression::Literal(LiteralExpr {
                value: LiteralValue::String("foo".to_string()),
                type_: None,
                source_span: None,
            }),
        );
        attributes.insert(
            "style".to_string(),
            Expression::Literal(LiteralExpr {
                value: LiteralValue::String("color: red".to_string()),
                type_: None,
                source_span: None,
            }),
        );

        let input = HostBindingInput {
            component_name: "TestComp".to_string(),
            component_selector: "test-comp".to_string(),
            properties: HashMap::new(),
            attributes,
            events: HashMap::new(),
        };

        let mut job = ingest_host_binding(input, ConstantPool::default());

        run_host(&mut job);

        // Check hostAttrs (job.root.attributes)
        assert!(
            job.root.attributes.is_some(),
            "hostAttrs should be present"
        );
        
        if let Some(Expression::LiteralArray(arr)) = &job.root.attributes {
             // We expect something like [AttributeMarker.Classes, "foo", AttributeMarker.Styles, "color", "red"] or similar
             // Inspecting the actual structure might be intricate, but just asserting it's not empty and contains "foo" is a good start.
             
             let debug_str = format!("{:?}", arr);
             assert!(debug_str.contains("foo"), "hostAttrs should contain class 'foo'");
             // Styles are parsed into key/value pairs
             assert!(debug_str.contains("color"), "hostAttrs should contain style key 'color'");
             assert!(debug_str.contains("red"), "hostAttrs should contain style value 'red'");
        } else {
             panic!("hostAttrs should be a LiteralArray");
        }

        // Check hostBindings function - it should NOT contain generic style/class instruction if they were extracted
        // Only if they were DYNAMIC bindings they would stay.
        // Static attributes should form hostAttrs and be REMOVED from the update block.
        assert!(
            job.root.update.is_empty(),
            "Update block should be empty for static host attributes, but found: {:?}",
            job.root.update
        );
    }
}
