export const tokenize = (str: string, regex: RegExp): string[] => {
	const tokens: string[] = [];
	let j = str.search(regex);
	while(j !== -1){
		if(j!==0){
			//Prevent empty tokens
			tokens.push(str.substring(0, j));
		}

		tokens.push(str[j]);
		str = str.substring(j+1);
		j = str.search(regex);
	}
	if(str !== ""){
		tokens.push(str);
	}
	return tokens;
};
