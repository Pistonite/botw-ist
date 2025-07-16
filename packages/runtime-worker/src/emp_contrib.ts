
/** 
 * A non-null, **E**ngine-**m**anaged **P**ointer
 *
 * This uses the ECMA FinalizationRegistry, to free the resource,
 * once this object is garbage-collected.
 *
 * The free function may be async.
 *
 * ## Memory Safety
 * There are 2 rules to ensure safe usage of an Emp:
 * - The underlying object must be kept alive until it's freed
 *   by the GC callback
 * - Usage of the raw inner pointer value should be restricted, and the Emp must be kept alive
 *   during its use. The raw pointer value should not be accessible anywhere after the Emp
 *   is no longer accessible
 */
export type Emp<T, TRepr> = {
    /** The type marker for T */
    readonly __phantom: T;
    /** The underlying pointer value */
    readonly value: TRepr;
}

export type EmpConstructor<T, TRepr> = {
    /** 
     * The marker for the Emp type, used to distinguish between multiple types
     *
     * Typically, this is a unique symbol:
     * ```typescript
     * const MyNativeType = Symbol("MyNativeType");
     * export type MyNativeType = typeof MyNativeType;
     *
     * const makeMyNativeTypeEmp = makeEmpType({
     *     marker: MyNativeType,
     *     free: (ptr) => void freeMyNativeType(ptr)
     * })
     * 
     * ```
     */
    marker: T;

    /**
     * Function to free the underlying object. Called when this Emp is garbage-collected
     */
    free: (ptr: TRepr) => void | Promise<void>
}

export const makeEmpType = <T, TRepr>({ free }: EmpConstructor<T, TRepr>): (ptr: TRepr) => Emp<T, TRepr> => {
    const registry = new FinalizationRegistry(free);
    return (ptr: TRepr) => {
        const obj = Object.freeze({ value: ptr });
        registry.register(obj, ptr);
        return obj as Emp<T, TRepr>;
    };
}

const heldEmps: Set<unknown> = new Set();

/** 
 * Execute function while guaranteeing a strong reference to the Emp exists during
 * the context fn is executing, by tying it to a global reference
 */
export const holdStrongRefs = async <TOut>(objs: unknown[], fn: () => Promise<TOut>): Promise<TOut> => {
    const l = objs.length;
    for (let i = 0;i<l;i++) {
        heldEmps.add(objs[i]);
    }
    try {
        return await fn();
    } finally {
        const l = objs.length;
        for (let i = 0;i<l;i++) {
            heldEmps.delete(objs[i]);
        }
    }
}
