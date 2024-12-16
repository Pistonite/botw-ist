//! Emit tokens from expressions

use std::{cell::RefCell, rc::Rc};

use intwc_semantic_tokens::{token, TokenBuilderByPos};
use swc_common::{BytePos, Span, Spanned};
use swc_core::ecma::ast::{ArrayLit, ArrowExpr, AssignOp, AssignProp, AssignTarget, BigInt, BinaryOp, BlockStmtOrExpr, Callee, Decorator, Expr, ExprOrSpread, Function, Ident, IdentName, Lit, MemberExpr, MetaPropKind, Number, ObjectLit, OptChainExpr, ParenExpr, Prop, PropOrSpread, SimpleAssignTarget, SpreadElement, Str, Super, SuperPropExpr, Tpl, UnaryOp, UpdateOp};

use crate::{scope::{Scope, ScopeEntry, ScopeRef}, util::{match_operator, IdentFlavor, IdentOrIdentNameRef}, SemanticTokenizer};

impl<'a> SemanticTokenizer<'a> {
    /// Emit tokens from an object literal like { foo: 1, bar: 2, ...biz },
    /// evaluated in the input scope
    pub fn emit_obj_literal(&mut self, obj_lit: &ObjectLit, scope: &Rc<RefCell<Scope>>) {
        for prop in &obj_lit.props {
            match prop {
                PropOrSpread::Spread(spread) => {
                    self.emit_spread(spread, scope);
                }
                PropOrSpread::Prop(prop) => {
                    self.emit_property(prop, scope);
                }
            }
        }
    }

    pub fn emit_array_literal(&mut self, array_lit: &ArrayLit, scope: &Rc<RefCell<Scope>>) {
        for elem in &array_lit.elems {
            if let Some(e) = elem {
                self.emit_spread_arg(e, scope);
            }
        }
    }

    pub fn emit_spread_arg(&mut self, e: &ExprOrSpread, scope: &ScopeRef) {
        // make the 3 dots an operator
        if let Some(spread) = &e.spread {
            self.add_span(token!(operator), spread.span());
        }
        let _ = self.emit_expr(&e.expr, scope, IdentFlavor::WeakVariable);
    }

    /// Emit tokens from a spread expression (...foo)
    pub fn emit_spread(&mut self, expr: &SpreadElement, scope: &Rc<RefCell<Scope>>) {
        // treat the 3 dots as an operator
        self.add_span(token!(operator), expr.dot3_token);
        // there's no key, ignore the flavor
        let _ = self.emit_expr(&expr.expr, scope, IdentFlavor::WeakVariable);
    }

    /// Emit tokens from an object property, evaluated in the input scope
    pub fn emit_property(&mut self, prop: &Prop, scope: &Rc<RefCell<Scope>>) {
        match prop {
            Prop::Shorthand(ident) => {
                scope.borrow_mut().add_reference(ident, None);
            }
            Prop::KeyValue(prop) => {
                if let Some(ident) = self.emit_prop_name(&prop.key, scope) {
                    self.emit_keyed_expr(ident.span, &prop.value, scope, IdentFlavor::guess2(ident));
                } else {
                    // key is not ident, ignore the flavor
                    let _ = self.emit_expr(&prop.value, scope, IdentFlavor::WeakVariable);
                }
            }
            Prop::Assign(prop) => {
                self.emit_keyed_expr(prop.key.span(), &prop.value, scope, IdentFlavor::guess(&prop.key));
                // there is probably an equal sign?
                self.find_and_add(token!(operator), prop.span, "=");
            }
            Prop::Getter(prop) => {
                self.add_span(token!(variable), prop.key.span());
                if let Some(typ) = &prop.type_ann {
                    self.emit_type(&typ.type_ann);
                }
                if let Some(body) = &prop.body {
                    self.emit_block_stmt_inner_scope(body, scope);
                }
            }
            Prop::Setter(prop) => {
                self.add_span(token!(variable), prop.key.span());
                // bind the setter parameter
                let child_scope = self.scopes.make_child(scope);
                self.emit_bind_pat(&prop.param, &child_scope, scope, IdentFlavor::Variable);
                if let Some(body) = &prop.body {
                    self.emit_block_stmt(body, &child_scope);
                }
            }
            Prop::Method(prop) => {
                // method is definitely a function :)
                self.add_span(token!(function), prop.key.span());
            }
        }
    }

