/*
 * This file is generated by the workex CLI Tool
 *
 * Please visit https://workex.pistonite.dev for more information
 */

import type { RuntimeApp } from "../RuntimeApp.ts";

import type { WxPromise, WxBusRecvHandler, WxProtocolBoundSender } from "@pistonite/workex";
import type { ItemSearchResult } from "../types.ts";

/*
 * These generated implementations are used internally by other generated code.
 * They should not be used directly!
 */

/**
 * API provided by the simulator app that the runtime needs to call
 */
export class _wxSenderImpl implements RuntimeApp {
    private sender: WxProtocolBoundSender

    constructor(sender: WxProtocolBoundSender) {
        this.sender = sender
    }

    /**
     * Get the custom BlueFlame image provided by the user.
     * 
     * The runtime may request this if it's instructed to initialize
     * with a custom image. For the best user experience, the app should
     * prompt file selection and have the image ready before initializing,
     * and return the file in this callback.
     * 
     * If the user did not provide a custom image, the app should return undefined,
     * in which case the runtime initialization will fail.
     */
    public getCustomBlueFlameImage( ): WxPromise<Uint8Array | undefined> {
        return this.sender.send<Uint8Array | undefined>(28 /* RuntimeApp.getCustomBlueFlameImage */, [ ]);
    }

    /**
     * The app will be notified whenever a simulation run completes.
     * Note if multiple runs are queued, this will only be called for the
     * last one.
     */
    public onRunCompleted( ): WxPromise<void> {
        return this.sender.sendVoid(29 /* RuntimeApp.onRunCompleted */, [ ]);
    }

    /**
     * Resolve a quoted item search query to a single item, or
     * return undefined if the item cannot be resolved due to error
     * or no match.
     */
    public resolveQuotedItem( query: string ): WxPromise<ItemSearchResult | undefined> {
        return this.sender.send<ItemSearchResult | undefined>(30 /* RuntimeApp.resolveQuotedItem */, [ query ]);
    }
}

/**
 * API provided by the simulator app that the runtime needs to call
 */
export const _wxRecverImpl = (handler: RuntimeApp): WxBusRecvHandler => {
    return <WxBusRecvHandler>((fId, args: any[]) => { switch (fId) {
        case 28 /* RuntimeApp.getCustomBlueFlameImage */: {
            return handler.getCustomBlueFlameImage();
        }
        case 29 /* RuntimeApp.onRunCompleted */: {
            return handler.onRunCompleted();
        }
        case 30 /* RuntimeApp.resolveQuotedItem */: {
            const [ a0 ] = args;
            return handler.resolveQuotedItem( a0 );
        }
    } return Promise.resolve({ err: { code: "UnknownFunction" } }); })
};
