import * as monaco from "monaco-editor";

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
export type CompletionContext = monaco.languages.CompletionContext;
export type CompletionResult = monaco.languages.ProviderResult<CompletionList>;
export type MarkerResult = monaco.languages.ProviderResult<MarkerData[]>;

export type LanguageTokenizer = monaco.languages.IMonarchLanguage;
export type LanguageConfiguration = monaco.languages.LanguageConfiguration;

export type IEditorOptions = monaco.editor.IEditorOptions;
export type IGlobalEditorOptions = monaco.editor.IGlobalEditorOptions;

export type Uri = monaco.Uri;
