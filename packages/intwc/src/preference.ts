import { useSyncExternalStore } from "react";
import { persist } from "@pistonite/pure/sync";

import { InputMode, Preference, PreferenceOption } from "./types.ts";

const getDefaultPreference = (): Preference => {
    return {
        inputMode: "code"
    };
}
const deserializePreference = (value: string): Preference => {
    try {
        return validatePreference(JSON.parse(value));
    } catch  {
        return getDefaultPreference();
    }
}

const validatePreference = (obj: unknown): Preference => {
    if (!obj || typeof obj !== "object") {
        return getDefaultPreference();
    }
    let inputMode: InputMode = "code";
    if ("inputMode" in obj) {
        const value = obj.inputMode;
        if (value === "vim" || value === "emacs") {
            inputMode = value;
        }
    }
    return {
        ...getDefaultPreference(),
        inputMode,
    }
}

const preference = persist({
    storage: localStorage,
    initial: getDefaultPreference(),
    key: "Intwc.Preference",
    deserialize: deserializePreference,
});

export const initPreference = ({persist, defaults}: PreferenceOption) => {
    let value: Preference = {
        ...getDefaultPreference(),
        ...defaults,
    }
    if (persist) {
        preference.init(value);
    } else {
        preference.disable();
        preference.set(value);
    }
}

export const addPreferenceSubscriber = (subscriber: (preference: Preference) => void
    , notifyImmediately?: boolean
): () => void => {
    return preference.subscribe(subscriber, notifyImmediately);
}

export function getPreference(): Preference {
    return preference.get();
}

export const setPreference = (newPreference: Partial<Preference>) => {
    const newPreferenceMerged = {
        ...preference.get(),
        ...newPreference,
    };
    preference.set(newPreferenceMerged);
}

export function useInputMode(): InputMode {
    const preference = useSyncExternalStore(addPreferenceSubscriber, getPreference);
    return preference.inputMode;
}

