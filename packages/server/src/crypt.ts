import { errstr, Result } from "@pistonite/pure/result";
import crypto from "crypto";

export type Crypto = {
    /** 
     * Encrypt a string value. Returns base64 encoded string
     */
    encrypt(input: string): Result<string, string>;
    /** 
     * Decrypt a base64 encoded string previously encrypted with this manager
     *
     * Note the error message has the reason why the decryption failed.
     * This might not be suitable to expose to the attacker
     */
    decrypt(input: string): Result<string, string>;
}

export const randomKey = (): string => {
    const randomBytes = crypto.randomBytes(64);
    return randomBytes.toString("hex");
}

/**
 * Create a crypto object with the master key
 */
export const createCrypto = (masterKey: string): Result<Crypto, string> => {
    if (masterKey.length !== 128) {
        return {
            err: "Master key must be 128 characters long"
        };
    }
    const aesKeyString = masterKey.substring(0, 64);
    const aesKey = hexToBytes(aesKeyString);
    if ("err" in aesKey) {
        return {err: aesKey.err || "Failed to parse AES key"};
    }
    const hmacKeyString = masterKey.substring(64);
    const hmacKey = hexToBytes(hmacKeyString);
    if ("err" in hmacKey) {
        return {err: hmacKey.err || "Failed to parse HMAC key"};
    }

    return {
        val: new CryptoImpl(aesKey.val, hmacKey.val)
    };
}

const hexToBytes = (hex: string): Result<Buffer, string> => {
    if (hex.length % 2 !== 0) {
        return {
            err: "Hex string must be even length"
        };
    }
    try {
        return { val: Buffer.from(hex, "hex") };
    } catch (e) {
        return {
            err: "Failed to parse hex string: "+errstr(e)
        };
    }
}

class CryptoImpl implements Crypto {
    /** 256-bit key used for AES encryption */
    private aesKey: Uint8Array;
    /** 256-bit key used for message verification with HMAC */
    private hmacKey: Uint8Array;

    private blockSize = 16;
    private textEncoder = new TextEncoder();
    private textDecoder = new TextDecoder("utf-8", {fatal: true});

    constructor(aesKey: Uint8Array, hmacKey: Uint8Array) {
        this.aesKey = aesKey;
        this.hmacKey = hmacKey;
    }

    public encrypt(input: string): Result<string, string> {
        try {
            // Construct the input - note that crypto API automatically pads the input
            const inputBytes = this.textEncoder.encode(input);
            const iv = this.getInitializationVector();

            const aesCipher = crypto.createCipheriv("aes-256-cbc", this.aesKey, iv);
            // not sure why node/bun has an extra block in the cipher text.. ?
            const cipherBytes = Buffer.concat([aesCipher.update(inputBytes), aesCipher.final()]);

            const hasher = new Bun.CryptoHasher("sha256", this.hmacKey);
            hasher.update(iv);
            hasher.update(cipherBytes);
            const hmac = hasher.digest();

            // Construct the output - 32 is for HMAC digest
            const output = 
            Buffer.concat([iv, cipherBytes, hmac]);

            // Emit base64 string output
            return {
                val: output.toString("base64")
            };
        } catch (e) {
            console.error("Failed to encrypt value");
            console.error(e);
            return {
                err: "Failed to encrypt value: "+errstr(e)
            };
        }
    }

    public decrypt(input: string): Result<string, string> {
        try {
            // Convert base64 string to buffer
            const inputBytes = Buffer.from(input, "base64");
            // extract IV, message, HMAC
            if (inputBytes.length < this.blockSize + 32) {
                return {
                    err: "Input too short"
                };
            }
            if (inputBytes.length % this.blockSize !== 0) {
                return {
                    err: "Input not multiple of block size"
                };
            }
            const hmac = inputBytes.subarray(inputBytes.length - 32);
            const iv = inputBytes.subarray(0, this.blockSize);
            const cipherBytes = inputBytes.subarray(this.blockSize, inputBytes.length - 32);

            // Verify the HMAC
            const hasher = new Bun.CryptoHasher("sha256", this.hmacKey);
            hasher.update(iv);
            hasher.update(cipherBytes);
            const realHmac = hasher.digest();
            if (!hmac.equals(realHmac)) {
                return {
                    err: "Failed to verify the message"
                };
            }

            // Decrypt the message
            const aesDecipher = crypto.createDecipheriv("aes-256-cbc", this.aesKey, iv);
            const decryptedBytes = Buffer.concat([aesDecipher.update(cipherBytes), aesDecipher.final()]);

            return { val: this.textDecoder.decode(decryptedBytes) };
        } catch (e) {
            console.error("Failed to decrypt value");
            console.error(e);
            return {
                err: "Failed to decrypt value: "+errstr(e)
            }
        }
    }

    /** Get a random IV */
    private getInitializationVector(): Buffer {
        return crypto.randomBytes(this.blockSize);
    }
}
