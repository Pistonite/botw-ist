import type { Result } from "@pistonite/pure/result";
import { create } from "zustand";

import type { InvView_Gdt, RuntimeViewError } from "@pistonite/skybook-api";

export type SaveViewerState = {
    selectedSave: string | undefined;
    saveNames: string[];
    setSelectedSave: (name: string | undefined) => void;
    setSaveNames: (names: string[]) => void;
    displayedData: Result<InvView_Gdt, RuntimeViewError> | undefined;
    setDisplayedData: (data: Result<InvView_Gdt, RuntimeViewError>) => void;

    // item display props
    cheap: boolean;
    disableAnimation: boolean;
    setItemDisplayProps: (cheap: boolean, disableAnimation: boolean) => void;
};

export const createSaveViewerStore = () => {
    return create<SaveViewerState>()((set) => ({
        selectedSave: undefined,
        saveNames: [],
        setSelectedSave: (name) => {
            set(({ saveNames }) => {
                if (name && saveNames.includes(name)) {
                    return { selectedSave: name };
                }
                return { selectedSave: undefined };
            });
        },
        setSaveNames: (names) => {
            set(({ selectedSave }) => {
                if (selectedSave && !names.includes(selectedSave)) {
                    return {
                        selectedSave: undefined,
                        saveNames: names,
                    };
                }
                return { saveNames: names };
            });
        },
        displayedData: undefined,
        setDisplayedData: (displayedData) => {
            set({ displayedData });
        },
        cheap: false,
        disableAnimation: false,
        setItemDisplayProps: (cheap, disableAnimation) => {
            set({ cheap, disableAnimation });
        },
    }));
};

export type SaveViewerStore = ReturnType<typeof createSaveViewerStore>;
