export * from "./asset_util.ts";
export * from "./first_party_extension.ts";
export * from "./log.ts";
export * from "./shared_styles.ts";
export * from "./starter_script.ts";
export * from "./view_algorithms.ts";

export const shallowEqual = <T>(a: T[], b: T[]): boolean => {
    if (a.length !== b.length) {
        return false;
    }
    const l = a.length;
    for (let i = 0; i < l; i++) {
        if (a[i] !== b[i]) {
            return false;
        }
    }
    return true;
};
