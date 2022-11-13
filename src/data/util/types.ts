type Setter<T extends string> = `set${Capitalize<T>}`;

export type GetSetPair<N extends string, T> = {
    [n in N]: T;
} & {
    [n in Setter<N>]:(t: T)=>void;
};
