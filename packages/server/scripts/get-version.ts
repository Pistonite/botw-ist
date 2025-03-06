const packageJson = await Bun.file("../../package.json").json();
console.log(packageJson.version);
