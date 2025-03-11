use serde_json::from_str;
use swc_class_decorator::{Config, TransformVisitor};
use swc_core::ecma::ast::Program;
use swc_core::ecma::visit::VisitMutWith;
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

/// An example plugin function with macro support.
/// `plugin_transform` macro interop pointers into deserialized structs, as well
/// as returning ptr back to host.
///
/// It is possible to opt out from macro by writing transform fn manually
/// if plugin need to handle low-level ptr directly via
/// `__transform_plugin_process_impl(
///     ast_ptr: *const u8, ast_ptr_len: i32,
///     unresolved_mark: u32, should_enable_comments_proxy: i32) ->
///     i32 /*  0 for success, fail otherwise.
///             Note this is only for internal pointer interop result,
///             not actual transform result */`
///
/// This requires manual handling of serialization / deserialization from ptrs.
/// Refer swc_plugin_macro to see how does it work internally.

/// Plugin entry point for swc_class_decorator_plugin
#[plugin_transform]
pub fn process_transform(mut program: Program, data: TransformPluginProgramMetadata) -> Program {
    // Get and parse the configuration
    let config = from_str::<Config>(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for swc-class-decorator-plugin"),
    )
    .expect("invalid config for swc-class-decorator-plugin, please check your configuration");

    // Redirect to transform/src/lib.rs and apply the transformation
    program.visit_mut_with(&mut TransformVisitor { config });
    program
}
