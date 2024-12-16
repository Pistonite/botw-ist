use std::thread::Scope;

use intwc_semantic_tokens::{token, TokenBuilderByPos, TokenModifier, TokenType, TokenVec};

use scope::ScopeManager;
use swc_common::comments::SingleThreadedComments;
use swc_common::{BytePos, Span};
use swc_core::ecma::ast::{EsVersion, Module, ModuleItem, Script};
use swc_ecma_parser::lexer::Lexer;
use swc_ecma_parser::{Parser, StringInput, Syntax};

use js_sys::Uint32Array;
use wasm_bindgen::prelude::*;

mod item;
mod stmt;
mod expr;
mod util;
mod scope;
mod tstype;
mod pat;
mod jsx;


// #[wasm_bindgen]
// pub fn tokenize(source: String) -> Uint32Array {
//     let mut token_vec = TokenVec::new(&source);
//     tokenize_impl(&source, &mut token_vec);
//     Uint32Array::from(token_vec.data())
// }

pub struct SemanticTokenizer<'a> {
    pub source: &'a str,
    pub tokens: TokenBuilderByPos<u32>,
    pub scopes: ScopeManager
}

impl<'a> SemanticTokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        let tokens = TokenBuilderByPos::new(source);
        let scopes = ScopeManager::new();
        Self {
            source,
            tokens,
            scopes
        }
    }

    pub fn tokenize(mut self) -> TokenVec {
        let comments = SingleThreadedComments::default();
        let byte_start = BytePos(0u32);
        let byte_end = BytePos(self.source.len() as u32);
        let lexer = Lexer::new(
            Syntax::Typescript(Default::default()),
            EsVersion::EsNext,
            StringInput::new(self.source, byte_start, byte_end),
            Some(&comments),
        );

        let mut parser = Parser::new_from(lexer);

        let global_scope = self.scopes.global();

        // module has more features than script, try module first
        loop {
            match parser.parse_module_item() {
                Ok(item) => {
                    match item {
                        ModuleItem::ModuleDecl(module_decl) => {
                            self.emit_module_decl(&module_decl, &global_scope);
                        }
                        ModuleItem::Stmt(stmt) => {
                            self.emit_stmt(&stmt, &global_scope);
                        }
                    }
                }
                Err(_e) => {
                    println!("Failed to parse module item, trying agane: {:?}", _e);
                }
            }
        }
    }


    pub fn add_span(&mut self, token: (TokenType, TokenModifier), span: Span) {
        let start = span.lo.0 as u32;
        let end = span.hi.0 as u32;
        self.tokens.add(token, start, end);
    }

    pub fn find_and_add(&mut self, token: (TokenType, TokenModifier), span: Span, pat: &str) {
        let start = span.lo.0 as usize;
        let end = span.hi.0 as usize;
        if let Some(x) = self.source[start..end].find(pat) {
            let start = span.lo.0 + x as u32;
            self.tokens.add(token, start, start + pat.len() as u32);
        }
    }
}
