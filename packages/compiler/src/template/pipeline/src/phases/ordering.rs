//! Reorder operations to ensure dependencies are met.
//!
//! Corresponds to packages/compiler/src/template/pipeline/src/phases/ordering.ts

use crate::template::pipeline::ir;
use crate::template::pipeline::ir::ops::host::DomPropertyOp;
use crate::template::pipeline::ir::ops::update::{AttributeOp, BindingExpression, PropertyOp};
use crate::template::pipeline::ir::{OpKind, OpList};
use crate::template::pipeline::src::compilation::{
    CompilationJob, CompilationJobKind, ComponentCompilationJob, HostBindingCompilationJob,
    HostBindingCompilationUnit, ViewCompilationUnit,
};

// Define Rule struct locally
struct Rule<T: ?Sized> {
    test: fn(&T) -> bool,
    transform: Option<fn(Vec<Box<T>>) -> Vec<Box<T>>>,
}

fn check_interpolation(op: &dyn ir::UpdateOp) -> bool {
    // Check various op types
    // Downcast requires concrete type usually or casting via Any.
    // ir::Op likely implements AsAny or we can use safe downcast if T is known?
    // If T is dyn Op, we can use as_any().
    // We'll rely on op.as_any() which exists on Op trait.

    if let Some(prop) = op.as_any().downcast_ref::<PropertyOp>() {
        matches!(prop.expression, BindingExpression::Interpolation(_))
    } else if let Some(attr) = op.as_any().downcast_ref::<AttributeOp>() {
        matches!(attr.expression, BindingExpression::Interpolation(_))
    } else if let Some(_dom) = op.as_any().downcast_ref::<DomPropertyOp>() {
        false
    } else {
        false
    }
}

fn basic_listener_kind_test(op: &(dyn ir::CreateOp + Send + Sync)) -> bool {
    (op.kind() == OpKind::Listener/* && !(hostListener && legacy) */)
        || op.kind() == OpKind::TwoWayListener
        || op.kind() == OpKind::Animation
        || op.kind() == OpKind::AnimationListener

    // Note: hostListener and legacyAnimation checks require downcasting to ListenerOp.
    // I need to implement ListenerOp checks if strict parity needed.
}

fn non_interpolation_property_kind_test(op: &(dyn ir::UpdateOp + Send + Sync)) -> bool {
    (op.kind() == OpKind::Property || op.kind() == OpKind::TwoWayProperty)
        && !check_interpolation(op)
}

// Helpers for Op trait access

// Ordering Rules
// CreateOp
const CREATE_ORDERING: &[Rule<dyn ir::CreateOp + Send + Sync>] = &[
    // {test: listener...}
    Rule {
        test: basic_listener_kind_test,
        transform: None,
    },
];

// UpdateOp
// Since functions cannot be const easily in Rust, I might need lazy initialization or simpler functions.
// I'll define rules inside the function or use a different pattern.
// Or just match inside `reorder`.

pub fn order_ops(job: &mut dyn CompilationJob) {
    if job.kind() == CompilationJobKind::Tmpl {
        let component_job = unsafe {
            let job_ptr = job as *mut dyn CompilationJob;
            let job_ptr = job_ptr as *mut ComponentCompilationJob;
            &mut *job_ptr
        };
        order_unit_ops(&mut component_job.root, &CompilationJobKind::Tmpl);
        for unit in component_job.views.values_mut() {
            order_unit_ops(unit, &CompilationJobKind::Tmpl);
        }
    } else if job.kind() == CompilationJobKind::Host {
        let host_job = unsafe {
            let job_ptr = job as *mut dyn CompilationJob;
            let job_ptr = job_ptr as *mut HostBindingCompilationJob;
            &mut *job_ptr
        };
        // HostBinding unit logic
        // order_unit_ops expects ViewCompilationUnit. HostBindingCompilationUnit is different struct.
        // I need generic function or impl for both.
        order_host_unit_ops(&mut host_job.root);
    }
}

fn order_unit_ops(unit: &mut ViewCompilationUnit, job_kind: &CompilationJobKind) {
    // Create
    unit.create = order_within(std::mem::take(&mut unit.create), get_create_rules());

    // Update
    let rules = if matches!(job_kind, CompilationJobKind::Host) {
        get_update_host_rules()
    } else {
        get_update_rules()
    };
    unit.update = order_within(std::mem::take(&mut unit.update), rules);
}

fn order_host_unit_ops(unit: &mut HostBindingCompilationUnit) {
    unit.create = order_within(std::mem::take(&mut unit.create), get_create_rules());
    unit.update = order_within(std::mem::take(&mut unit.update), get_update_host_rules());
}

