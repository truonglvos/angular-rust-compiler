//! Render3 Identifiers
//!
//! Corresponds to packages/compiler/src/render3/r3_identifiers.ts
//! Contains all Angular runtime identifiers/symbols used in generated code

use crate::output::output_ast::ExternalReference;

const CORE: &str = "@angular/core";

/// Angular runtime identifiers used in generated code
pub struct Identifiers;

impl Identifiers {
    /* Methods */
    pub const NEW_METHOD: &'static str = "factory";
    pub const TRANSFORM_METHOD: &'static str = "transform";
    pub const PATCH_DEPS: &'static str = "patchedDeps";

    fn make_ref(name: Option<&str>) -> ExternalReference {
        ExternalReference {
            module_name: Some(CORE.to_string()),
            name: name.map(|s| s.to_string()),
            runtime: None,
        }
    }

    pub fn core() -> ExternalReference {
        Self::make_ref(None)
    }

    /* Instructions */
    pub fn namespace_html() -> ExternalReference {
        Self::make_ref(Some("ɵɵnamespaceHTML"))
    }

    pub fn namespace_math_ml() -> ExternalReference {
        Self::make_ref(Some("ɵɵnamespaceMathML"))
    }

    pub fn namespace_svg() -> ExternalReference {
        Self::make_ref(Some("ɵɵnamespaceSVG"))
    }

    pub fn element() -> ExternalReference {
        Self::make_ref(Some("ɵɵelement"))
    }

    pub fn element_start() -> ExternalReference {
        Self::make_ref(Some("ɵɵelementStart"))
    }

    pub fn element_end() -> ExternalReference {
        Self::make_ref(Some("ɵɵelementEnd"))
    }

    pub fn dom_element() -> ExternalReference {
        Self::make_ref(Some("ɵɵdomElement"))
    }

    pub fn dom_element_start() -> ExternalReference {
        Self::make_ref(Some("ɵɵdomElementStart"))
    }

    pub fn dom_element_end() -> ExternalReference {
        Self::make_ref(Some("ɵɵdomElementEnd"))
    }

    pub fn dom_element_container() -> ExternalReference {
        Self::make_ref(Some("ɵɵdomElementContainer"))
    }

    pub fn dom_element_container_start() -> ExternalReference {
        Self::make_ref(Some("ɵɵdomElementContainerStart"))
    }

    pub fn dom_element_container_end() -> ExternalReference {
        Self::make_ref(Some("ɵɵdomElementContainerEnd"))
    }

    pub fn dom_template() -> ExternalReference {
        Self::make_ref(Some("ɵɵdomTemplate"))
    }

    pub fn dom_listener() -> ExternalReference {
        Self::make_ref(Some("ɵɵdomListener"))
    }

    pub fn advance() -> ExternalReference {
        Self::make_ref(Some("ɵɵadvance"))
    }

    pub fn synthetic_host_property() -> ExternalReference {
        Self::make_ref(Some("ɵɵsyntheticHostProperty"))
    }

    pub fn synthetic_host_listener() -> ExternalReference {
        Self::make_ref(Some("ɵɵsyntheticHostListener"))
    }

    pub fn attribute() -> ExternalReference {
        Self::make_ref(Some("ɵɵattribute"))
    }

    pub fn class_prop() -> ExternalReference {
        Self::make_ref(Some("ɵɵclassProp"))
    }

    pub fn element_container_start() -> ExternalReference {
        Self::make_ref(Some("ɵɵelementContainerStart"))
    }

    pub fn element_container_end() -> ExternalReference {
        Self::make_ref(Some("ɵɵelementContainerEnd"))
    }

    pub fn element_container() -> ExternalReference {
        Self::make_ref(Some("ɵɵelementContainer"))
    }

    pub fn style_map() -> ExternalReference {
        Self::make_ref(Some("ɵɵstyleMap"))
    }

    pub fn class_map() -> ExternalReference {
        Self::make_ref(Some("ɵɵclassMap"))
    }

    pub fn style_prop() -> ExternalReference {
        Self::make_ref(Some("ɵɵstyleProp"))
    }

