import { convertSemanticTokens, rangeToSpan } from "@pistonite/intwc";
import type {
    CancellationToken,
    SemanticTokensLegend,
    TextModel,
    Range,
    SemanticTokens,
} from "@pistonite/intwc";
import { debounce } from "@pistonite/pure/sync";
import type { ExtensionApp } from "@pistonite/skybook-api";

// need to be kept in sync with skybook_parser::SemanticToken
export const legend: SemanticTokensLegend = {
    tokenTypes: ["keyword", "variable", "type", "number", "string.item"],
    tokenModifiers: [],
};

type ProviderFn = (
    app: ExtensionApp,
    script: string,
    start: number,
    end: number,
    cancellationToken: CancellationToken,
) => Promise<number[]>;

// This is used to debounce the request per-model.
// We should only have a fixed size of models, so this won't have
// memory size issues
const cachedProvideSemanticTokenFns = new Map<TextModel, ProviderFn>();
const getProvideSemanticTokenFn = (model: TextModel): ProviderFn => {
    const cached = cachedProvideSemanticTokenFns.get(model);
    if (cached) {
        return cached;
    }
    const fn = debounce({
        fn: async (
            app: ExtensionApp,
            script: string,
            start: number,
            end: number,
            cancellationToken: CancellationToken,
        ) => {
            const tokens = await app.provideSemanticTokens(script, start, end);
            if (cancellationToken.isCancellationRequested) {
                return [];
            }
            if (tokens.err) {
                return [];
            }
            return convertSemanticTokens(tokens.val, model, {
                convertType: (raw) => {
                    if (!raw || raw > legend.tokenTypes.length) {
                        return [undefined, 0];
                    }
                    return [raw - 1, 0];
                },
            });
        },
        interval: 100,
    });

    cachedProvideSemanticTokenFns.set(model, fn);
    return fn;
};

export const provideSemanticTokens = async (
    app: ExtensionApp,
    model: TextModel,
    range: Range,
    cancellationToken: CancellationToken,
): Promise<SemanticTokens> => {
    const script = model.getValue();
    const [start, end] = rangeToSpan(model, range);
    const providerFn = getProvideSemanticTokenFn(model);
    const tokens = await providerFn(app, script, start, end, cancellationToken);
    return {
        data: new Uint32Array(tokens),
    };
};
