const packageJson = await Bun.file("../../package.json").json();
const version = packageJson.version;
console.log(`export const VERSION = "${version}";`);
