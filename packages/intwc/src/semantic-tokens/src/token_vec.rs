use crate::{TokenModifier, TokenType};

/// Holder of token data according to LSP as they are parsed from a tokenizer
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenVec {
    /// Byte positions of the first character of each line
    ///
    /// This is used to calculate line position based on byte position
    line_byte_pos: Vec<usize>,
    /// The token data in the format of [delta_line, delta_start, len, type, modifier]
    token_data: Vec<u32>,
    /// The length of the source
    source_len: usize,
    /// Previously recorded line index. Used to calculate delta_line
    prev_line: usize,
    /// Previously recorded start index. Used to calculate delta_start
    prev_start: usize,
}

impl TokenVec {
    /// Create a new TokenVec to accept tokens from the given source
    pub fn new(source: &str) -> Self {
        let mut line_byte_pos = Vec::new();
        let mut current = 0;
        if !source.is_empty() {
            line_byte_pos.push(0);
        }
        let mut rest = source;
        while let Some(x) = rest.find('\n') {
            current += x + 1;
            if current < source.len() {
                line_byte_pos.push(current);
            }
            rest = &rest[x + 1..];
        }

        Self {
            line_byte_pos,
            token_data: Vec::new(),
            source_len: source.len(),
            prev_line: 0,
            prev_start: 0,
        }
    }

    /// Get the token data
    pub fn data(&self) -> &[u32] {
        &self.token_data
    }

    /// Add a token by its line and start (col) position. The positions are 0-based
    pub fn add_by_line_start(&mut self, token: (TokenType, TokenModifier), line: usize, start: usize, len: usize) {
        if len == 0 {
            return;
        }
        // ignore overlapping tokens
        if line < self.prev_line {
            return;
        }
        let line_len = self.get_line_length(line);
        if start + len <= line_len {
            // single line
            self.add_by_line_start_internal(token, line, start, len as u32);
        }
        // multiple lines
        let current_len = line_len.saturating_sub(start);
        self.add_by_line_start_internal(token, line, start, current_len as u32);
        let mut remaining_len = len.saturating_sub(current_len);
        let mut current_line = line + 1;
        while remaining_len > 0 && current_line < self.line_byte_pos.len() {
            let line_len = self.get_line_length(current_line);
            if remaining_len <= line_len {
                self.add_by_line_start_internal(token, current_line, 0, remaining_len as u32);
                break;
            }
            self.add_by_line_start_internal(token, current_line, 0, line_len as u32);
            remaining_len -= line_len;
            current_line += 1;
        }
    }
    /// Add a token by its byte position (lo, hi) in the source. lo is inclusive, hi is exclusive
    pub fn add_by_byte_pos_u32(&mut self, token: (TokenType, TokenModifier), lo: u32, hi: u32) {
        self.add_by_byte_pos(token, lo as usize, hi as usize);
    }

    /// Add a token by its byte position (lo, hi) in the source. lo is inclusive, hi is exclusive
    pub fn add_by_byte_pos(&mut self, token: (TokenType, TokenModifier), lo: usize, hi: usize) {
        if lo >= hi {
            return;
        }
        // monaco-editor does not support tokens spanning multiple lines
        // so we need to split them manually

        let (lo_line, lo_start) = match self.get_line_by_byte_pos(lo) {
            Some(x) => x,
            None => return,
        };
        let (hi_line, hi_start) = match self.get_line_by_byte_pos(hi) {
            Some(x) => x,
            None => return,
        };

        if hi_line == lo_line {
            let len = (hi- lo) as u32;
            self.add_by_line_start_internal(token, lo_line, lo_start, len);
            return;
        }

        let lo_line_token_len = self.get_line_length(lo_line).saturating_sub(lo_start);
        self.add_by_line_start_internal(token, lo_line, lo_start, lo_line_token_len as u32);
        for line in lo_line+1 .. hi_line {
            let line_len = self.get_line_length(line);
            // since hi_line is exclusive, we can always add the whole line
            self.add_by_line_start_internal(token, line, 0, line_len as u32);
        }

        // add the portion on the last line if needed
        if hi_start > 0 {
            self.add_by_line_start_internal(token, hi_line, 0, hi_start as u32);
        }
    }
    /// Add a token by its line and start (col) position. The positions are 0-based.
    /// Assumes the len fits in the line
    fn add_by_line_start_internal(&mut self, token: (TokenType, TokenModifier), line: usize, start: usize, len: u32) {
        if len == 0 {
            return;
        }
        // ignore overlapping tokens
        if line < self.prev_line {
            return;
        }
        let delta_line = line - self.prev_line;
        let delta_start = if delta_line == 0 {
            // ignore overlapping tokens
            if start <= self.prev_start {
                return;
            }
            start - self.prev_start
        } else {
            start
        };
        self.add(token, delta_line as u32, delta_start as u32, len as u32);
    }

