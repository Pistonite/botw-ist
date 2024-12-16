//! Emit tokens from binding patterns in declaration and parameters

use intwc_semantic_tokens::token;
use swc_core::ecma::ast::{Ident, IdentName, ObjectPatProp, Pat, PropName, RestPat};

use crate::expr::ExprFlavor;
use crate::scope::ScopeRef;
use crate::util::IdentFlavor;
use crate::SemanticTokenizer;

impl<'a> SemanticTokenizer<'a> {
    /// Emit tokens from a binding pattern. Identifiers from the binding
    /// will be added to the scope
    pub fn emit_bind_pat(&mut self, 
        // The binding pattern to emit
        pat: &Pat, 
        // The scope where the identifiers will be added
        binding_scope: &ScopeRef, 
        // The scope where expressions from the pattern are evaluated
        eval_scope: &ScopeRef, 
        // The flavor of the binding
        bind_flavor: IdentFlavor) {
        match pat {
            Pat::Ident(ident) => {
                let flavor = if let Some(typ) = &ident.type_ann {
                    self.emit_type(&typ.type_ann);
                    IdentFlavor::from_type(&typ.type_ann, bind_flavor)
                } else {
                        bind_flavor
                    };
                binding_scope.borrow_mut().add_declaration(&ident.id, Some(flavor));
            }
            Pat::Array(array) => {
                if let Some(typ) = &array.type_ann {
                    self.emit_type(&typ.type_ann);
                }
                // not doing type analysis here for nested types
                for pat in &array.elems {
                    if let Some(pat) = pat {
                        self.emit_bind_pat(pat, binding_scope, eval_scope, bind_flavor);
                    }
                }
            }
            Pat::Rest(rest) => {
                self.emit_bind_pat_rest(rest, binding_scope, eval_scope, bind_flavor);
            }
            Pat::Object(obj) => {
                if let Some(typ) = &obj.type_ann {
                    self.emit_type(&typ.type_ann);
                }
                // not doing type analysis here for nested types
                for prop in &obj.props {
                    match prop {
                        // { foo: bar }
                        ObjectPatProp::KeyValue(kv) => {
                            if let Some(name) = self.emit_prop_name(&kv.key, eval_scope) {
                                self.add_span(token!(variable), name.span);
                            }
                            self.emit_bind_pat(&kv.value, binding_scope, eval_scope, bind_flavor);
                        }
                        // { foo } or { foo = <expr> }
                        ObjectPatProp::Assign(assign) => {
                            match &assign.value {
                                Some(value) => {
                                    self.emit_keyed_expr(assign.key.span, value, eval_scope, bind_flavor);
                                }
                                None => {
                                    binding_scope.borrow_mut().add_declaration(&assign.key, Some(bind_flavor));
                                }
                            }
                        }
                        // { ...foo }
                        ObjectPatProp::Rest(rest) => {
                            self.emit_bind_pat_rest(rest, binding_scope, eval_scope, bind_flavor);
                        }
                    }
                }
            }
            // foo = <expr>, in function parameters
            Pat::Assign(assign) => {
                let flavor = match self.emit_expr(&assign.right, eval_scope, bind_flavor) {
                    // keep the binding flavor if the right side is an identifier
                    ExprFlavor::Ident(_) => bind_flavor,
                    // otherwise, use the stronger of right side flavor
                    ExprFlavor::Flavor(flavor) => flavor.max(bind_flavor),
                };
                self.emit_bind_pat(&assign.left, binding_scope, eval_scope, flavor);
            }
            Pat::Invalid(_) => {
                // skip
            }
            Pat::Expr(expr) => {
                let _ = self.emit_expr(&expr, eval_scope, bind_flavor);
            }
        }

    }

    /// Helper to bind a ...rest pattern
    fn emit_bind_pat_rest(&mut self, pat: &RestPat, binding_scope: &ScopeRef, eval_scope: &ScopeRef, bind_flavor: IdentFlavor) {
        if let Some(typ) = &pat.type_ann {
            self.emit_type(&typ.type_ann);
        }
        // not doing complex type analysis here
        self.emit_bind_pat(&pat.arg, binding_scope, eval_scope, bind_flavor);
    }

    /// Emit token from PropName
    ///
    /// If the prop name is an ident, the ident will be returned for further processing
    /// If the prop name is a computed expression, the expression will be evaluated
    /// in the input scope
    pub fn emit_prop_name<'x>(&mut self, name: &'x PropName, eval_scope: &ScopeRef) -> Option<&'x IdentName> {
        match name {
            // { foo: ... }
            PropName::Ident(ident) => {
                return Some(ident);
            }
            PropName::Str(_)
            |
            PropName::Num(_)
            |
            PropName::BigInt(_) => {
                // literals are already tokens, no need to emit semantic tokens
            }
            PropName::Computed(prop) => {
                // does not take input hint because the key is computed
                // from current scope, not related to the current binding or the expression
                self.emit_expr(&prop.expr, eval_scope, IdentFlavor::WeakVariable);
            }
        }

        None
    }
}