    /// Emit tokens from any kind of key-value pair
    ///
    /// The key_flavor is used to hint the expr flavor. The output expr
    /// flavor will be assigned to the key, which can have 3 cases:
    /// 1. The expr is an identifier, in which case the key will be connected
    ///    to that identifier.
    /// 2. The expr has a stronger flavor that overrides the key.
    /// 3. The expr has a weaker flavor, in which case the key will have the input flavor
    pub fn emit_keyed_expr(&mut self, 
        key_span: Span, value: &Expr, scope: &Rc<RefCell<Scope>>, key_flavor: IdentFlavor) {
        match self.emit_expr(value, scope, key_flavor) {
            ExprFlavor::Ident(entry) => {
                // add the key to be resolved together with the value ident
                entry.borrow_mut().add_reference(key_span, None);
            }
            ExprFlavor::Flavor(flavor) => {
                // resolve the key as the flavor
                self.add_span(flavor.to_token(), key_span);
            }
        }
    }

    /// Emit tokens for an expression. 
    ///
    /// 2 flavor hints are needed, one for hinting the type of the expression,
    /// and the other for hinting the type of the identifier that the expression
    /// expected to be assigned to.
    ///
    /// If the flavor of this expression to be assigned to an identifier
    /// is tied to another identifier, return Ident, otherwise return a flavor
    /// of this expression
    #[must_use]
    pub fn emit_expr(&mut self, expr: &Expr, scope: &Rc<RefCell<Scope>>, eval_hint: IdentFlavor, assign_hint: IdentFlavor) -> ExprFlavor {
        match expr {
            Expr::This(expr) => {
                self.add_span(token!(variable.defaultLibrary), expr.span);
                flavor_hint.max(IdentFlavor::Variable).into()
            }
            Expr::Array(expr) => {
                self.emit_array_literal(expr, scope);
                flavor_hint.max(IdentFlavor::Variable).into()
            }
            Expr::Object(expr) => {
                self.emit_obj_literal(expr, scope);
                flavor_hint.max(IdentFlavor::Variable).into()
            }
            Expr::Fn(expr) => {
                // function expressions don't bind the name of the function
                // to the scope
                if let Some(ident) = &expr.ident {
                    self.add_span(token!(function), ident.span());
                }
                self.emit_function(&expr.function, scope);
                flavor_hint.max(IdentFlavor::Function).into()
            }
            Expr::Unary(expr) => {
                match_operator!(self, expr.op, UnaryOp, expr.span, {
                    Plus => "+",
                    Minus => "-",
                    Bang => "!",
                    Tilde => "~",
                    TypeOf => "typeof",
                    Void => "void",
                    Delete => "delete"
                });
                self.emit_expr(&expr.arg, scope, flavor_hint);
                flavor_hint.max(IdentFlavor::Variable).into()
            }
            Expr::Update(expr) => {
                let span = if expr.prefix {
                    expr.span
                } else {
                    let hi = expr.span.hi;
                    let lo = BytePos(hi.0 - 2);
                    Span::new(lo, hi)
                };
                match_operator!(self, expr.op, UpdateOp, span, {
                    PlusPlus => "++",
                    MinusMinus => "--"
                });
                self.emit_expr(&expr.arg, scope, flavor_hint);
                flavor_hint.max(IdentFlavor::Variable).into()
            }
            Expr::Bin(expr) => {
                let op_span = Span::new(expr.left.span().hi, expr.right.span().lo);
                let _ = self.emit_expr(&expr.left, scope, flavor_hint);
                match_operator!(self, expr.op, BinaryOp, op_span, {
                    EqEq => "==",
                    NotEq => "!=",
                    EqEqEq => "===",
                    NotEqEq => "!==",
                    Lt => "<",
                    LtEq => "<=",
                    Gt => ">",
                    GtEq => ">=",
                    LShift => "<<",
                    RShift => ">>",
                    ZeroFillRShift => ">>>",
                    Add => "+",
                    Sub => "-",
                    Mul => "*",
                    Div => "/",
                    Mod => "%",
                    BitOr => "|",
                    BitXor => "^",
                    BitAnd => "&",
                    LogicalOr => "||",
                    LogicalAnd => "&&",
                    In => "in",
                    InstanceOf => "instanceof",
                    Exp => "**",
                    NullishCoalescing => "??"
                });
                self.emit_expr(&expr.right, scope, flavor_hint);
                flavor_hint.max(IdentFlavor::Variable).into()
            }
            Expr::Assign(expr) => {
                // emit tokens from the target, and get identifiers to analyze the type
                let left_expr_flavor  = match &expr.left {
                    AssignTarget::Simple(target) => {
                        match target {
                            SimpleAssignTarget::Ident(ident) => {
                                let ident = scope.borrow().add_reference(ident, None);
                                ExprFlavor::Ident(ident)
                            },
                            SimpleAssignTarget::Member(expr) => {
                                self.emit_member_expr(expr, scope, flavor_hint)
                            },
                            SimpleAssignTarget::SuperProp(expr) => {
                                self.emit_super_expr(expr, scope, flavor_hint)
                            }
                            SimpleAssignTarget::Paren(expr) => {
                                self.emit_paren_expr(expr, scope, flavor_hint)
                            }
                            SimpleAssignTarget::OptChain(expr) => {
                                // cannot assign to opt chain, so it doesn't matter
                                self.emit_opt_chain_expr(expr, scope);
                                ExprFlavor::Flavor(IdentFlavor::Variable)
                            }
                            SimpleAssignTarget::TsAs(expr) => {
                                let flavor = IdentFlavor::from_type(&expr.type_ann, flavor_hint);
                                self.emit_expr(&expr.expr, scope, flavor)
                            }
                            _ => todo!()

                        }
                    }
                    _ => todo!()
                };

                // the assignment operator
                let op_span = Span::new(expr.left.span().hi, expr.right.span().lo);
                match_operator!(self, expr.op, AssignOp, op_span, {
                    Assign => "=",
                    AddAssign => "+=",
                    SubAssign => "-=",
                    MulAssign => "*=",
                    DivAssign => "/=",
                    ModAssign => "%=",
                    LShiftAssign => "<<=",
                    RShiftAssign => ">>=",
                    ZeroFillRShiftAssign => ">>>=",
                    BitOrAssign => "|=",
                    BitXorAssign => "^=",
                    BitAndAssign => "&=",
                    ExpAssign => "**=",
                    AndAssign => "&&=",
                    OrAssign => "||=",
                    NullishAssign => "??="
                });

                let right_expr_flavor = self.emit_expr(&expr.right, scope, flavor_hint);
                // consolidate the flavor
                match (left_expr_flavor, right_expr_flavor) {
                    (ExprFlavor::Ident(left), ExprFlavor::Ident(right)) => {
                        // whatever type right gets, left will also get
                        // however, they are not the same identifier
                        right.borrow_mut().add_link(Rc::clone(&left));
                        ExprFlavor::Ident(Rc::clone(&left))
                    }
                    (ExprFlavor::Ident(left), ExprFlavor::Flavor(right)) => {
                        left.borrow_mut().set_flavor(right);
                        ExprFlavor::Ident(Rc::clone(&left))
                    }
                    (ExprFlavor::Flavor(left), ExprFlavor::Ident(right)) => {
                        right.borrow_mut().set_flavor(left);
                        ExprFlavor::Ident(Rc::clone(&right))
                    }
                    (ExprFlavor::Flavor(left), ExprFlavor::Flavor(right)) => {
                        let flavor = left.max(right);
                        flavor.max(flavor_hint).into()
                    }
                }
            }
            // member access
            Expr::Member(expr) => {
                self.emit_member_expr(expr, scope, flavor_hint)
            }
            // super.prop
            Expr::SuperProp(expr) => {
                self.emit_super_expr(expr, scope, flavor_hint)
            }
            // turnary
            Expr::Cond(expr) => {
                let question_span = Span::new(expr.test.span().hi, expr.cons.span().lo);
                let colon_span = Span::new(expr.cons.span().hi, expr.alt.span().lo);
                self.emit_expr(&expr.test, scope, IdentFlavor::WeakVariable);
                self.find_and_add(token!(operator), question_span, "?");
                let left = self.emit_expr(&expr.cons, scope, flavor_hint);
                self.find_and_add(token!(operator), colon_span, ":");
                let right = self.emit_expr(&expr.alt, scope, flavor_hint);
                // consolidate
                match (left, right) {
                    (ExprFlavor::Ident(left), ExprFlavor::Ident(right)) => {
                        // we will just pick left - but they don't have to be the same
                        // so we don't add link
                        ExprFlavor::Ident(left)
                    }
                    (ExprFlavor::Ident(left), ExprFlavor::Flavor(right)) => {
                        right.max(flavor_hint).into()
                    }
                    (ExprFlavor::Flavor(left), ExprFlavor::Ident(right)) => {
                        left.max(flavor_hint).into()
                    }
                    (ExprFlavor::Flavor(left), ExprFlavor::Flavor(right)) => {
                        let flavor = left.max(right);
                        flavor.max(flavor_hint).into()
                    }
                }
            }
            Expr::Call(expr) => {
                match &expr.callee {
                    Callee::Super(x) => {
                        self.emit_super(x);
                    }
                    Callee::Import(_) => {
                        // import is already a keyword
                    }
                    Callee::Expr(expr) => {
                        // expression should be callable
                        self.emit_expr(expr, scope, IdentFlavor::Function);
                    }
                }
                if let Some(type_params) = &expr.type_args {
                    self.emit_type_param_inst(type_params);
                }
                for arg in &expr.args {
                    self.emit_spread_arg(arg, scope);
                }

                flavor_hint.max(IdentFlavor::Variable).into()
            },
            Expr::New(expr) => {
                // you probably want to new a type right?
                let _ = self.emit_expr(&expr.callee, scope, IdentFlavor::Type);
                if let Some(type_params) = &expr.type_args {
                    self.emit_type_param_inst(type_params);
                }
                if let Some(args) = &expr.args {
                    for arg in args {
                        self.emit_spread_arg(arg, scope);
                    }
                }

                flavor_hint.max(IdentFlavor::Variable).into()
            },
            // (a, b, c)
            Expr::Seq(expr) => {
                for (i, e) in expr.exprs.iter().enumerate() {
                    let last = i == expr.exprs.len() - 1;
                    if last {
                        break;
                    } else {
                        self.emit_expr(e, scope, IdentFlavor::WeakVariable);
                    }
                }
                // the last expression is the flavor
                if let Some(last) = expr.exprs.last() {
                    self.emit_expr(last, scope, flavor_hint)
                } else {
                    flavor_hint.max(IdentFlavor::Variable).into()
                }
            }
            Expr::Ident(ident) => {
                let entry = scope.borrow().add_reference(ident, None);
                ExprFlavor::Ident(entry)
            }
            Expr::Lit(expr) => {
                // most are already tokens, some are keywords, change them to constants instead
                match expr {
                    Lit::Bool(expr) => {
                        self.add_span(token!(constant.boolean), expr.span);
                    },
                    Lit::Null(expr) => {
                        self.add_span(token!(constant.undefined), expr.span);
                    },
                    Lit::JSXText(expr) => {
                        self.add_span(token!(source), expr.span);
                    },
                    _ => {
                        // other are already tokens
                    }
                }

                flavor_hint.max(IdentFlavor::Variable).into()
            }
            // templated
            Expr::Tpl(expr) => {
                self.emit_tpl_expr(expr, scope);
                flavor_hint.max(IdentFlavor::Variable).into()
            }
            Expr::TaggedTpl(expr) => {
                let _ = self.emit_expr(&expr.tag, scope, IdentFlavor::Function);
                if let Some(type_params) = &expr.type_params {
                    self.emit_type_param_inst(type_params);
                }
                self.emit_tpl_expr(&expr.tpl, scope);
                flavor_hint.max(IdentFlavor::Variable).into()
            }
            Expr::Arrow(expr) => {
                self.emit_function_arrow(expr, scope);
                flavor_hint.max(IdentFlavor::Function).into()
            }
            Expr::Class(expr) => {
                todo!()
                }
            Expr::Yield(expr) => {
                if let Some(arg) = &expr.arg {
                    self.emit_expr(arg, scope, flavor_hint)
                } else {
                    flavor_hint.max(IdentFlavor::Variable).into()
                }
            }
            Expr::MetaProp(expr) => {
                // we will treat new/import as keyword, and the prop as builtin
                match expr.kind {
                    MetaPropKind::NewTarget => {
                        self.find_and_add(token!(variable.defaultLibrary), expr.span, "target");
                    }
                    MetaPropKind::ImportMeta => {
                        self.find_and_add(token!(variable.defaultLibrary), expr.span, "meta");
                    }
                }

                flavor_hint.max(IdentFlavor::Variable).into()
            }
            Expr::Await(expr) => {
                self.emit_expr(&expr.arg, scope, flavor_hint);
                flavor_hint.max(IdentFlavor::Variable).into()
            }
            Expr::Paren(expr) => {
                self.emit_paren_expr(expr, scope, flavor_hint)
            }

        }
    }

