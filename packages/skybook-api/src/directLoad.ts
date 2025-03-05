/** Type of the DirectLoad payload injected into the page by the server */
export type DirectLoad = {
    /**
     * Type of the payload
     * - v3: Either 'r' or 'c' parameter in the URL.
     *   The script is decompressed on the server
     * - v4: Script loaded through various means
     */
    type: "v3" | "v4";

    /** The plaintext content of the script */
    content: string;

    /** If editing should be enabled by default */
    edit?: boolean;
};
