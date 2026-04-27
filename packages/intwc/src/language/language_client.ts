import type {
    Range,
    CancellationToken,
    CompletionContext,
    CompletionItem,
    CompletionResult,
    LanguageConfiguration,
    LanguageTokenizer,
    Position,
    SemanticTokensLegend,
    SemanticTokensResult,
    TextModel,
} from "../monaco_types.ts";

import type { DiagnosticProvider } from "./diagnostic_provider.ts";

export type LanguageClient = {
    /** Get the language id */
    getId: () => string;
    getExtensions?: () => string[];
    /** Get the tokenizer to register on initialization */
    getTokenizer?: () => LanguageTokenizer;
    /** Get the configuration to register on initialization */
    getConfiguration?: () => LanguageConfiguration;

    /** Get diagnostic providers for this language */
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    getDiagnosticProviders?: () => DiagnosticProvider<any, any>[];

    /** Get the semantic token provider for this language */
    getSemanticTokensProvider?: () => SemanticProvider;

    getCompletionTriggerCharacters?: () => string[];

    provideCompletionItems?: (
        model: TextModel,
        position: Position,
        context: CompletionContext,
        token: CancellationToken,
    ) => CompletionResult;

    resolveCompletionItem?: (item: CompletionItem, token: CancellationToken) => CompletionItem;
};

export type SemanticProvider = {
    legend: SemanticTokensLegend;
    provideDocumentRangeSemanticTokens: (
        model: TextModel,
        range: Range,
        token: CancellationToken,
    ) => SemanticTokensResult;
};
