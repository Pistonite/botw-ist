import { errstr, type Result } from "@pistonite/pure/result";

/** Decode compressed parameter into plain text */
export const decodeCompressedParam = (
    param: string,
): Result<string, string> => {
    try {
        const compressedBytes = Buffer.from(param, "base64");
        const uncompressedBytes = Bun.gunzipSync(compressedBytes);
        return { val: new TextDecoder().decode(uncompressedBytes) };
    } catch (e) {
        console.error("error decoding compressed param: ", e);
        return { err: errstr(e) };
    }
};

/** Encode plain text into compressed parameter */
export const encodeScript = (script: string): Result<string, string> => {
    try {
        const bytes = new TextEncoder().encode(script);
        const compressedBytes = Bun.gzipSync(bytes);
        const encoded = compressedBytes.toBase64({ alphabet: "base64url" });
        return { val: encoded };
    } catch (e) {
        console.error("error encoding script: ", e);
        return { err: errstr(e) };
    }
};
