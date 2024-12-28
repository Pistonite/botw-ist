import * as monaco from 'monaco-editor-contrib';


export type TextModel = monaco.editor.ITextModel;
export type MarkerData = monaco.editor.IMarkerData;
export type Range = monaco.Range;
export type Position = monaco.Position;
export type CancellationToken = monaco.CancellationToken;
export type SemanticTokensProvider = monaco.languages.DocumentRangeSemanticTokensProvider;
export type SemanticTokensLegend = monaco.languages.SemanticTokensLegend;
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

    getSemanticTokensLegend?: () => SemanticTokensLegend;

    provideDocumentRangeSemanticTokens?: (model: TextModel, range: Range, token: CancellationToken) => SemanticTokensResult;

    getMarkerOwner?: () => string;

    provideMarkers?: (model: TextModel) => MarkerResult;

    getCompletionTriggerCharacters?: () => string[];

    provideCompletionItems?: (model: TextModel, position: Position, context: monaco.languages.CompletionContext, token: CancellationToken) => CompletionResult;

    resolveCompletionItem?: (item: CompletionItem, token: CancellationToken) => CompletionItem,
};

monaco.languages.registerCompletionItemProvider