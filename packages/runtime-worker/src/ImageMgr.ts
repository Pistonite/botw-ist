/** Manager for custom image loading and storage */
export interface ImageMgr {
    /** Put the image into persisted storage. Return true if succeeded */
    putImage: (image: Uint8Array | undefined) => Promise<boolean>;
    /** Get the image from persisted storage. Return undefined if not found or failed */
    getImage: () => Promise<Uint8Array | undefined>;
}
