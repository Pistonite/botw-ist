import { MetaOption } from "data/item";

export const parseMetadata = (metaString: string): MetaOption | string => {
	if(metaString.trim().length === 0){
		return {};
	}
	const parts = metaString.toLowerCase().split(",");
	const option: MetaOption = {};
	for(let i =0;i<parts.length;i++){
		const part = parts[i];
		const j = part.indexOf("=");
		let key, value;
		if(j===-1){
			key = part;
			value = "true";
		}else{
			key = part.substring(0,j);
			value = part.substring(j+1);
		}
		key = key.trim();
		switch(key){
			case "life": {
				const life = parseInt(value.trim());
				if (!Number.isInteger(life)){
					return `Life must be an integer: ${value.trim()}`;
				}
				option.life = life;
				break;
			}
			case "equip": {
				const equip = value === "true";
				option.equip = equip;
				break;
			}
			default:
				return `Invalid metadata name: ${key}`;
		}
	}
	return option;
};
