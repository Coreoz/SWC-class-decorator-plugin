use swc_core::ecma::transforms::testing::test;
use swc_core::common::{DUMMY_SP, Spanned};
use swc_core::ecma::{
    ast::Program,
    transforms::testing::test_inline,
    visit::{as_folder, FoldWith, VisitMut},
};
use swc_core::ecma::ast::*;
use swc_ecma_parser::{Syntax, TsConfig};
use swc_core::ecma::visit::{Fold, VisitMutWith};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

pub struct AddSymbols;

fn class_method(name: &str, body: Vec<Option<ExprOrSpread>>) -> ClassProp {
    ClassProp {
        key: PropName::Computed(ComputedPropName {
            span: DUMMY_SP,
            expr: Box::new(Expr::Call(CallExpr {
                span: DUMMY_SP,
                callee: Callee::Expr(Box::new(Expr::Member(MemberExpr {
                    span: DUMMY_SP,
                    obj: Box::new(Expr::Ident(Ident::new("Symbol".into(), DUMMY_SP))),
                    prop: MemberProp::Ident(Ident::new("for".into(), DUMMY_SP)),
                }))),
                args: vec![ExprOrSpread {
                    spread: None,
                    expr: Box::new(Expr::Lit(Lit::Str(Str {
                        span: DUMMY_SP,
                        value: name.into(),
                        raw: None,
                    }))),
                }],
                type_args: None,
            })),
        }),
        value: Some(Box::new(Expr::Array(ArrayLit {
            span: DUMMY_SP,
            elems: body,
        }))),
        type_ann: None,
        is_static: true,
        decorators: vec![],
        span: DUMMY_SP,
        accessibility: None,
        is_abstract: false,
        is_optional: false,
        is_override: false,
        readonly: false,
        declare: false,
        definite: false,
    }
}

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    // Implement necessary visit_mut_* methods for actual custom transform.
    // A comprehensive list of possible visitor methods can be found here:
    // https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html

    fn visit_mut_class_decl(&mut self, class_decl: &mut ClassDecl) {
        // Get the class name from ClassDecl
        let class_name = class_decl.ident.sym.to_string();
        self.process_class(&mut class_decl.class, Some(class_name));
    }

    fn visit_mut_class_expr(&mut self, class_expr: &mut ClassExpr) {
        // Get the class name from ClassExpr if it has one
        let class_name = class_expr.ident.as_ref().map(|ident| ident.sym.to_string());
        self.process_class(&mut class_expr.class, class_name);
    }
}

impl TransformVisitor {
    fn process_class(&self, class: &mut Class, class_name: Option<String>) {
        if class_name.is_none() {
            return;
        }

        let class_name = class_name.unwrap();

        // Find the constructor and its parameter types
        let mut ctor_args = vec![];
        for member in &class.body {
            if let ClassMember::Constructor(constructor) = member {
                for param in &constructor.params {
                    if let ParamOrTsParamProp::Param(param) = param {
                        if let Pat::Ident(ident) = &param.pat {
                            if let Some(type_ann) = &ident.type_ann {
                                if let TsType::TsTypeRef(type_ref) = &*type_ann.type_ann {
                                    if let TsEntityName::Ident(ident) = &type_ref.type_name {
                                        ctor_args.push(ident.sym.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Create the new symbol properties
        let ctor_args_prop = class_method(
            "___CTOR_ARGS___",
            vec![
                Some(ExprOrSpread {
                    spread: None,
                    expr: Box::new(Expr::Array(ArrayLit {
                        span: DUMMY_SP,
                        elems: ctor_args
                            .into_iter()
                            .map(|arg| Some(ExprOrSpread {
                                spread: None,
                                expr: Box::new(Expr::Lit(Lit::Str(Str {
                                    span: DUMMY_SP,
                                    value: arg.into(),
                                    raw: None,
                                }))),
                            }))
                            .collect(),
                    })),
                }),
            ],
        );

        let ctor_name_prop = class_method(
            "___CTOR_NAME___",
            vec![
                Some(ExprOrSpread {
                    spread: None,
                    expr: Box::new(Expr::Lit(Lit::Str(Str {
                        span: DUMMY_SP,
                        value: class_name.into(),
                        raw: None,
                    }))),
                }),
            ],
        );

        // Add the new properties to the class
        class.body.push(ClassMember::ClassProp(ctor_args_prop));
        class.body.push(ClassMember::ClassProp(ctor_name_prop));
    }
}

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
#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor))
}

test!(
    Syntax::Typescript(Default::default()),
    |_| as_folder(TransformVisitor),
    boo,
    r#"import CollaboratorsApi from '@api/collaborator/CollaboratorsApi';
import SsrBrowserObservableManager
  from '@services/ssr/SsrBrowserObservableManager';
import { Observable } from 'micro-observables';
import { SsrObservableContent, SsrWritableObservable } from 'plume-ssr-browser';
import { HttpPromise } from 'simple-http-rest-client';
import { CollaboratorPhoto } from '../../types/collaborator/collaborators.type';
import { SsrObservableKey } from '../../types/ssr/SsrTypes';

class CollaboratorService {

  private photos: SsrWritableObservable<CollaboratorPhoto[], SsrObservableKey>;

  constructor(
    protected readonly collaboratorsApi: CollaboratorsApi,
    private readonly observableManager: SsrBrowserObservableManager,
  ) {
    this.photos = observableManager.observable('collaborator-photos');
  }

  fetchCollaboratorsPhoto(): HttpPromise<void> {
    return this.collaboratorsApi
      .fetchCollaboratorsPhoto()
      .then((collaboratorPhotos: CollaboratorPhoto[]) => {
        this.photos.set({
          data: collaboratorPhotos,
          config: {},
        });
      })
      .catch(() => {
        this.photos.set({
          data: [],
          config: {},
        });
      });
  }

  getPhotos(): Observable<SsrObservableContent<CollaboratorPhoto[], SsrObservableKey> | undefined> {
    return this.photos.readOnly();
  }
}

export default CollaboratorService;
"#
);

// An example to test plugin transform.
// Recommended strategy to test plugin's transform is verify
// the Visitor's behavior, instead of trying to run `process_transform` with mocks
// unless explicitly required to do so.
// test_inline!(
//     Default::default(),
//     |_| as_folder(TransformVisitor),
//     boo,
//     // Input codes
//     r#"console.log("transform");"#,
//     // Output codes after transformed with plugin
//     r#"console.log("transform");"#
// );
