
/** Get if the request accepts Gzip encoding */
export const useAcceptsGzip = (req: Request): boolean => {
    const acceptEncoding = req.headers.get("Accept-Encoding");
    if (!acceptEncoding) {
        return false;
    }
    const parts = acceptEncoding.split(",");
    const len = parts.length;
    for (let i = 0; i < len; i++) {
        const part = parts[i].trimStart();
        if (part.match(/^[gG][zZ][iI][pP](;)?/)) {
            return true;
        }
    }
    return false;
}
