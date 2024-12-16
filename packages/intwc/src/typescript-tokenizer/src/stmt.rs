//! Emit tokens from statements

use std::{cell::RefCell, rc::Rc};

use intwc_semantic_tokens::{TokenBuilderByPos, TokenVec};
use swc_core::ecma::ast::{BlockStmt, Stmt};

use crate::{scope::{Scope, ScopeRef}, SemanticTokenizer};

impl<'a> SemanticTokenizer<'a> {
    /// Emit tokens from a block statement with a new scope
    pub fn emit_block_stmt_inner_scope(&mut self, stmt: &BlockStmt, scope: &ScopeRef) {
        // enter inner scope
        let inner_scope = self.scopes.make_child(scope);
        self.emit_block_stmt(stmt, &inner_scope);
    }

    /// Emit tokens from a block statement in the scope
    pub fn emit_block_stmt(&mut self, stmt: &BlockStmt, scope: &ScopeRef) {
        for stmt in &stmt.stmts {
            self.emit_stmt(stmt, &scope);
        }
    }

    pub fn emit_stmt(&mut self, stmt: &Stmt, scope: &Rc<RefCell<Scope>>) {
        println!("{:#?}", stmt);
        todo!()
    }
}
