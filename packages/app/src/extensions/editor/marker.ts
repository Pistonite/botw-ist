/**
 * Diagnostic markers handler
 */

import type { MarkerData, TextModel } from "@pistonite/intwc";
import { MarkerSeverity, spanToRange } from "@pistonite/intwc";
import type { WxPromise } from "@pistonite/workex";
import { v4 as makeUUID } from "uuid";

import type { Diagnostic, ExtensionApp } from "@pistonite/skybook-api";

let lastRequestDiagnosticScript = "";
let lastRequestDiagnosticTaskIds: string[] = [];

export const provideDiagnostics = async (
    app: ExtensionApp,
    model: TextModel,
    owner: string,
): Promise<MarkerData[]> => {
    const script = model.getValue();
    let diagnostics: Awaited<WxPromise<Diagnostic[]>>;
    if (owner === "runtime") {
        const taskId = makeUUID();
        if (script !== lastRequestDiagnosticScript) {
            lastRequestDiagnosticScript = script;
            const newTaskIds = [taskId];
            const diagnosticsPromise = app.provideRuntimeDiagnostics(
                script,
                taskId,
            );
            const previousTaskIds = lastRequestDiagnosticTaskIds;
            lastRequestDiagnosticTaskIds = newTaskIds;
            previousTaskIds.forEach((id) => {
                void app.cancelRuntimeDiagnosticsRequest(id);
            });
            diagnostics = await diagnosticsPromise;
        } else {
            lastRequestDiagnosticTaskIds.push(taskId);
            diagnostics = await app.provideRuntimeDiagnostics(script, taskId);
        }
    } else {
        diagnostics = await app.provideParserDiagnostics(script);
    }
    if (diagnostics.err) {
        // this is expected if it's cancelled, let's just ignore it
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