    /* Interpolation instructions */
    pub fn interpolate() -> ExternalReference {
        Self::make_ref(Some("ɵɵinterpolate"))
    }

    pub fn interpolate1() -> ExternalReference {
        Self::make_ref(Some("ɵɵinterpolate1"))
    }

    pub fn interpolate2() -> ExternalReference {
        Self::make_ref(Some("ɵɵinterpolate2"))
    }

    pub fn interpolate3() -> ExternalReference {
        Self::make_ref(Some("ɵɵinterpolate3"))
    }

    pub fn interpolate4() -> ExternalReference {
        Self::make_ref(Some("ɵɵinterpolate4"))
    }

    pub fn interpolate5() -> ExternalReference {
        Self::make_ref(Some("ɵɵinterpolate5"))
    }

    pub fn interpolate6() -> ExternalReference {
        Self::make_ref(Some("ɵɵinterpolate6"))
    }

    pub fn interpolate7() -> ExternalReference {
        Self::make_ref(Some("ɵɵinterpolate7"))
    }

    pub fn interpolate8() -> ExternalReference {
        Self::make_ref(Some("ɵɵinterpolate8"))
    }

    pub fn interpolate_v() -> ExternalReference {
        Self::make_ref(Some("ɵɵinterpolateV"))
    }

    pub fn next_context() -> ExternalReference {
        Self::make_ref(Some("ɵɵnextContext"))
    }

    pub fn reset_view() -> ExternalReference {
        Self::make_ref(Some("ɵɵresetView"))
    }

    pub fn template_create() -> ExternalReference {
        Self::make_ref(Some("ɵɵtemplate"))
    }

    /* Defer instructions */
    pub fn defer() -> ExternalReference {
        Self::make_ref(Some("ɵɵdefer"))
    }

    pub fn defer_when() -> ExternalReference {
        Self::make_ref(Some("ɵɵdeferWhen"))
    }

    pub fn defer_on_idle() -> ExternalReference {
        Self::make_ref(Some("ɵɵdeferOnIdle"))
    }

    pub fn defer_on_immediate() -> ExternalReference {
        Self::make_ref(Some("ɵɵdeferOnImmediate"))
    }

    pub fn defer_on_timer() -> ExternalReference {
        Self::make_ref(Some("ɵɵdeferOnTimer"))
    }

    pub fn defer_on_hover() -> ExternalReference {
        Self::make_ref(Some("ɵɵdeferOnHover"))
    }

    pub fn defer_on_interaction() -> ExternalReference {
        Self::make_ref(Some("ɵɵdeferOnInteraction"))
    }

    pub fn defer_on_viewport() -> ExternalReference {
        Self::make_ref(Some("ɵɵdeferOnViewport"))
    }

    pub fn defer_prefetch_when() -> ExternalReference {
        Self::make_ref(Some("ɵɵdeferPrefetchWhen"))
    }

    pub fn defer_prefetch_on_idle() -> ExternalReference {
        Self::make_ref(Some("ɵɵdeferPrefetchOnIdle"))
    }

    pub fn defer_prefetch_on_immediate() -> ExternalReference {
        Self::make_ref(Some("ɵɵdeferPrefetchOnImmediate"))
    }

    pub fn defer_prefetch_on_timer() -> ExternalReference {
        Self::make_ref(Some("ɵɵdeferPrefetchOnTimer"))
    }

    pub fn defer_prefetch_on_hover() -> ExternalReference {
        Self::make_ref(Some("ɵɵdeferPrefetchOnHover"))
    }

    pub fn defer_prefetch_on_interaction() -> ExternalReference {
        Self::make_ref(Some("ɵɵdeferPrefetchOnInteraction"))
    }

    pub fn defer_prefetch_on_viewport() -> ExternalReference {
        Self::make_ref(Some("ɵɵdeferPrefetchOnViewport"))
    }

    pub fn defer_hydrate_when() -> ExternalReference {
        Self::make_ref(Some("ɵɵdeferHydrateWhen"))
    }

    pub fn defer_hydrate_never() -> ExternalReference {
        Self::make_ref(Some("ɵɵdeferHydrateNever"))
    }