    fn emit_member_expr<'x>(&mut self, expr: &'x MemberExpr, scope: &ScopeRef, flavor_hint: IdentFlavor) -> ExprFlavor {
        todo!()
    }

    fn emit_super_expr<'x>(&mut self, expr: &'x SuperPropExpr, scope: &ScopeRef, flavor_hint: IdentFlavor) -> ExprFlavor {
        todo!()
    }

    #[inline]
    fn emit_super(&mut self, s: &Super) {
        self.add_span(token!(variable.defaultLibrary), s.span);
    }

    #[inline]
    fn emit_paren_expr(&mut self, expr: &ParenExpr, scope: &ScopeRef, flavor_hint: IdentFlavor) -> ExprFlavor {
        self.emit_expr(&expr.expr, scope, flavor_hint)
    }

    fn emit_opt_chain_expr(&mut self, expr: &OptChainExpr, scope: &ScopeRef) {
        todo!()
    }

    fn emit_tpl_expr(&mut self, expr: &Tpl, scope: &ScopeRef) {
        // the template string is already a token, we just need to parse
        // the expressions inside
        for expr in &expr.exprs {
            let _ = self.emit_expr(expr, scope, IdentFlavor::WeakVariable);
        }
    }

    /// Emit a function expression
    ///
    /// The input scope is the scope where the function is defined,
    /// not the inner scope of the function body
    pub fn emit_function(&mut self, func: &Function, scope: &Rc<RefCell<Scope>>) {
        for decor in &func.decorators {
            self.emit_decorator(decor, scope);
        }
        if let Some(type_params) = &func.type_params {
            self.emit_type_param(type_params);
        }
        let child_scope = self.scopes.make_child(scope);
        for param in &func.params {
            for decor in &param.decorators {
                self.emit_decorator(decor, scope);
            }
            // parameters are bound to the inner scope
            self.emit_bind_pat(&param.pat, &child_scope, &scope, IdentFlavor::Variable);
        }

        if let Some(rettype) = &func.return_type {
            self.emit_type(&rettype.type_ann);
        }

        if let Some(body) = &func.body {
            self.emit_block_stmt(body, &child_scope);
        }
    }

    /// Similar to emit_function, but for arrow functions
    pub fn emit_function_arrow(&mut self, func: &ArrowExpr, scope: &ScopeRef) {
        if let Some(type_params) = &func.type_params {
            self.emit_type_param(type_params);
        }
        let child_scope = self.scopes.make_child(scope);
        for param in &func.params {
            // parameters are bound to the inner scope
            self.emit_bind_pat(param, &child_scope, &scope, IdentFlavor::Variable);
        }

        if let Some(rettype) = &func.return_type {
            self.emit_type(&rettype.type_ann);
        }

        match func.body.as_ref() {
            BlockStmtOrExpr::Expr(expr) => {
                self.emit_expr(expr, &child_scope, IdentFlavor::Variable);
            }
            BlockStmtOrExpr::BlockStmt(block) => {
                self.emit_block_stmt(block, &child_scope);
            }
        }
    }

    pub fn emit_decorator(&mut self, decorator: &Decorator, scope: &Rc<RefCell<Scope>>) {
        // decorators should be treated as functions
        let _ = self.emit_expr(&decorator.expr, scope, IdentFlavor::Function);
    }
}

pub enum ExprFlavor {
    /// The expression is an identifier in the current scope
    Ident(Rc<RefCell<ScopeEntry>>),
    /// The expression is not an identifier, but has a flavor
    Flavor(IdentFlavor),
}

impl From<IdentFlavor> for ExprFlavor {
    fn from(flavor: IdentFlavor) -> Self {
        ExprFlavor::Flavor(flavor)
    }
}
