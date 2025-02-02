import { create } from "zustand";
import { persist } from "zustand/middleware";

export type ApplicationStore = {
    script: string;
};

export const useApplicationStore = create<ApplicationStore>()(
    persist(
        (set) => ({
            script: "",
        }),
        {
            name: "Skybook.Application",
        },
    ),
);