    pub fn defer_hydrate_on_idle() -> ExternalReference {
        Self::make_ref(Some("ɵɵdeferHydrateOnIdle"))
    }

    pub fn defer_hydrate_on_immediate() -> ExternalReference {
        Self::make_ref(Some("ɵɵdeferHydrateOnImmediate"))
    }

    pub fn defer_hydrate_on_timer() -> ExternalReference {
        Self::make_ref(Some("ɵɵdeferHydrateOnTimer"))
    }

    pub fn defer_hydrate_on_hover() -> ExternalReference {
        Self::make_ref(Some("ɵɵdeferHydrateOnHover"))
    }

    pub fn defer_hydrate_on_interaction() -> ExternalReference {
        Self::make_ref(Some("ɵɵdeferHydrateOnInteraction"))
    }

    pub fn defer_hydrate_on_viewport() -> ExternalReference {
        Self::make_ref(Some("ɵɵdeferHydrateOnViewport"))
    }

    pub fn defer_enable_timer_scheduling() -> ExternalReference {
        Self::make_ref(Some("ɵɵdeferEnableTimerScheduling"))
    }

    /* Control flow instructions */
    pub fn conditional_create() -> ExternalReference {
        Self::make_ref(Some("ɵɵconditionalCreate"))
    }

    pub fn conditional_branch_create() -> ExternalReference {
        Self::make_ref(Some("ɵɵconditionalBranchCreate"))
    }

    pub fn conditional() -> ExternalReference {
        Self::make_ref(Some("ɵɵconditional"))
    }

    pub fn repeater() -> ExternalReference {
        Self::make_ref(Some("ɵɵrepeater"))
    }

    pub fn repeater_create() -> ExternalReference {
        Self::make_ref(Some("ɵɵrepeaterCreate"))
    }

    pub fn repeater_track_by_index() -> ExternalReference {
        Self::make_ref(Some("ɵɵrepeaterTrackByIndex"))
    }

    pub fn repeater_track_by_identity() -> ExternalReference {
        Self::make_ref(Some("ɵɵrepeaterTrackByIdentity"))
    }

    pub fn component_instance() -> ExternalReference {
        Self::make_ref(Some("ɵɵcomponentInstance"))
    }

    pub fn text() -> ExternalReference {
        Self::make_ref(Some("ɵɵtext"))
    }

    pub fn enable_bindings() -> ExternalReference {
        Self::make_ref(Some("ɵɵenableBindings"))
    }

    pub fn disable_bindings() -> ExternalReference {
        Self::make_ref(Some("ɵɵdisableBindings"))
    }

    pub fn get_current_view() -> ExternalReference {
        Self::make_ref(Some("ɵɵgetCurrentView"))
    }

    /* Text interpolation instructions */
    pub fn text_interpolate() -> ExternalReference {
        Self::make_ref(Some("ɵɵtextInterpolate"))
    }

    pub fn text_interpolate1() -> ExternalReference {
        Self::make_ref(Some("ɵɵtextInterpolate1"))
    }

    pub fn text_interpolate2() -> ExternalReference {
        Self::make_ref(Some("ɵɵtextInterpolate2"))
    }

    pub fn text_interpolate3() -> ExternalReference {
        Self::make_ref(Some("ɵɵtextInterpolate3"))
    }

    pub fn text_interpolate4() -> ExternalReference {
        Self::make_ref(Some("ɵɵtextInterpolate4"))
    }

    pub fn text_interpolate5() -> ExternalReference {
        Self::make_ref(Some("ɵɵtextInterpolate5"))
    }

    pub fn text_interpolate6() -> ExternalReference {
        Self::make_ref(Some("ɵɵtextInterpolate6"))
    }

    pub fn text_interpolate7() -> ExternalReference {
        Self::make_ref(Some("ɵɵtextInterpolate7"))
    }

    pub fn text_interpolate8() -> ExternalReference {
        Self::make_ref(Some("ɵɵtextInterpolate8"))
    }

    pub fn text_interpolate_v() -> ExternalReference {
        Self::make_ref(Some("ɵɵtextInterpolateV"))
    }

