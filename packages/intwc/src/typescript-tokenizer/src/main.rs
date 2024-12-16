use intwc_typescript_tokenizer::SemanticTokenizer;

pub fn main() {
    let source = include_str!("test.ts");
    let tokenizer = SemanticTokenizer::new(source);
    tokenizer.tokenize();
}
