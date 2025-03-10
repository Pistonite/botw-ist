import type { ResponsePayload } from "./types.ts";

/** Make a 404 Not Found response */
export const make404 = (): ResponsePayload => {
    return makeError("Not Found", 404);
};

/** Make a response with an error message and status*/
export const makeError = (
    message: string,
    status?: number,
): ResponsePayload => {
    return {
        body: message,
        options: {
            status: status || 500,
        },
    };
};

/** Make a response sending a file */
export const makeFile = (
    path: string,
    options?: ResponsePayload["options"] | false | undefined | null,
): ResponsePayload => {
    return {
        body: Bun.file(path),
        options: options || undefined,
    };
};

/** Make a response with a plain text body */
export const makePlainText = (body: string): ResponsePayload => {
    return {
        body: body,
        options: {
            headers: {
                "Content-Type": "text/plain",
            },
        },
    };
};
