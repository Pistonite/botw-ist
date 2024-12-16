use crate::{TokenModifier, TokenType, TokenVec};


/// A token builder that allows inserting tokens out-of-order,
/// by their byte position
pub struct TokenBuilderByPos<T> {
    data: Vec<TokenBuilderByPosPayload<T>>,
    token_vec: TokenVec,
}

impl<T> TokenBuilderByPos<T> {
    pub fn new(source: &str) -> Self {
        Self {
            data: Vec::new(),
            token_vec: TokenVec::new(source),
        }
    }

    pub fn add(&mut self, token: (TokenType, TokenModifier), lo: T, hi: T) {
        self.data.push(TokenBuilderByPosPayload { token, lo, hi });
    }

}

impl TokenBuilderByPos<u32> {
    pub fn done(mut self) -> TokenVec {
        self.data.sort_by_key(|x| x.lo);
        for payload in self.data {
            self.token_vec.add_by_byte_pos_u32(payload.token, payload.lo, payload.hi);
        }
        self.token_vec
    }
}

impl TokenBuilderByPos<usize> {
    pub fn done(mut self) -> TokenVec {
        self.data.sort_by_key(|x| x.lo);
        for payload in self.data {
            self.token_vec.add_by_byte_pos(payload.token, payload.lo, payload.hi);
        }
        self.token_vec
    }
}

struct TokenBuilderByPosPayload<T> {
    token: (TokenType, TokenModifier),
    lo: T,
    hi: T,
}
