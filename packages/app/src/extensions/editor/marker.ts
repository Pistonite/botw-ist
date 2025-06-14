/**
 * Diagnostic markers handler
 */

import type { MarkerData, TextModel } from "@pistonite/intwc";
import { MarkerSeverity, spanToRange } from "@pistonite/intwc";
import type { Diagnostic, ExtensionApp } from "@pistonite/skybook-api";

export const provideDiagnostics = async (
    app: ExtensionApp,
    model: TextModel,
    owner: string,
): Promise<MarkerData[]> => {
    const script = model.getValue();
    const diagnostics =
        owner === "runtime"
            ? await app.provideRuntimeDiagnostics(script)
            : await app.provideParserDiagnostics(script);
    if (diagnostics.err) {
        return [];
    }
    // convert to the editor marker format
    return diagnostics.val.map((diagnostic) => {
        return convertDiagnosticToMarker(model, diagnostic);
    });
};

const convertDiagnosticToMarker = (
    model: TextModel,
    diagnostic: Diagnostic,
): MarkerData => {
    const { message, isWarning, start, end } = diagnostic;
    const range = spanToRange(model, start, end);
    return {
        message,
        severity: isWarning ? MarkerSeverity.Warning : MarkerSeverity.Error,
        ...range,
    };
};
