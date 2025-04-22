// currently we only support storing one image,
// the runtime will check if the image satisfies the env requirements
// (even if it doesn't, the runtime will still run the script, just display an error)
const DbName = "BlueFlameImageDB";
const DbStore = "Image";
const DbKey = "Custom";

// TODO: errors from the worker are currently logged to console
// and returned as blanket errors. Tracked by #69

/** Open the IndexedDB for the custom BlueFlame image, returns undefined if fails */
const openImageDB = (): Promise<IDBDatabase | undefined> => {
    return new Promise((resolve) => {
        const request = indexedDB.open(DbName, 1);
        request.onerror = (event) => {
            console.error("Failed to open imagedb", event);
            resolve(undefined);
        };
        request.onupgradeneeded = () => {
            const db: IDBDatabase = request.result;
            if (!db) {
                console.error(
                    "Failed to open database: (null in onupgradeneeded)",
                );
                return resolve(undefined);
            }
            db.onerror = (event) => {
                console.error("Failed to upgrade imagedb", event);
                resolve(undefined);
            };
            db.createObjectStore(DbStore);
        };
        request.onsuccess = () => {
            const db: IDBDatabase = request.result;
            if (!db) {
                console.error("Failed to open database: (null in onsuccess)");
                return resolve(undefined);
            }
            resolve(db);
        };
    });
};

/**
 * Put the image into the IndexedDB, returns false if fails
 *
 * Undefined means to delete the stored image
 */
export const putImage = async (
    image: Uint8Array | undefined,
): Promise<boolean> => {
    const db = await openImageDB();
    if (!db) {
        return false;
    }
    try {
        return await new Promise((resolve) => {
            const tx = db.transaction(DbStore, "readwrite");
            tx.onerror = (event) => {
                console.error("Failed to put image", event);
                resolve(false);
            };
            tx.oncomplete = () => {
                resolve(true);
            };
            const store = tx.objectStore(DbStore);
            if (image) {
                store.put(image, DbKey);
            } else {
                store.delete(DbKey);
            }
        });
    } catch (e) {
        console.error("Failed to put image", e);
        return false;
    }
};

/** Get the image from the IndexedDB, returns undefined if fails */
export const getImage = async (): Promise<Uint8Array | undefined> => {
    const db = await openImageDB();
    if (!db) {
        return undefined;
    }
    try {
        return await new Promise((resolve) => {
            const tx = db.transaction(DbStore, "readonly");
            const store = tx.objectStore(DbStore);
            tx.onerror = (event) => {
                console.error("Failed to get image", event);
                resolve(undefined);
            };
            const request = store.get(DbKey);
            request.onsuccess = () => {
                const image = request.result;
                if (!image || !(image instanceof Uint8Array)) {
                    return resolve(undefined);
                }
                resolve(image);
            };
        });
    } catch (e) {
        console.error("Failed to get image", e);
        return undefined;
    }
};
