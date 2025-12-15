//! Generate pipe creation instructions.
//!
//! Corresponds to packages/compiler/src/template/pipeline/src/phases/pipe_creation.ts

use crate::template::pipeline::ir;

use crate::template::pipeline::src::compilation::{
    CompilationJob, ComponentCompilationJob, CompilationUnit,
};

pub fn create_pipes(job: &mut dyn CompilationJob) {
    // Dispatch based on job kind or cast to ComponentCompilationJob
    // Since we likely only run this on Component jobs (pipes in templates)
    // Host bindings don't usually have pipes?
    // TS doesn't check job kind.
    
    // We'll use the unsafe cast pattern for now as per other files
     if let Some(component_job) = unsafe {
        let job_ptr = job as *mut dyn CompilationJob;
        // Check if it is actually a ComponentCompilationJob?
        // job.kind() is available on trait.
        if job.kind() == crate::template::pipeline::src::compilation::CompilationJobKind::Tmpl {
            Some(&mut *(job_ptr as *mut ComponentCompilationJob))
        } else {
            None
        }
    } {
        process_pipe_bindings_in_view(&mut component_job.root, job.compatibility());
        for unit in component_job.views.values_mut() {
            process_pipe_bindings_in_view(unit, job.compatibility());
        }
    }
}

fn process_pipe_bindings_in_view(unit: &mut dyn CompilationUnit, compatibility: ir::CompatibilityMode) {
    // We need to collect pipe bindings first to avoid borrowing conflicts
    // iterating update ops while modifying create ops is allowed if lists are separate.
    // unit.update() returns &OpList. unit.create_mut() returns &mut OpList.
    // Rust borrow checker is fine if we split borrows.
    // BUT generic CompilationUnit trait getters might not allow splitting borrows easily if methods take &self.
    // unit.update() takes &self. unit.create_mut() takes &mut self.
    // You cannot preserve &self borrow while taking &mut self.
    
    // Solution: Collect bindings into a Vec, then mutate create list.
    
    struct PipeInfo {
        target: ir::XrefId,
        target_slot: ir::SlotHandle,
        name: String,
        // for compatibility mode
        update_op_target: Option<ir::XrefId>, 
    }
    
    let mut pipes = Vec::new();
    let _unit_xref = unit.xref();
    
    for op in unit.update_mut().iter_mut() {
        let update_op_target_cache = if compatibility == ir::CompatibilityMode::TemplateDefinitionBuilder {
             get_op_target(op.as_ref())
        } else {
             None
        };

        ir::visit_expressions_in_op(op.as_mut(), &mut |expr, _flags: ir::VisitorContextFlag| {
             if !ir::is_ir_expression(expr) { return; }
             
             let pipe_info = if let Some(ir_expr) = ir::as_ir_expression(&expr) {
                 match ir_expr {
                     ir::IRExpression::PipeBinding(pb) => {
                         Some((pb.target, pb.target_slot.clone(), pb.name.clone()))
                     }
                     ir::IRExpression::PipeBindingVariadic(pb) => {
                         Some((pb.target, pb.target_slot.clone(), pb.name.clone()))
                     }
                     _ => None
                 }
             } else {
                 None
             };

             if let Some((target, target_slot, name)) = pipe_info {
                 pipes.push(PipeInfo {
                     target,
                     target_slot,
                     name,
                     update_op_target: update_op_target_cache,
                 });
             }
        });
    }
    
    // Process gathered pipes
    for pipe in pipes {
        let create_op = ir::ops::create::create_pipe_op(pipe.target, pipe.target_slot, pipe.name);
        
        if compatibility == ir::CompatibilityMode::TemplateDefinitionBuilder {
            if let Some(target) = pipe.update_op_target {
                add_pipe_to_creation_block(unit, target, create_op);
            }
        } else {
            unit.create_mut().push(create_op);
        }
    }
}

