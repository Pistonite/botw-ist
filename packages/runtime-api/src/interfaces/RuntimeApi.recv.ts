/**
 * This file is generated by workex
 */
import { type WorkexBindOptions, bindHost } from "@pistonite/workex";
import type { RuntimeApi } from "../protocol.ts";

export function bindRuntimeApiHost(delegate: RuntimeApi, options: WorkexBindOptions) {
    return bindHost("runtime", options, (fId: number, _payload: any[]) => {
        switch (fId) {
            case 16 /* RuntimeApi.resolveItemIdent */: {
                const [ a0 ] = _payload;
                return delegate.resolveItemIdent( a0 );
            }
            case 17 /* RuntimeApi.setScript */: {
                const [ a0 ] = _payload;
                return delegate.setScript( a0 );
            }
        }
        return undefined;
    });
}