    pub fn restore_view() -> ExternalReference {
        Self::make_ref(Some("ɵɵrestoreView"))
    }

    /* Pure function instructions */
    pub fn pure_function0() -> ExternalReference {
        Self::make_ref(Some("ɵɵpureFunction0"))
    }

    pub fn pure_function1() -> ExternalReference {
        Self::make_ref(Some("ɵɵpureFunction1"))
    }

    pub fn pure_function2() -> ExternalReference {
        Self::make_ref(Some("ɵɵpureFunction2"))
    }

    pub fn pure_function3() -> ExternalReference {
        Self::make_ref(Some("ɵɵpureFunction3"))
    }

    pub fn pure_function4() -> ExternalReference {
        Self::make_ref(Some("ɵɵpureFunction4"))
    }

    pub fn pure_function5() -> ExternalReference {
        Self::make_ref(Some("ɵɵpureFunction5"))
    }

    pub fn pure_function6() -> ExternalReference {
        Self::make_ref(Some("ɵɵpureFunction6"))
    }

    pub fn pure_function7() -> ExternalReference {
        Self::make_ref(Some("ɵɵpureFunction7"))
    }

    pub fn pure_function8() -> ExternalReference {
        Self::make_ref(Some("ɵɵpureFunction8"))
    }

    pub fn pure_function_v() -> ExternalReference {
        Self::make_ref(Some("ɵɵpureFunctionV"))
    }

    /* Pipe bind instructions */
    pub fn pipe_bind1() -> ExternalReference {
        Self::make_ref(Some("ɵɵpipeBind1"))
    }

    pub fn pipe_bind2() -> ExternalReference {
        Self::make_ref(Some("ɵɵpipeBind2"))
    }

    pub fn pipe_bind3() -> ExternalReference {
        Self::make_ref(Some("ɵɵpipeBind3"))
    }

    pub fn pipe_bind4() -> ExternalReference {
        Self::make_ref(Some("ɵɵpipeBind4"))
    }

    pub fn pipe_bind_v() -> ExternalReference {
        Self::make_ref(Some("ɵɵpipeBindV"))
    }

    /* Property instructions */
    pub fn dom_property() -> ExternalReference {
        Self::make_ref(Some("ɵɵdomProperty"))
    }

    pub fn aria_property() -> ExternalReference {
        Self::make_ref(Some("ɵɵariaProperty"))
    }

    pub fn property() -> ExternalReference {
        Self::make_ref(Some("ɵɵproperty"))
    }

    pub fn control() -> ExternalReference {
        Self::make_ref(Some("ɵɵcontrol"))
    }

    pub fn control_create() -> ExternalReference {
        Self::make_ref(Some("ɵɵcontrolCreate"))
    }

    /* Animation instructions */
    pub fn animation_enter_listener() -> ExternalReference {
        Self::make_ref(Some("ɵɵanimateEnterListener"))
    }

    pub fn animation_leave_listener() -> ExternalReference {
        Self::make_ref(Some("ɵɵanimateLeaveListener"))
    }

    pub fn animation_enter() -> ExternalReference {
        Self::make_ref(Some("ɵɵanimateEnter"))
    }

    pub fn animation_leave() -> ExternalReference {
        Self::make_ref(Some("ɵɵanimateLeave"))
    }

    /* i18n instructions */
    pub fn i18n() -> ExternalReference {
        Self::make_ref(Some("ɵɵi18n"))
    }

    pub fn i18n_attributes() -> ExternalReference {
        Self::make_ref(Some("ɵɵi18nAttributes"))
    }

    pub fn i18n_exp() -> ExternalReference {
        Self::make_ref(Some("ɵɵi18nExp"))
    }

    pub fn i18n_start() -> ExternalReference {
        Self::make_ref(Some("ɵɵi18nStart"))
    }

    pub fn i18n_end() -> ExternalReference {
        Self::make_ref(Some("ɵɵi18nEnd"))
    }

    pub fn i18n_apply() -> ExternalReference {
        Self::make_ref(Some("ɵɵi18nApply"))
    }

    pub fn i18n_postprocess() -> ExternalReference {
        Self::make_ref(Some("ɵɵi18nPostprocess"))
    }

