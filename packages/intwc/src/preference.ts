import { useEffect, useState } from "react";
import { InputMode, Preference, PreferenceOption } from "./types.ts";

const getDefaultPreference = (): Preference => {
    return {
        inputMode: "code"
    };
}
let preference: Preference = getDefaultPreference();
const subscribers: ((preference: Preference) => void)[] = [];

export const initPreference = ({persist, defaults}: PreferenceOption) => {
    let value: Preference = {
        ...getDefaultPreference(),
        ...defaults,
    }
    if (persist) {
        value = loadPreference();
        addPreferenceSubscriber((preference) => {
            localStorage.setItem(KEY, JSON.stringify(preference));
        });
    } else {
        localStorage.removeItem(KEY);
    }

    setPreference(value);
}

export const addPreferenceSubscriber = (subscriber: (preference: Preference) => void
    , notifyImmediately?: boolean
) => {
    subscribers.push(subscriber);
    if (notifyImmediately) {
        subscriber(preference);
    }
}

export const removePreferenceSubscriber = (
    subscriber: (preference: Preference) => void
): void => {
    const index = subscribers.indexOf(subscriber);
    if (index >= 0) {
        subscribers.splice(index, 1);
    }
};

const KEY = "Intwc.Preference";

export function getPreference(): Preference {
    return preference;
}

export const setPreference = (newPreference: Partial<Preference>) => {
    preference = {
        ...preference,
        ...newPreference,
    };
    const len = subscribers.length;
    for (let i = 0; i < len; i++) {
        subscribers[i](preference);
    }
}

export function useInputMode(): InputMode {
    const [value, setValue] = useState(getPreference().inputMode);
    useEffect(() => {
        const { inputMode } = getPreference();
        if (inputMode !== value) {
            setValue(inputMode);
        }
        const subscriber = ({inputMode}: Preference) => {
            setValue(inputMode);
        };
        addPreferenceSubscriber(subscriber);
        return () => {
            removePreferenceSubscriber(subscriber);
        };
    }, []);

    return value;
}

const loadPreference = (): Preference => {
    try {
        const objString = localStorage.getItem(KEY);
        if (!objString) {
            return getDefaultPreference();
        }
        return validatePreference(JSON.parse(objString));
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

