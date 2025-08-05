import { readdir, lstat } from "node:fs/promises";

const threshold = 4096;
const expectedRatio = 0.8;

const gzipDir = async (dir: string) => {
    const files = await readdir(dir, { recursive: true });
    const promises = files.map(async (path) => {
        // already compressed
        if (
            path.endsWith(".gz") ||
            path.endsWith(".webp") ||
            path.endsWith(".png") ||
            path.endsWith(".jpg")
        ) {
            return;
        }
        const fullPath = dir + "/" + path;
        if ((await lstat(fullPath)).isDirectory()) {
            return;
        }
        const file = Bun.file(fullPath);
        if (file.size < threshold) {
            return;
        }
        const buffer = await file.arrayBuffer();
        const compressed = Bun.gzipSync(buffer);
        const actualRatio = compressed.byteLength / buffer.byteLength;
        if (actualRatio > expectedRatio) {
            console.log(`Skipping ${fullPath} - compression ratio too low (${actualRatio})`);
            return;
        }
        console.log(`${fullPath} (${(actualRatio * 100).toFixed(2)}%)`);
        const gzipPath = fullPath + ".gz";
        await Bun.write(gzipPath, compressed);
    });
    await Promise.all(promises);
};

void gzipDir("dist/app/assets");
void gzipDir("dist/app/runtime");
void gzipDir("dist/app/static");
