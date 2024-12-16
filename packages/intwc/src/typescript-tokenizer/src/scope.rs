//! Manage variable types across scopes

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use intwc_semantic_tokens::TokenBuilderByPos;
use swc_common::Span;
use swc_core::ecma::ast::Ident;

use crate::util::{IdentFlavor, IdentOrIdentNameRef};

#[derive(Debug)]
pub struct ScopeManager {
    scopes: Vec<ScopeRef>,
}

impl ScopeManager {
    pub fn new() -> Self {
        let global = Scope::global();
        let global = Rc::new(RefCell::new(global));
        Self {
            scopes: vec![global],
        }
    }

    /// Get the global scope
    pub fn global(&self) -> ScopeRef {
        Rc::clone(&self.scopes[0])
    }

    /// Make a new child scope from the parent
    pub fn make_child(&mut self, parent: &ScopeRef) -> ScopeRef {
        let child = Scope::new_child(parent);
        let child = Rc::new(RefCell::new(child));
        self.scopes.push(Rc::clone(&child));
        child
    }

    /// Emit tokens for all identifiers in all scopes
    pub fn emit(&self, tokens: &mut TokenBuilderByPos<u32>) {
        for scope in &self.scopes {
            let scope = scope.borrow();
            scope.emit(tokens);
        }
    }
}

pub type ScopeRef = Rc<RefCell<Scope>>;

#[derive(Debug)]
pub struct Scope {
    /// The parent scope
    pub parent: Option<ScopeRef>,
    /// Variables in this scope
    pub data: Rc<RefCell<ScopeData>>,
}

impl Scope {
    /// Create a new global scope
    pub fn global() -> Self {
        Self {
            parent: None,
            data: Rc::new(RefCell::new(ScopeData::default())),
        }
    }
    /// Create a child scope
    pub fn new_child(s: &Rc<RefCell<Scope>>) -> Self {
        Self {
            parent: Some(Rc::clone(s)),
            data: Rc::new(RefCell::new(ScopeData::default())),
        }
    }

    /// Add an identifier reference.
    ///
    /// Looks up the identifier in the scope and parent scopes,
    /// and adds the span to the found entry. If flavor is provided,
    /// also set the flavor for the identifier.
    pub fn add_reference<'a>(&self, ident: impl Into<IdentOrIdentNameRef<'a>>, flavor: Option<IdentFlavor>) -> Rc<RefCell<ScopeEntry>> {
        let ident = ident.into();
        let found = {
            let data = self.data.borrow();
            data.add_reference(ident, flavor)
        };
        let entry = match found {
            Some(entry) => {
                // found in this scope, from this scope or from parent cache
                return entry
            },
            None => {
                if let Some(parent) = &self.parent {
                    // look up in the parent
                    let r = {
                        let parent = parent.borrow();
                        parent.add_reference(ident, flavor)
                    };
                    r
                } else {
                    // it's probably a global/external entry
                    let entry = ScopeEntry::new_global(ident, flavor);
                    Rc::new(RefCell::new(entry))
                }
            }
        };
        // cache it in this scope
        {
            let mut data = self.data.borrow_mut();
            data.insert_from_parent(ident, Rc::clone(&entry));
        }
        entry
    }

    /// Add a declaration of the identifier with an initial flavor
    ///
    /// If None, the flavor will be guessed from the identifier
    pub fn add_declaration<'a>(&self, ident: impl Into<IdentOrIdentNameRef<'a>>, flavor: Option<IdentFlavor>) {
        let ident = ident.into();
        let flavor = flavor.unwrap_or_else(|| IdentFlavor::guess(ident));
        {
            let mut data = self.data.borrow_mut();
            data.declare(ident, flavor, self.parent.is_none());
        }
    }

    /// Emit tokens in this scope to the token builder
    pub fn emit(&self, tokens: &mut TokenBuilderByPos<u32>) {
        let data = self.data.borrow();
        data.emit(tokens);
    }
}

#[derive(Debug, Default, Clone)]
pub struct ScopeData {
    /// Variables declared in the scope that are still accessible
    pub vars: BTreeMap<String, Rc<RefCell<ScopeEntry>>>,
    /// Variables declared in the scope that are no longer accessible
    pub shallowed: Vec<Rc<RefCell<ScopeEntry>>>,
    /// Cache of identifiers from the parent scope
    pub parent_cache: BTreeMap<String, Rc<RefCell<ScopeEntry>>>,
}

