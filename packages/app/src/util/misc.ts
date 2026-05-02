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