fn add_pipe_to_creation_block(
    unit: &mut dyn CompilationUnit,
    after_target_xref: ir::XrefId,
    pipe_op: Box<dyn ir::CreateOp + Send + Sync>, // CreatePipeOp
) {
    // Insert logic
    // We need to iterate create list and find insertion point.
    // OpList support generic iteration?
    // unit.create_mut() gives OpList.
    // We need indices logic again because explicit linked list manipulation not available in Vec-based OpList easily?
    // Actually OpList has insert_at.
    
    let mut insert_idx = None;
    let create_list = unit.create();
    
    for (i, op) in create_list.iter().enumerate() {
        // Skip ListStart? Vec doesn't have ListStart node usually (unless explicit)
        
        // check ConsumesSlotOpTrait
        // check xref
        
        if let Some(consumes) = as_consumes_slot(op.as_ref()) {
            if consumes.xref() == after_target_xref {
                // Found tentative point
                // Skip subsequent pipe ops
                let mut offset = 1;
                while i + offset < create_list.len() {
                    if create_list.get(i + offset).unwrap().kind() == ir::OpKind::Pipe {
                        offset += 1;
                    } else {
                        break;
                    }
                }
                insert_idx = Some(i + offset);
                break;
            }
        }
    }
    
    if let Some(idx) = insert_idx {
        unit.create_mut().insert_at(idx, pipe_op);
    } else {
         // Panic or fallback?
         // TS throws error
    }
}

// Helpers

fn get_op_target(op: &dyn ir::Op) -> Option<ir::XrefId> {
    // Attempt to get target xref from Op using ConsumesSlotOpTrait
    // Most ops that have dependencies implement this or explicitly have a target field
    // We can try downcasting to known types or checking trait
    if let Some(consumes) = as_consumes_slot(op) {
        return Some(consumes.xref());
    }
    
    // Fallback for specific ops that might not implement the trait but have a target
    match op.kind() {
        ir::OpKind::StyleProp => {
             if let Some(op) = op.as_any().downcast_ref::<ir::ops::update::StylePropOp>() {
                 return Some(op.target);
             }
        }
        ir::OpKind::ClassProp => {
             if let Some(op) = op.as_any().downcast_ref::<ir::ops::update::ClassPropOp>() {
                 return Some(op.target);
             }
        }
        ir::OpKind::StyleMap => {
             if let Some(op) = op.as_any().downcast_ref::<ir::ops::update::StyleMapOp>() {
                 return Some(op.target);
             }
        }
        ir::OpKind::ClassMap => {
             if let Some(op) = op.as_any().downcast_ref::<ir::ops::update::ClassMapOp>() {
                 return Some(op.target);
             }
        }
        ir::OpKind::Property => {
             if let Some(op) = op.as_any().downcast_ref::<ir::ops::update::PropertyOp>() {
                 return Some(op.target);
             }
        }
        ir::OpKind::Attribute => {
             if let Some(op) = op.as_any().downcast_ref::<ir::ops::update::AttributeOp>() {
                 return Some(op.target);
             }
        }
        _ => {}
    }
    None
}

fn as_consumes_slot(op: &dyn ir::Op) -> Option<&dyn ir::ConsumesSlotOpTrait> {
    // Helper to downcast/access trait
    // Since we can't easily check for trait implementation dynamically on a trait object without Any,
    // and ir::Op implements Any, we can try to downcast to known types that implement ConsumesSlotOpTrait
    
    // Create Ops
    if let Some(op) = op.as_any().downcast_ref::<ir::ops::create::ElementStartOp>() { return Some(op); }
    if let Some(op) = op.as_any().downcast_ref::<ir::ops::create::ElementOp>() { return Some(op); }
    if let Some(op) = op.as_any().downcast_ref::<ir::ops::create::TemplateOp>() { return Some(op); }
    if let Some(op) = op.as_any().downcast_ref::<ir::ops::create::ContainerStartOp>() { return Some(op); }
    if let Some(op) = op.as_any().downcast_ref::<ir::ops::create::ContainerOp>() { return Some(op); }
    if let Some(op) = op.as_any().downcast_ref::<ir::ops::create::PipeOp>() { return Some(op); }
    if let Some(op) = op.as_any().downcast_ref::<ir::ops::create::TextOp>() { return Some(op); }
    
    // We could add more if needed
    None
}
