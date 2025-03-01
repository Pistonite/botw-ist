import { errstr, Result } from "@pistonite/pure/result";

export const decodeCompressedParam = (param: string): Result<string, string> => {
    try {
    const compressedBytes = Buffer.from(param, "base64");
    const uncompressedBytes = Bun.gunzipSync(compressedBytes);
        return { val: new TextDecoder().decode(uncompressedBytes) };
    } catch (e) {
        console.error("error decoding compressed param: ", e);
        return { val: errstr(e) };
    }
}