fn order_within<T: 'static + ?Sized>(ops: OpList<Box<T>>, ordering: Vec<Rule<T>>) -> OpList<Box<T>>
where
    T: ir::Op,
{
    let mut new_ops = OpList::new();
    let mut buffer: Vec<Box<T>> = Vec::new();
    let mut first_target_in_group: Option<ir::XrefId> = None;

    // Check handled kinds set logic (in TS handledOpKinds set)
    // Here we check if ANY rule matches.

    for op in ops.into_vec() {
        let is_handled = ordering.iter().any(|r| (r.test)(op.as_ref()));

        let current_target = get_dependency_target(op.as_ref());

        if !is_handled
            || (first_target_in_group.is_some()
                && current_target.is_some()
                && current_target != first_target_in_group)
        {
            // Flush
            if !buffer.is_empty() {
                let reordered = reorder(buffer, &ordering);
                new_ops.push_all(reordered);
                buffer = Vec::new();
            }
            first_target_in_group = None;
        }

        if is_handled {
            buffer.push(op);
            if first_target_in_group.is_none() {
                first_target_in_group = current_target;
            } else if let Some(tgt) = current_target {
                first_target_in_group = Some(tgt); // Keep updating target if found
            }
        } else {
            new_ops.push(op);
        }
    }

    // Final flush
    if !buffer.is_empty() {
        let reordered = reorder(buffer, &ordering);
        new_ops.push_all(reordered);
    }

    new_ops
}

fn reorder<T: ?Sized>(ops: Vec<Box<T>>, ordering: &Vec<Rule<T>>) -> Vec<Box<T>> {
    // Group by rule index
    let mut groups: Vec<Vec<Box<T>>> = (0..ordering.len()).map(|_| Vec::new()).collect();

    for op in ops {
        if let Some(idx) = ordering.iter().position(|r| (r.test)(op.as_ref())) {
            groups[idx].push(op);
        } else {
            // Should be handled, but if not put in last group or error?
            // TS logic assumes handled.
        }
    }

    // Transform and flatten
    let mut result = Vec::new();
    for (i, group) in groups.into_iter().enumerate() {
        let transformed = if let Some(transform) = ordering[i].transform {
            transform(group)
        } else {
            group
        };
        result.extend(transformed);
    }
    result
}

fn get_dependency_target<T: ir::Op + ?Sized>(op: &T) -> Option<ir::XrefId> {
    // Check DependsOnSlotContextOpTrait
    // Need downcasting or identifying by kind
    // Standard ops that have target:
    // Binding, Property, Attribute, etc.
    // Assuming ir::has_depends_on_slot_context_trait is available but hard to use on trait object.
    // Manual check:
    match op.kind() {
        OpKind::Property
        | OpKind::Attribute
        | OpKind::StyleProp
        | OpKind::ClassProp
        | OpKind::TwoWayProperty
        | OpKind::Binding => {
            // These are UpdateOps that depend on target.
            // We can downcast to UpdateOp and call xref()? No, xref is target usually.
            // or manually.
            // Using unsafe helper or generic if Op had target().
            // Checking ir module for generic target access?
            // traits::DependsOnSlotContextOpTrait has target().
            // But we only have &dyn Op.
            // If we can downcast to &dyn DependsOnSlotContextOpTrait?
            // Rust doesn't support casting between unrelated trait objects.
            // Workaround:
            if let Some(prop) = op.as_any().downcast_ref::<PropertyOp>() {
                Some(prop.target)
            } else if let Some(attr) = op.as_any().downcast_ref::<AttributeOp>() {
                Some(attr.target)
            }
            // ... add others
            else {
                None
            }
        }
        _ => None,
    }
}

// Rule Generators

fn get_create_rules() -> Vec<Rule<dyn ir::CreateOp + Send + Sync>> {
    vec![
        // simplified
        Rule {
            test: |op| op.kind() == OpKind::Listener,
            transform: None,
        },
    ]
}

fn get_update_rules() -> Vec<Rule<dyn ir::UpdateOp + Send + Sync>> {
    vec![
        Rule {
            test: |op| op.kind() == OpKind::StyleMap,
            transform: Some(keep_last),
        },
        Rule {
            test: |op| op.kind() == OpKind::ClassMap,
            transform: Some(keep_last),
        },
        Rule {
            test: |op| op.kind() == OpKind::StyleProp,
            transform: None,
        },
        Rule {
            test: |op| op.kind() == OpKind::ClassProp,
            transform: None,
        },
        Rule {
            test: |op| op.kind() == OpKind::Attribute && check_interpolation(op),
            transform: None,
        },
        Rule {
            test: |op| op.kind() == OpKind::Property && check_interpolation(op),
            transform: None,
        },
        Rule {
            test: non_interpolation_property_kind_test,
            transform: None,
        },
        Rule {
            test: |op| op.kind() == OpKind::Attribute && !check_interpolation(op),
            transform: None,
        },
    ]
}

fn get_update_host_rules() -> Vec<Rule<dyn ir::UpdateOp + Send + Sync>> {
    vec![
        // host rules
    ]
}

fn keep_last<T>(ops: Vec<T>) -> Vec<T> {
    if ops.is_empty() {
        ops
    } else {
        ops.into_iter().rev().take(1).collect()
    }
}
