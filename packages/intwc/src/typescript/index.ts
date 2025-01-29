import * as monaco from "monaco-editor-contrib";
import { language } from "./language.ts";
import { DocumentRangeSemanticTokensProviderAdapter } from "./semantic.ts";

export type Option = {
    /** maximum source length to enable semantic highlighting */
    semanticTokensMaxLength?: number;
}

export function patchMonacoTypeScript(options?: Option) {
    monaco.languages.setMonarchTokensProvider("typescript", language);
    monaco.languages.registerDocumentRangeSemanticTokensProvider(
        "typescript", 
        new DocumentRangeSemanticTokensProviderAdapter(options?.semanticTokensMaxLength)
    );
}
