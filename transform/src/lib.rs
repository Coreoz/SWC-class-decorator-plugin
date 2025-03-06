use serde::Deserialize;
use swc_core::common::DUMMY_SP;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::VisitMut;
use swc_core::ecma::visit::VisitMutWith;
use swc_ecma_utils::ExprFactory;

#[derive(Debug, Default, Clone, Deserialize)]
enum LogLevel {
    #[default]
    None,
    Info,
    Debug,
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Config {
    // Add any configuration fields here
    #[serde(default)]
    log: LogLevel,
}

pub struct TransformVisitor {
    pub config: Config,
}

impl VisitMut for TransformVisitor {
    // Implement necessary visit_mut_* methods for actual custom transform.
    // A comprehensive list of possible visitor methods can be found here:
    // https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html

    fn visit_mut_class_decl(&mut self, class_decl: &mut ClassDecl) {
        // Get the class name from ClassDecl
        let class_name = class_decl.ident.sym.to_string();
        self.process_class(
            &mut class_decl.class,
            Some(class_name),
            self.config.log.clone(),
        );
    }

    fn visit_mut_class_expr(&mut self, class_expr: &mut ClassExpr) {
        // Get the class name from ClassExpr if it has one
        let class_name = class_expr.ident.as_ref().map(|ident| ident.sym.to_string());
        self.process_class(&mut class_expr.class, class_name, self.config.log.clone());
    }
}

fn class_method(name: &str, body: Vec<Stmt>) -> ClassMethod {
    ClassMethod {
        span: Default::default(),
        key: PropName::Computed(ComputedPropName {
            span: DUMMY_SP,
            expr: Box::new(Expr::Call(CallExpr {
                span: DUMMY_SP,
                callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                    span: DUMMY_SP,
                    obj: Box::new(Expr::Ident(Ident {
                        sym: "Symbol".into(),
                        ..Default::default()
                    })),
                    prop: MemberProp::Ident(IdentName {
                        sym: "for".into(),
                        span: DUMMY_SP,
                    }),
                }))),
                args: vec![Lit::Str(Str {
                    span: DUMMY_SP,
                    value: name.into(),
                    raw: None,
                })
                .as_arg()],
                ..Default::default()
            })),
        }),
        function: Box::new(Function {
            params: vec![],
            body: Some(BlockStmt {
                stmts: body,
                ..Default::default()
            }),
            is_async: false,
            is_generator: false,
            ..Default::default()
        }),
        is_static: true,
        accessibility: None,
        is_abstract: false,
        is_optional: false,
        is_override: false,
        kind: MethodKind::Getter,
    }
}

fn extract_ident_type_ann(type_ann: &Option<Box<TsTypeAnn>>) -> Option<String> {
    if let Some(type_ann) = type_ann {
        if let TsType::TsTypeRef(type_ref) = &*type_ann.type_ann {
            if let TsEntityName::Ident(ident) = &type_ref.type_name {
                return Some(ident.sym.to_string());
            }
        }
    }
    None
}

impl TransformVisitor {
    fn process_class(&self, class: &mut Class, class_name: Option<String>, debug: LogLevel) {
        if class_name.is_none() {
            return;
        }

        let class_name = class_name.unwrap();

        // Find the constructor and its parameter types
        let mut ctor_args = vec![];
        for member in &class.body {
            if let ClassMember::Constructor(constructor) = member {
                for param in &constructor.params {
                    match param {
                        ParamOrTsParamProp::TsParamProp(ts_param_prop) => {
                            if let TsParamPropParam::Ident(ident) = &ts_param_prop.param {
                                if let Some(ident_type) = extract_ident_type_ann(&ident.type_ann) {
                                    ctor_args.push(ident_type);
                                }
                            }
                        }
                        ParamOrTsParamProp::Param(param) => match &param.pat {
                            Pat::Ident(ident) => {
                                if let Some(ident_type) = extract_ident_type_ann(&ident.type_ann) {
                                    ctor_args.push(ident_type);
                                }
                            }
                            Pat::Array(array_pat) => {
                                for elem in &array_pat.elems {
                                    if let Some(Pat::Ident(ident)) = elem {
                                        if let Some(ident_type) =
                                            extract_ident_type_ann(&ident.type_ann)
                                        {
                                            ctor_args.push(ident_type);
                                        }
                                    }
                                }
                            }
                            Pat::Object(object_pat) => {
                                for prop in &object_pat.props {
                                    match prop {
                                        ObjectPatProp::KeyValue(key_value) => {
                                            if let Pat::Ident(ident) = *key_value.value.clone() {
                                                if let Some(ident_type) =
                                                    extract_ident_type_ann(&ident.type_ann)
                                                {
                                                    ctor_args.push(ident_type);
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            _ => {}
                        },
                    }
                }
            }
        }

        match debug {
            LogLevel::Debug => {
                println!(
                    "\nswc-class-decorator-plugin - Processing class : {}, found constructor arguments : [{}] \n ------------------ \n {:?}\n",
                    class_name,
                    ctor_args.join(", "),
                    class
                );
            }
            LogLevel::Info => {
                println!(
                    "swc-class-decorator-plugin - Processing class : {}, found constructor arguments : [{}]",
                    class_name,
                    ctor_args.join(", ")
                );
            }
            _ => {}
        }

        // Create the new symbol methods
        let ctor_args_method = class_method(
            "___CTOR_ARGS___",
            vec![Stmt::Return(ReturnStmt {
                span: DUMMY_SP,
                arg: Some(Box::new(Expr::Array(ArrayLit {
                    span: DUMMY_SP,
                    elems: ctor_args
                        .into_iter()
                        .map(|arg| {
                            Some(ExprOrSpread {
                                spread: None,
                                expr: Box::new(Expr::Lit(Lit::Str(Str {
                                    span: DUMMY_SP,
                                    value: arg.into(),
                                    raw: None,
                                }))),
                            })
                        })
                        .collect(),
                }))),
            })],
        );

        let ctor_name_method = class_method(
            "___CTOR_NAME___",
            vec![Stmt::Return(ReturnStmt {
                span: DUMMY_SP,
                arg: Some(Box::new(Expr::Lit(Lit::Str(Str {
                    span: DUMMY_SP,
                    value: class_name.into(),
                    raw: None,
                })))),
            })],
        );

        class.body.push(ClassMember::Method(ctor_args_method));
        class.body.push(ClassMember::Method(ctor_name_method));
    }
}

#[cfg(test)]
mod tests {
    use crate::TransformVisitor;
    use swc_core::ecma::transforms::testing::test;
    use swc_core::ecma::visit::visit_mut_pass;
    use swc_ecma_parser::Syntax;

    test!(
        Syntax::Typescript(Default::default()),
        |_| visit_mut_pass(TransformVisitor {
            config: Default::default()
        }),
        boo,
        r#"class SampleApi {}

    /**
     * A sample Service that can be copied.
     * After it has been copied, this file should be deleted :)
     */
    export default class SampleService {
      constructor(private readonly sampleApi: SampleApi) {}
      sayHello(name: string) {
        return this.sampleApi.sample(name);
      }
    }
    "#
    );

    // An example to test plugin transform.
    // Recommended strategy to test plugin's transform is verify
    // the Visitor's behavior, instead of trying to run `process_transform` with mocks
    // unless explicitly required to do so.
    // test_inline!(
    //     Default::default(),
    //     |_| visit_mut_pass (TransformVisitor),
    //     boo,
    //     // Input codes
    //     r#"console.log("transform");"#,
    //     // Output codes after transformed with plugin
    //     r#"console.log("transform");"#
    // );
}
