type Setter<T extends string> = `set${Capitalize<T>}`;

export type GetSetPair<N extends string, T> = {
    [n in N]: T;
} & {
    [n in Setter<N>]:(t: T)=>void;
};

export interface Ref<T> {
    get(): T,
    set(t: T): void
};

export const newRef = <T>(obj: T): Ref<T> => new RefImpl(obj);

class RefImpl<T> implements Ref<T> {
    obj: T;
    constructor(obj: T){
        this.obj = obj;
    }
    public get(): T {
        return this.obj;
    }
    public set(t: T): void{
        this.obj = t;
    }
}
