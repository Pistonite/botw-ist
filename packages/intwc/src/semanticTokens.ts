
export const legend = {
    // Should be kept in sync with semantic-tokens
    tokenTypes: [
        "comment",
        "punctuation",
        "keyword",
        "variable",
        "support",
        "constant",
        "string",
        "meta",
        "source"
    ],
    // Should be kept in sync with semantic-tokens
    tokenModifiers: [
        "operator",
        "readonly",
        "function",
        "type",
        "language",
        "boolean",
        "undefined",
        "numeric",
        "macro",
        "defaultLibrary",
        "regexp"
    ]
};

// export type SemanticTokenProvideFn = 
// (model: monaco.editor.ITextModel, 
//     lastResultId: string,
//     cancellationToken: monaco.CancellationToken
// ) => monaco.languages.ProviderResult<monaco.languages.SemanticTokens | monaco.languages.SemanticTokensEdits>;
//
// export type SemanticTokenReleaseFn = () => void;
//
// export const createDocumentSemanticTokenProvider = (provide: SemanticTokenProvideFn) => {
//     return {
//         getLegend: () => legend,
//         provideDocumentSemanticTokens: provide,
//         releaseDocumentSemanticTokens: () => {}
//     } satisfies monaco.languages.DocumentSemanticTokensProvider;
// }
