//! Emit tokens from module level items like interfaces, classes, functions, etc.

use std::{cell::RefCell, rc::Rc};

use intwc_semantic_tokens::{token, TokenBuilderByPos};
use swc_core::ecma::ast::{ImportDecl, ImportSpecifier, ModuleDecl, ModuleExportName};

use crate::{expr, scope::Scope, util::{self, IdentFlavor}, SemanticTokenizer};

impl<'a> SemanticTokenizer<'a> {
    pub fn emit_module_decl(&mut self, decl: &ModuleDecl, scope: &Rc<RefCell<Scope>>) {
    match decl {
        ModuleDecl::Import(decl) => {
            self.emit_import_decl(decl, scope);
        }
        ModuleDecl::ExportDecl(decl) => {
            todo!()
        }
        ModuleDecl::ExportDefaultDecl(export_default_decl) => {
            todo!()
        }
        ModuleDecl::ExportDefaultExpr(export_default_expr) => {
            todo!()
        }
        ModuleDecl::ExportNamed(named_export) => {
            todo!()
        }
        ModuleDecl::ExportAll(export_all) => {
            todo!()
        }
        ModuleDecl::TsImportEquals(ts_import_equals) => {
            todo!()
        }
        ModuleDecl::TsExportAssignment(ts_export_assignment) => {
            todo!()
        }
        ModuleDecl::TsNamespaceExport(ts_namespace_export) => {
            todo!()
        }
    }
    }

    /// Emit tokens from an import declaration like import { foo } from 'bar'
    pub fn emit_import_decl(&mut self, decl: &ImportDecl, scope: &Rc<RefCell<Scope>>) {
        // keywords (import, type, as, from, with) are already tokens, so we don't need to emit them
        // strings are also already tokens
        //
        // we need:
        // - * -> operator
        // - add the identifiers in the import specifiers
        //   - mark them as type if the import is type_only
        // - parse the object in import with
        let outer_is_type = decl.type_only;
        for item in &decl.specifiers {
            match item {
                // import { foo } ...
                ImportSpecifier::Named(named) => {
                    let is_type = outer_is_type || named.is_type_only;
                    let flavor = if is_type {
                        IdentFlavor::Type
                    } else {
                        IdentFlavor::guess(&named.local)
                    };
                    scope.borrow_mut().add_declaration(&named.local, Some(flavor));
                    if let Some(ModuleExportName::Ident(ident)) = &named.imported {
                        let flavor = if is_type {
                            IdentFlavor::Type
                        } else {
                            IdentFlavor::guess(ident)
                        };
                        scope.borrow_mut().add_declaration(ident, Some(flavor));
                    }
                }
                // import foo ...
                ImportSpecifier::Default(default) => {
                    let flavor = if outer_is_type {
                        IdentFlavor::Type
                    } else {
                        IdentFlavor::guess(&default.local)
                    };
                    scope.borrow_mut().add_declaration(&default.local, Some(flavor));
                }
                // import * as foo ...
                ImportSpecifier::Namespace(namespace) => {
                    // SWC doesn't give span for the *, so we need to find it
                    self.find_and_add(token!(operator), namespace.span, "*");

                    // spread import cannot be type
                    let flavor = IdentFlavor::guess(&namespace.local);
                    scope.borrow_mut().add_declaration(&namespace.local, Some(flavor));
                }
            }
        }

        if let Some(with_spec) = &decl.with {
            self.emit_obj_literal(with_spec, scope);
        }
    }
}

