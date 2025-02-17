import * as monaco from "monaco-editor";
import { convertSemanticTokens } from "../language/SemanticConverter";

const legend: monaco.languages.SemanticTokensLegend  = {
    tokenTypes: [
        // 1 - Class
        "type.class",
        // 2 - Enum
        "type.enum",
        // 3 - Interface
        "type.interface",
        // 4 - Namespace
        "type.namespace",
        // 5 - TypeParameter
        "type.parameter",
        // 6 - Type
        "type",
        // 7 - Parameter
        "variable.parameter",
        // 8 - Variable,
        "variable",
        // 9 - EnumMember
        "variable.other.enummember",
        // 10 - Property
        "variable",
        // 11 - Function
        "function",
        // 12 - Member
        "variable",
    ],
    tokenModifiers: [
        "declaration",
        "static",
        "async",
        "readonly",
        // "defaultLibrary"
        "language",
        "local",
    ]
}

const setTimeout = self.setTimeout;

/** Per-model request for semantic tokens */
type RangeSemanticTokensRequest = {
    scheduled: false;
} | {
    scheduled: true;
    model: monaco.editor.ITextModel;
    // merged range
    range: monaco.Range;
    // latest cancellation token
    token: monaco.CancellationToken;
    promise: Promise<monaco.languages.SemanticTokens | undefined>;
    resolve: (value: monaco.languages.SemanticTokens | undefined) => void;
    reject: (reason: unknown) => void;
};
export class DocumentRangeSemanticTokensProviderAdapter
    implements monaco.languages.DocumentRangeSemanticTokensProvider {

    private worker?: (...uris: monaco.Uri[]) => Promise<monaco.languages.typescript.TypeScriptWorker>;


    constructor(
        private maxLength: number = 50000,
        private debounceInterval: number = 500,
    ) {
    }

    // --- batcher implementation ---
    private requests: Map<string, RangeSemanticTokensRequest> = new Map();

    public provideDocumentRangeSemanticTokens(
        model: monaco.editor.ITextModel, 
        range: monaco.Range, 
        token: monaco.CancellationToken): 
    Promise<monaco.languages.SemanticTokens| undefined> {
        if (!this.shouldRun(model)) {
            return Promise.resolve(undefined);
        }
        const resource = model.uri.toString();
        const request = this.requests.get(resource);
        if (!request) {
            // not currently running any request for this model,
            // execute immediately and mark as running
            this.requests.set(resource, {scheduled: false});
            return this.executeBatched(resource, model, range, token);
        }
        return this.updateRequest(request, model, resource, range, token);
    }

    private shouldRun(model: monaco.editor.ITextModel): boolean {
        if (this.maxLength > 0 && model.getValueLength() > this.maxLength) {
            return false;
        }
        return true;
    }

    private updateRequest(
        request: RangeSemanticTokensRequest, 
        model: monaco.editor.ITextModel,
        resource: string,
        range: monaco.Range, 
        token: monaco.CancellationToken
    ): Promise<monaco.languages.SemanticTokens | undefined> {
        if (request.scheduled) {
            // abandon old range, since it's probably out of view anyway
            request.range = range;
            request.token = token;
            return request.promise;
        }
        let resolve;
        let reject;
        const promise = new Promise<monaco.languages.SemanticTokens | undefined>((res, rej) => {
            resolve = res;
            reject= rej;
        });
        this.requests.set(resource, {
            scheduled: true,
            model,
            range,
            token,
            promise,
            resolve: resolve!,
            reject: reject!
        });
        return promise;
    }

    private onRequestFinished(resource: string) {
        const request = this.requests.get(resource);
        if (!request) {
            return;
        }
        if (!request.scheduled) {
            // delete the entry to signify that no current
            // request is running for this model
            this.requests.delete(resource);
            return;
        }

        this.requests.set(resource, {scheduled: false});

        const { model, range, token, resolve, reject } = request;
        this.executeBatched(resource, model, range, token).then(resolve, reject);
    }

    // --- adapter implementation ---

    getLegend(): monaco.languages.SemanticTokensLegend {
        return legend;
    }

    async executeBatched(
        resource: string,
        model: monaco.editor.ITextModel, 
        range: monaco.Range, 
        token: monaco.CancellationToken): 
    Promise<monaco.languages.SemanticTokens| undefined> {
        let isWaitingForWorker = false;
        const cb = () => {
            if (!isWaitingForWorker) {
                this.onRequestFinished(resource);
                return;
            }
            setTimeout(cb, this.debounceInterval);
        };
        setTimeout(cb, this.debounceInterval);
        const start = model.getOffsetAt({
            lineNumber: range.startLineNumber,
            column: range.startColumn
        })
        const end = model.getOffsetAt({
            lineNumber: range.endLineNumber,
            column: range.endColumn
        })
        const worker = await this.getWorker(model.uri);
        // check after await
        if (model.isDisposed() || token.isCancellationRequested) {
            return undefined;
        }
        isWaitingForWorker = true;
        const result = await worker.getEncodedSemanticClassifications(resource, start, end);
        isWaitingForWorker = false;
        // check after await
        if (!result || model.isDisposed() || token.isCancellationRequested) {
            return undefined;
        }
        const { spans } = result;
        const data = this.convertTokens(model, spans );
        return {
            data: new Uint32Array(data)
        }
    }

    private convertTokens(model: monaco.editor.ITextModel, inputs: number[]): number[] {
        return convertSemanticTokens(inputs, model, {
            convertType: (raw) => {
                let modifier = raw;
                let type = raw >> 8;
                // type should be 1-indexed
                if (!type || type > legend.tokenTypes.length) {
                    return [undefined, 0];
                }

                // fix the type and modifiers to have better highlighting

                // readonly + lower bits (declaration, static, async)
                if ((modifier & 0b1000) && (modifier & 0b111)) {
                    // only keep readonly, so less important
                    // modifiers don't take priority
                    modifier = 0b1000;
                }
                // special handling for property and member
                if (type === 10 || type === 12) {
                    // ignore non-readonly modifiers
                    // for things like foo.bar(), we want to hightlight
                    // bar as a function instead of variable
                    if (!(modifier & 0b1000)) {
                        return [undefined, 0];
                    }
                    // only keep defaultLibrary on non-property/member
                    // this is for things like [].length, where length
                    // would be highlighted as a normal property,
                    // instead of the same as this, self, super, etc..
                    modifier &= ~0b10000;
                } 
                // offset by 1
                type--;
                // only keep the bits of modifier that matters
                modifier &= 0xff;
                return [type, modifier];
            }
        });
    }

    // --- debug/utils --- can be removed when upstreaming the work

    // lazy get the worker, since typescript may not be loaded yet
    private async getWorker(resource: monaco.Uri): Promise<monaco.languages.typescript.TypeScriptWorker> {
        while (!this.worker) {
            console.log("getting instance of TypeScript worker...");
            try {
                this.worker = await monaco.languages.typescript.getTypeScriptWorker();
                if (!this.worker) {
                    throw new Error("getTypeScriptWorker returned undefined");
                }
                break;
            } catch (e) {
                console.error("Failed to get worker", e);
                console.warn("will try again in a bit. This should not happen when this is initialized as part of TS mode");
                await new Promise(r => setTimeout(r, 1000));
            }
        }
        return await this.worker(resource);
    }
}
