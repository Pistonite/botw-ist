
// Use webpack require context to import all images under image directory and create a map
const r = require as any; // eslint-disable-line @typescript-eslint/no-explicit-any

const images = ((requireContext)=>{
	const imgMap: {[name: string]: string} = {};
	requireContext.keys().forEach((k: string)=>{
		if(k.startsWith("./") && k.endsWith(".png")){
			const module = requireContext(k);
			// Clean image path ./name.png => name

			const name = k.substring(2, k.length - 4);
			if(typeof module === "string"){
				imgMap[name] = module;
			}else if (typeof module === "object" && "default" in module){
				imgMap[name] = module["default"];
			}else{
				console.error("Failed to load image: ", k);
			}
		}
	});
	return imgMap;
})(r.context(".", false, /\.png$/));

export default images as {readonly [name:string]:string};