    /// Add a token directly
    fn add(&mut self, token: (TokenType, TokenModifier), delta_line: u32, delta_start: u32, len: u32) {
            self.prev_line += delta_line as usize;
            if delta_line > 0 {
                self.prev_start = delta_start as usize;
            } else {
                self.prev_start += delta_start as usize;
            }
            self.token_data.reserve(5);
            let (typ, modifier) = token;
            let typ = typ as u32;
            let modifier = modifier as u32;
            self.token_data.push(delta_line);
            self.token_data.push(delta_start);
            self.token_data.push(len);
            self.token_data.push(typ);
            self.token_data.push(modifier);
    }


    /// Get the line and col from a byte position
    fn get_line_by_byte_pos(&self, byte_pos: usize) -> Option<(usize, usize)> {
        if byte_pos >= self.source_len {
            return None;
        }
        match self.line_byte_pos.binary_search(&byte_pos) {
            Ok(line) => Some((line, 0)),
            Err(line_plus_1) => {
                let line = line_plus_1.saturating_sub(1);
                // get the starting byte position of the line
                let line_pos = self.line_byte_pos.get(line)?;
                let start = byte_pos.saturating_sub(*line_pos);

                // let line_len = self.line_byte_pos.get(line + 1).copied().unwrap_or(self.source_len).saturating_sub(*line_pos);
                // if start >= line_len {
                //     None
                // } else {
                    Some((line, start))
                // }
            }
        }
    }

    fn get_line_length(&self, line: usize) -> usize {
        let next_line = line + 1;
        let next_line = self.line_byte_pos.get(next_line).unwrap_or(&self.source_len);
        next_line - self.line_byte_pos.get(line).copied().unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn token_vec_empty() {
        let tv = TokenVec::new("");
        assert_eq!(tv.line_byte_pos, vec![]);
    }

    #[test]
    fn token_vec_single_line() {
        let tv = TokenVec::new("hello world");
        assert_eq!(tv.line_byte_pos, vec![0]);
    }

    #[test]
    fn token_vec_2_line() {
        let tv = TokenVec::new("hello\nworld");
        assert_eq!(tv.line_byte_pos, vec![0, 6]);
    }
    #[test]
    fn token_vec_2_line_trailing() {
        let tv = TokenVec::new("hello\nworld\n");
        assert_eq!(tv.line_byte_pos, vec![0, 6]);
    }

    #[test]
    fn line_position_single_line() {
        let tv = TokenVec::new("hello world");
        assert_eq!(tv.get_line_by_byte_pos(0), Some((0, 0)));
        assert_eq!(tv.get_line_by_byte_pos(5), Some((0, 5)));
        assert_eq!(tv.get_line_by_byte_pos(10), Some((0, 10)));
        assert_eq!(tv.get_line_by_byte_pos(11), None);
        assert_eq!(tv.get_line_by_byte_pos(12), None);
    }

    #[test]
    fn line_position_2_lines() {
        let tv = TokenVec::new("hello\nworld");
        assert_eq!(tv.get_line_by_byte_pos(0), Some((0, 0)));
        assert_eq!(tv.get_line_by_byte_pos(4), Some((0, 4)));
        // the new line character is still on the first line
        assert_eq!(tv.get_line_by_byte_pos(5), Some((0, 5)));
        assert_eq!(tv.get_line_by_byte_pos(6), Some((1, 0)));
        assert_eq!(tv.get_line_by_byte_pos(7), Some((1, 1)));
        assert_eq!(tv.get_line_by_byte_pos(10), Some((1, 4)));
        assert_eq!(tv.get_line_by_byte_pos(11), None);
        assert_eq!(tv.get_line_by_byte_pos(12), None);
    }
}
