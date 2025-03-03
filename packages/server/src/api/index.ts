
import { convertLegacyScript } from "skybook-parser-legacy";

import { makeError, makePlainText, RouteBuilder, useStringBody } from "framework";

import { Crypto } from "crypt";

export const createApiRoutes = (
    crypto: Crypto,
    builder: RouteBuilder
) => {

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
                }
            })
        },

        "/api/test": builder.route({
            handler: async (req) => {
                const r = crypto.encrypt("fooasdklfjalskdjflkajsdlkfjlasdjlkfajsdfa");
                if ("err" in r) {
                    return makeError(r.err || "", 500);
                }
                console.log(r);
                const r2 = crypto.decrypt(r.val);
                console.log(r2);
                return makePlainText("OK");
            }
        }),
    }

}