    pub fn pipe() -> ExternalReference {
        Self::make_ref(Some("ɵɵpipe"))
    }

    pub fn projection() -> ExternalReference {
        Self::make_ref(Some("ɵɵprojection"))
    }

    pub fn projection_def() -> ExternalReference {
        Self::make_ref(Some("ɵɵprojectionDef"))
    }

    pub fn reference() -> ExternalReference {
        Self::make_ref(Some("ɵɵreference"))
    }

    pub fn inject() -> ExternalReference {
        Self::make_ref(Some("ɵɵinject"))
    }

    pub fn inject_attribute() -> ExternalReference {
        Self::make_ref(Some("ɵɵinjectAttribute"))
    }

    pub fn directive_inject() -> ExternalReference {
        Self::make_ref(Some("ɵɵdirectiveInject"))
    }

    pub fn invalid_factory() -> ExternalReference {
        Self::make_ref(Some("ɵɵinvalidFactory"))
    }

    pub fn invalid_factory_dep() -> ExternalReference {
        Self::make_ref(Some("ɵɵinvalidFactoryDep"))
    }

    pub fn template_ref_extractor() -> ExternalReference {
        Self::make_ref(Some("ɵɵtemplateRefExtractor"))
    }

    pub fn forward_ref() -> ExternalReference {
        Self::make_ref(Some("forwardRef"))
    }

    pub fn resolve_forward_ref() -> ExternalReference {
        Self::make_ref(Some("resolveForwardRef"))
    }

    pub fn replace_metadata() -> ExternalReference {
        Self::make_ref(Some("ɵɵreplaceMetadata"))
    }

    pub fn get_replace_metadata_url() -> ExternalReference {
        Self::make_ref(Some("ɵɵgetReplaceMetadataURL"))
    }

    pub fn define_injectable() -> ExternalReference {
        Self::make_ref(Some("ɵɵdefineInjectable"))
    }

    pub fn declare_injectable() -> ExternalReference {
        Self::make_ref(Some("ɵɵngDeclareInjectable"))
    }

    pub fn injectable_declaration() -> ExternalReference {
        Self::make_ref(Some("ɵɵInjectableDeclaration"))
    }

    pub fn resolve_window() -> ExternalReference {
        Self::make_ref(Some("ɵɵresolveWindow"))
    }

    pub fn resolve_document() -> ExternalReference {
        Self::make_ref(Some("ɵɵresolveDocument"))
    }

    pub fn resolve_body() -> ExternalReference {
        Self::make_ref(Some("ɵɵresolveBody"))
    }

    pub fn get_component_deps_factory() -> ExternalReference {
        Self::make_ref(Some("ɵɵgetComponentDepsFactory"))
    }

    /* Component/Directive definitions */
    pub fn define_component() -> ExternalReference {
        Self::make_ref(Some("ɵɵdefineComponent"))
    }

    pub fn declare_component() -> ExternalReference {
        Self::make_ref(Some("ɵɵngDeclareComponent"))
    }

    pub fn set_component_scope() -> ExternalReference {
        Self::make_ref(Some("ɵɵsetComponentScope"))
    }

    pub fn change_detection_strategy() -> ExternalReference {
        Self::make_ref(Some("ChangeDetectionStrategy"))
    }

    pub fn view_encapsulation() -> ExternalReference {
        Self::make_ref(Some("ViewEncapsulation"))
    }

    pub fn component_declaration() -> ExternalReference {
        Self::make_ref(Some("ɵɵComponentDeclaration"))
    }

    pub fn factory_declaration() -> ExternalReference {
        Self::make_ref(Some("ɵɵFactoryDeclaration"))
    }

    pub fn declare_factory() -> ExternalReference {
        Self::make_ref(Some("ɵɵngDeclareFactory"))
    }

    pub fn factory_target() -> ExternalReference {
        Self::make_ref(Some("ɵɵFactoryTarget"))
    }

    pub fn define_directive() -> ExternalReference {
        Self::make_ref(Some("ɵɵdefineDirective"))
    }

    pub fn declare_directive() -> ExternalReference {
        Self::make_ref(Some("ɵɵngDeclareDirective"))
    }

