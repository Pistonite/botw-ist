import { ResponsePayload } from "./types.ts"

/** Make a 404 Not Found response */
export const make404 = (): ResponsePayload => {
    return {
        body: "Not Found",
        options: {
            status: 404
        }
    }
}

/** Make a response sending a file */
export const makeFile = (path: string): ResponsePayload => {
    return {
        body: Bun.file(path),
    }
}
