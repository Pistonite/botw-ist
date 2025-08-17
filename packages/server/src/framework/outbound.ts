import type { OutboundHook, ResponsePayload } from "./types.ts";

export const withHeadersOnSuccess = (headers: Record<string, string>): OutboundHook => {
    return (_req, ok: boolean, response: ResponsePayload) => {
        if (!ok) {
            return response;
        }
        if ((response.options?.status || 200) < 400) {
            return {
                ...response,
                options: {
                    ...(response.options || {}),
                    headers: {
                        ...(response.options?.headers || {}),
                        ...headers,
                    },
                },
            };
        }
        return response;
    };
};