    pub fn directive_declaration() -> ExternalReference {
        Self::make_ref(Some("ɵɵDirectiveDeclaration"))
    }

    /* Injector definitions */
    pub fn injector_def() -> ExternalReference {
        Self::make_ref(Some("ɵɵInjectorDef"))
    }

    pub fn injector_declaration() -> ExternalReference {
        Self::make_ref(Some("ɵɵInjectorDeclaration"))
    }

    pub fn define_injector() -> ExternalReference {
        Self::make_ref(Some("ɵɵdefineInjector"))
    }

    pub fn declare_injector() -> ExternalReference {
        Self::make_ref(Some("ɵɵngDeclareInjector"))
    }

    /* NgModule definitions */
    pub fn ng_module_declaration() -> ExternalReference {
        Self::make_ref(Some("ɵɵNgModuleDeclaration"))
    }

    pub fn module_with_providers() -> ExternalReference {
        Self::make_ref(Some("ModuleWithProviders"))
    }

    pub fn define_ng_module() -> ExternalReference {
        Self::make_ref(Some("ɵɵdefineNgModule"))
    }

    pub fn declare_ng_module() -> ExternalReference {
        Self::make_ref(Some("ɵɵngDeclareNgModule"))
    }

    pub fn set_ng_module_scope() -> ExternalReference {
        Self::make_ref(Some("ɵɵsetNgModuleScope"))
    }

    pub fn register_ng_module_type() -> ExternalReference {
        Self::make_ref(Some("ɵɵregisterNgModuleType"))
    }

    /* Pipe definitions */
    pub fn pipe_declaration() -> ExternalReference {
        Self::make_ref(Some("ɵɵPipeDeclaration"))
    }

    pub fn define_pipe() -> ExternalReference {
        Self::make_ref(Some("ɵɵdefinePipe"))
    }

    pub fn declare_pipe() -> ExternalReference {
        Self::make_ref(Some("ɵɵngDeclarePipe"))
    }

    /* Class metadata */
    pub fn declare_class_metadata() -> ExternalReference {
        Self::make_ref(Some("ɵɵngDeclareClassMetadata"))
    }

    pub fn declare_class_metadata_async() -> ExternalReference {
        Self::make_ref(Some("ɵɵngDeclareClassMetadataAsync"))
    }

    pub fn set_class_metadata() -> ExternalReference {
        Self::make_ref(Some("ɵsetClassMetadata"))
    }

    pub fn set_class_metadata_async() -> ExternalReference {
        Self::make_ref(Some("ɵsetClassMetadataAsync"))
    }

    pub fn set_class_debug_info() -> ExternalReference {
        Self::make_ref(Some("ɵsetClassDebugInfo"))
    }

    /* Query instructions */
    pub fn query_refresh() -> ExternalReference {
        Self::make_ref(Some("ɵɵqueryRefresh"))
    }

    pub fn view_query() -> ExternalReference {
        Self::make_ref(Some("ɵɵviewQuery"))
    }

    pub fn load_query() -> ExternalReference {
        Self::make_ref(Some("ɵɵloadQuery"))
    }

    pub fn content_query() -> ExternalReference {
        Self::make_ref(Some("ɵɵcontentQuery"))
    }

    /* Signal queries */
    pub fn view_query_signal() -> ExternalReference {
        Self::make_ref(Some("ɵɵviewQuerySignal"))
    }

    pub fn content_query_signal() -> ExternalReference {
        Self::make_ref(Some("ɵɵcontentQuerySignal"))
    }

    pub fn query_advance() -> ExternalReference {
        Self::make_ref(Some("ɵɵqueryAdvance"))
    }

    /* Two-way bindings */
    pub fn two_way_property() -> ExternalReference {
        Self::make_ref(Some("ɵɵtwoWayProperty"))
    }

    pub fn two_way_binding_set() -> ExternalReference {
        Self::make_ref(Some("ɵɵtwoWayBindingSet"))
    }

    pub fn two_way_listener() -> ExternalReference {
        Self::make_ref(Some("ɵɵtwoWayListener"))
    }

