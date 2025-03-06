// This is needed because bun doesn't currently process TS project references
const f = Bun.file("tsconfig.json");
let tsconfig;
try {
    tsconfig = await f.json();
    if (!tsconfig.compilerOptions) {
        tsconfig.compilerOptions = {};
    }
    tsconfig.compilerOptions.baseUrl = "src";
} catch {
    tsconfig = {
        compilerOptions: {
            baseUrl: "src",
        },
    };
}
f.write(JSON.stringify(tsconfig, null, 4));
