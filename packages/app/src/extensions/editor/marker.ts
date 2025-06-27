/**
 * Diagnostic markers handler
 */

import type { MarkerData, TextModel } from "@pistonite/intwc";
import { MarkerSeverity, spanToRange } from "@pistonite/intwc";
import type { WxPromise } from "@pistonite/workex";

import type { Diagnostic, ExtensionApp } from "@pistonite/skybook-api";

const EDITOR_EXTENSION_TASK_UUID = "b1b45de4-1df7-4832-ae0b-99b516f81df6";

export const provideDiagnostics = async (
    app: ExtensionApp,
    model: TextModel,
    owner: string,
): Promise<MarkerData[] | undefined> => {
    const script = model.getValue();
    let diagnostics: Awaited<WxPromise<Diagnostic[]>>;
    if (owner === "runtime") {
        const result = await app.provideRuntimeDiagnostics(
            script,
            EDITOR_EXTENSION_TASK_UUID,
        );
        if (result.val?.type === "Aborted") {
            // don't update the markers if the run was aborted,
            // since the next run will update it
            return undefined;
        }
        if (result.val) {
            diagnostics = { val: result.val.value };
        } else {
            diagnostics = result;
        }
    } else {
        diagnostics = await app.provideParserDiagnostics(script);
    }
    if (diagnostics.err) {
        console.error("failed to get diagnostics", diagnostics.err);
        return undefined;
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