    /* Let declarations */
    pub fn declare_let() -> ExternalReference {
        Self::make_ref(Some("ɵɵdeclareLet"))
    }

    pub fn store_let() -> ExternalReference {
        Self::make_ref(Some("ɵɵstoreLet"))
    }

    pub fn read_context_let() -> ExternalReference {
        Self::make_ref(Some("ɵɵreadContextLet"))
    }

    pub fn attach_source_locations() -> ExternalReference {
        Self::make_ref(Some("ɵɵattachSourceLocations"))
    }

    /* Features */
    pub fn ng_on_changes_feature() -> ExternalReference {
        Self::make_ref(Some("ɵɵNgOnChangesFeature"))
    }

    pub fn inherit_definition_feature() -> ExternalReference {
        Self::make_ref(Some("ɵɵInheritDefinitionFeature"))
    }

    pub fn providers_feature() -> ExternalReference {
        Self::make_ref(Some("ɵɵProvidersFeature"))
    }

    pub fn host_directives_feature() -> ExternalReference {
        Self::make_ref(Some("ɵɵHostDirectivesFeature"))
    }

    pub fn external_styles_feature() -> ExternalReference {
        Self::make_ref(Some("ɵɵExternalStylesFeature"))
    }

    pub fn listener() -> ExternalReference {
        Self::make_ref(Some("ɵɵlistener"))
    }

    pub fn get_inherited_factory() -> ExternalReference {
        Self::make_ref(Some("ɵɵgetInheritedFactory"))
    }

    /* Sanitization functions */
    pub fn sanitize_html() -> ExternalReference {
        Self::make_ref(Some("ɵɵsanitizeHtml"))
    }

    pub fn sanitize_style() -> ExternalReference {
        Self::make_ref(Some("ɵɵsanitizeStyle"))
    }

    pub fn validate_attribute() -> ExternalReference {
        Self::make_ref(Some("ɵɵvalidateAttribute"))
    }

    pub fn sanitize_resource_url() -> ExternalReference {
        Self::make_ref(Some("ɵɵsanitizeResourceUrl"))
    }

    pub fn sanitize_script() -> ExternalReference {
        Self::make_ref(Some("ɵɵsanitizeScript"))
    }

    pub fn sanitize_url() -> ExternalReference {
        Self::make_ref(Some("ɵɵsanitizeUrl"))
    }

    pub fn sanitize_url_or_resource_url() -> ExternalReference {
        Self::make_ref(Some("ɵɵsanitizeUrlOrResourceUrl"))
    }

    pub fn trust_constant_html() -> ExternalReference {
        Self::make_ref(Some("ɵɵtrustConstantHtml"))
    }

    pub fn trust_constant_resource_url() -> ExternalReference {
        Self::make_ref(Some("ɵɵtrustConstantResourceUrl"))
    }

    /* Decorators */
    pub fn input_decorator() -> ExternalReference {
        Self::make_ref(Some("Input"))
    }

    pub fn output_decorator() -> ExternalReference {
        Self::make_ref(Some("Output"))
    }

    pub fn view_child_decorator() -> ExternalReference {
        Self::make_ref(Some("ViewChild"))
    }

    pub fn view_children_decorator() -> ExternalReference {
        Self::make_ref(Some("ViewChildren"))
    }

    pub fn content_child_decorator() -> ExternalReference {
        Self::make_ref(Some("ContentChild"))
    }

    pub fn content_children_decorator() -> ExternalReference {
        Self::make_ref(Some("ContentChildren"))
    }

    /* Type-checking */
    pub fn input_signal_brand_write_type() -> ExternalReference {
        Self::make_ref(Some("ɵINPUT_SIGNAL_BRAND_WRITE_TYPE"))
    }

    pub fn unwrap_directive_signal_inputs() -> ExternalReference {
        Self::make_ref(Some("ɵUnwrapDirectiveSignalInputs"))
    }

    pub fn unwrap_writable_signal() -> ExternalReference {
        Self::make_ref(Some("ɵunwrapWritableSignal"))
    }

    pub fn assert_type() -> ExternalReference {
        Self::make_ref(Some("ɵassertType"))
    }
}

