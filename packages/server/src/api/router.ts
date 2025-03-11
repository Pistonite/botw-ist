import { convertLegacyScript } from "skybook-parser-legacy";

import {
    makeError,
    makePlainText,
    type RouteBuilder,
    type Routes,
    useStringBody,
} from "self::framework";
import type { Crypto } from "self::util";

export const createApiRoutes = (
    _crypto: Crypto,
    builder: RouteBuilder,
): Routes => {
    return {
        // Convert from V3 script to V4 script
        "/api/convert": {
            POST: builder.route({
                handler: async (req) => {
                    const body = await useStringBody(req);
                    if ("err" in body) {
                        console.error(body.err);
                        return makeError("Failed to read request body", 400);
                    }
                    const script = convertLegacyScript(body.val);
                    return makePlainText(script);
                },
            }),
        },
    };
};
