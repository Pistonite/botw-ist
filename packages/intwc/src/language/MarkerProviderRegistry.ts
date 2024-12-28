import * as monaco from 'monaco-editor-contrib';
import type { MarkerData, MarkerResult, TextModel } from "./LanguageClient.ts";
import { serial } from '@pistonite/pure/sync';

export type MarkerProvider = {
    owner: string;
    provide: (model: TextModel, checkCancel: () => void) => MarkerResult;
};
const registeredProviders = new Map<string, MarkerProvider[]>();

export const registerMarkerProvider = (languageId: string, provider: MarkerProvider) => {
    const providers = registeredProviders.get(languageId);
    if (!providers) {
        registeredProviders.set(languageId, [provider]);
        return;
    }
    providers.push(provider);
}

const provideMarkersCallback = 
        serial({
        fn: (checkCancel) => (model: TextModel) => {
        return provideMarkers(model, checkCancel);
        }
    });

export const getProvideMarkersCallback = (): (model: TextModel) => void => {
    return provideMarkersCallback;
}

const provideMarkers = async (model: TextModel, checkCancel: () => void) => {
    const languageId = model.getLanguageId();
    const providers = registeredProviders.get(languageId);
    if (!providers) {
        return [];
    }
    const length = providers.length;
    for (let i = 0; i < length; i++) {
        const provider = providers[i];
        const markers = await provider.provide(model, checkCancel);
        checkCancel();
        if (markers) {
            monaco.editor.setModelMarkers(model, provider.owner, markers);
        }
    }
}


