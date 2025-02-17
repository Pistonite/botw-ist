import * as monaco from 'monaco-editor';

export type TextModel = monaco.editor.ITextModel;
export type MarkerData = monaco.editor.IMarkerData;
export const MarkerSeverity = monaco.MarkerSeverity;
export type MarkerSeverity = monaco.MarkerSeverity;
export type Range = monaco.Range;
export type Position = monaco.Position;
export type CancellationToken = monaco.CancellationToken;
export type SemanticTokensProvider = monaco.languages.DocumentRangeSemanticTokensProvider;
export type SemanticTokensLegend = monaco.languages.SemanticTokensLegend;
export type SemanticTokens = monaco.languages.SemanticTokens;
export type SemanticTokensResult = monaco.languages.ProviderResult<monaco.languages.SemanticTokens>;
export type CompletionItemProvider = monaco.languages.CompletionItemProvider;
export type CompletionItem = monaco.languages.CompletionItem;
export type CompletionList = monaco.languages.CompletionList;
export type CompletionResult = monaco.languages.ProviderResult<CompletionList>;
export type MarkerResult = monaco.languages.ProviderResult<MarkerData[]>;


export type LanguageTokenizer = monaco.languages.IMonarchLanguage;
export type LanguageConfiguration = monaco.languages.LanguageConfiguration;

export type LanguageClient = {
    /** Get the language id */
    getId: () => string;
    getExtensions?: () => string[];
    /** Get the tokenizer to register on initialization */
    getTokenizer?: () => LanguageTokenizer;
    /** Get the configuration to register on initialization */
    getConfiguration?: () => LanguageConfiguration;
    /** Get the marker owners that `provideMarkers` will be called with */
    getMarkerOwners?: () => string[];
    /** Provide markers for the given model and owner */
    provideMarkers?: (model: TextModel, owner: string) => MarkerResult;

    getSemanticTokensLegend?: () => SemanticTokensLegend;

    provideDocumentRangeSemanticTokens?: (model: TextModel, range: Range, token: CancellationToken) => SemanticTokensResult;


    getCompletionTriggerCharacters?: () => string[];

    provideCompletionItems?: (model: TextModel, position: Position, context: monaco.languages.CompletionContext, token: CancellationToken) => CompletionResult;

    resolveCompletionItem?: (item: CompletionItem, token: CancellationToken) => CompletionItem,
};
