/** Type of the DirectLoad payload injected into the page by the server */
export type DirectLoad = {
    /**
     * Type of the payload
     * - v3: Either 'r' or 'c' parameter in the URL.
     *   The script is decompressed on the server
     * - v4: Script loaded through various means
     */
    type: "v3" | "v4",

    /** The plaintext content of the script */
    content: string,
}

// /** Extract the DirectLoad payload from the page, if exists */
// export const extractDirectLoad = (): DirectLoad | undefined => {
//     if (typeof (globalThis as any).window === "undefined") {
//         return undefined;
//     }
//     if ("__skybook_direct_load" in (globalThis as any).window) {
//         // Remove script tag that's already executed
//         document
//         return (globalThis as any).window.__skybook_direct_load;
//     }
//     return undefined;
// }
