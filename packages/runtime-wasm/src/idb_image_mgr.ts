// currently we only support storing one image,
// the runtime will check if the image satisfies the env requirements

import { logger } from "@pistonite/pure/log";
import { makePromise } from "@pistonite/pure/sync";

// (even if it doesn't, the runtime will still run the script, just display an error)
const DbName = "BlueFlameImageDB";
const DbStore = "Image";
const DbKey = "Custom";

const log = logger("worker-imagedb", "#9226B6").debug();

/** Open the IndexedDB for the custom BlueFlame image, returns undefined if fails */
const openImageDB = (): Promise<IDBDatabase | undefined> => {
    const { promise, resolve } = makePromise<IDBDatabase | undefined>();
    const request = indexedDB.open(DbName, 1 /* version */);
    request.onerror = (event) => {
        log.error("failed to open imagedb");
        log.error(event);
        resolve(undefined);
    };
    request.onupgradeneeded = () => {
        const db: IDBDatabase = request.result;
        if (!db) {
            log.error("failed to open database: (null in onupgradeneeded)");
            return resolve(undefined);
        }
        db.onerror = (event) => {
            log.error("failed to upgrade imagedb");
            log.error(event);
            resolve(undefined);
        };
        db.createObjectStore(DbStore);
    };
    request.onsuccess = () => {
        const db: IDBDatabase = request.result;
        if (!db) {
            log.error("failed to open database: (null in onsuccess)");
            return resolve(undefined);
        }
        resolve(db);
    };
    return promise;
};

/**
 * Put the image into the IndexedDB, returns false if fails
 *
 * Undefined means to delete the stored image
 */
const putImage = async (image: Uint8Array | undefined): Promise<boolean> => {
    log.debug(`saving image to DB, size=${image?.length}`);
    const db = await openImageDB();
    if (!db) {
        return false;
    }
    try {
        return await new Promise((resolve) => {
            const tx = db.transaction(DbStore, "readwrite");
            tx.onerror = (event) => {
                log.error("failed to put image");
                log.error(event);
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
        log.error("failed to put image");
        log.error(e);
        return false;
    }
};

/** Get the image from the IndexedDB, returns undefined if fails */
const getImage = async (): Promise<Uint8Array | undefined> => {
    log.debug("getting image from DB");
    const db = await openImageDB();
    if (!db) {
        return undefined;
    }
    try {
        return await new Promise((resolve) => {
            const tx = db.transaction(DbStore, "readonly");
            const store = tx.objectStore(DbStore);
            tx.onerror = (event) => {
                log.error("failed to get image");
                log.error(event);
                resolve(undefined);
            };
            const request = store.get(DbKey);
            request.onsuccess = () => {
                const image = request.result;
                if (!image || !(image instanceof Uint8Array)) {
                    log.warn("could not get image from DB");
                    return resolve(undefined);
                }
                log.info(`got image from DB, size=${image.length}`);
                resolve(image);
            };
        });
    } catch (e) {
        log.error("failed to get image");
        log.error(e);
        return undefined;
    }
};

export const IndexedDBImageMgr = {
    putImage,
    getImage,
};