impl ScopeData {
    pub fn declare<'a>(&mut self, ident: impl Into<IdentOrIdentNameRef<'a>>, flavor: IdentFlavor, is_global_scope: bool) {
        // if this is the global scope, and there is a previously found external entry,
        // we can rectify it now. This is needed because things from outer scope
        // can be declared later

        let ident = ident.into();
        if is_global_scope {
            if let Some(entry) = self.vars.get(ident.as_ref()) {
                let mut entry = entry.borrow_mut();
                if entry.flavor == IdentFlavor::WeakExternal {
                    entry.add_reference(ident.span(), Some(flavor));
                    return;
                }
            }
        }
        let entry = Rc::new(RefCell::new(
            ScopeEntry::new_declaration(ident, flavor)
        ));
        if let Some(old) = self.vars.insert(ident.as_ref().to_string(), entry) {
            self.shallowed.push(old);
        }
    }
    /// Add the ident reference to self if it's found in the scope
    ///
    /// Returns true if the identifier is found in the scope
    pub fn add_reference<'a>(&self, ident: impl Into<IdentOrIdentNameRef<'a>>, flavor: Option<IdentFlavor>) -> Option<Rc<RefCell<ScopeEntry>>> {
        let ident = ident.into();
        if let Some(entry) = self.vars.get(ident.as_ref()) {
            {
                let mut entry = entry.borrow_mut();
                entry.add_reference(ident.span(), flavor);
            }
            return Some(Rc::clone(entry));
        }
        // try to find in the parent cache
        if let Some(entry) = self.parent_cache.get(ident.as_ref()) {
            {
                let mut entry = entry.borrow_mut();
                entry.add_reference(ident.span(), flavor);
            }
            return Some(Rc::clone(entry));
        }
        None
    }

    pub fn insert_from_parent<'a>(&mut self, ident: impl Into<IdentOrIdentNameRef<'a>>, entry: Rc<RefCell<ScopeEntry>>) {
        self.parent_cache.insert(ident.into().as_ref().to_string(), entry);
    }

    /// Emit tokens in this scope to the token builder
    pub fn emit(&self, tokens: &mut TokenBuilderByPos<u32>) {
        for entry in self.vars.values() {
            let mut entry = entry.borrow_mut();
            entry.emit(tokens);
        }
        for entry in &self.shallowed {
            let mut entry = entry.borrow_mut();
            entry.emit(tokens);
        }
    }
}

#[derive(Debug)]
pub struct ScopeEntry {
    /// If the entry is already emitted to the token builder,
    /// so we don't emit duplicates
    pub emitted: bool,
    /// Current flavor of the entry
    pub flavor: IdentFlavor,
    /// Current registered spans of the entry
    pub spans: Vec<(u32, u32)>,

    /// Link self to another entry
    ///
    /// If the flavor of self is updated, it will also update the linked entries
    pub links: Vec<Rc<RefCell<ScopeEntry>>>,
}

impl ScopeEntry {
    /// Create a new declaration entry
    pub fn new_declaration<'a>(ident: impl Into<IdentOrIdentNameRef<'a>>, flavor: IdentFlavor) -> Self {
        let span = ident.into().span();
        Self {
            emitted: false,
            flavor,
            spans: vec![(span.lo.0, span.hi.0)],
            links: Vec::new(),
        }
    }

    /// Create a new global/external entry
    pub fn new_global<'a>(ident: impl Into<IdentOrIdentNameRef<'a>>, flavor: Option<IdentFlavor>) -> Self {
        let mut e = Self {
            emitted: false,
            flavor: IdentFlavor::WeakExternal,
            spans: Vec::new(),
            links: Vec::new(),
        };
        e.add_reference(ident.into().span(), flavor);
        e
    }

    /// Add a reference to the identifier
    pub fn add_reference(&mut self, span: Span, flavor: Option<IdentFlavor>) {
        self.spans.push((span.lo.0, span.hi.0));
        if let Some(flavor) = flavor {
            self.set_flavor(flavor);
        }
    }

    pub fn set_flavor(&mut self, flavor: IdentFlavor) {
        // prevent circular dependency
        if self.emitted {
            return;
        }
        self.flavor = self.flavor.max(flavor);
        self.emitted = true;
        for link in &self.links {
            // cannot borrow mut also means circular dependency
            if let Ok(mut x) = link.try_borrow_mut() {
                x.set_flavor(self.flavor);
            }
        }
        self.emitted = false;
    }

    /// Emit the tokens for the identifier
    pub fn emit(&mut self, tokens: &mut TokenBuilderByPos<u32>) {
        if self.emitted {
            return;
        }
        self.emitted = true;
        let token = self.flavor.to_token();
        for (lo, hi) in &self.spans {
            tokens.add(token, *lo, *hi);
        }
    }

    pub fn add_link(&mut self, link: Rc<RefCell<ScopeEntry>>) {
        self.links.push(link);
    }
}
