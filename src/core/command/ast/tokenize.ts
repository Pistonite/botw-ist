
export type Token = {
	readonly value: string,
	readonly start: number, //inclusive
	readonly end: number //exclusive
};
// Very basic tokenizer. It separates the string into single-character special symbols and whatever is in between
export const tokenize = (str: string, regex: RegExp): string[] => {
	return tokenizeCore(str, regex).map(({value})=>value);
};

const tokenizeCore = (str: string, regex: RegExp): Token[] => {
	const tokens: Token[] = [];
	let j = str.search(regex);
	let start = 0;
	while(j !== -1){
		if(j!==0){
			//Prevent empty tokens
			tokens.push({
				value: str.substring(0, j),
				start,
				end: start+j
			});
		}

		tokens.push({
			value: str[j],
			start: start+j,
			end: start+j+1
		});
		str = str.substring(j+1);
		start+=j+1;
		j = str.search(regex);
	}
	if(str !== ""){
		tokens.push({
			value: str,
			start,
			end: start+str.length
		});
	}
	return tokens.filter(s=>!s.value.match(/^\s*$/));
}

export const tokenizeV2 = (str: string, regex: RegExp): TokenStream => {
	return new TokenStreamImpl(tokenizeCore(str.toLowerCase(), regex));
}

export interface TokenStream {
	// Push the current position to stack so it can be restored
	push: ()=>void,
	// pop position stack
	pop: ()=>void,
	// restore top of position stack, does not pop
	restore: ()=> void,
	// Consume the next token,
	consume: (out?: Token[])=>string | undefined
};

class TokenStreamImpl implements TokenStream {
	data: Token[];
	index: number;
	stack: number[]

	constructor(data: Token[]){
		this.data = data;
		this.index = 0;
		this.stack = [];
	}

	consume(out?: Token[]) {
		const token = this.data[this.index];
		if((window as any).__debug){
			console.log(token);
		}
		if(!token){
			return undefined;
		}
		if (out){
			out.push(token);
		}
		this.index++;
		return token.value;
	}

	push() {
		this.stack.push(this.index);
	}

	restore() {
		if(this.stack.length === 0){
			throw new Error("Token stream stack underflow");
		}
		this.index = this.stack[this.stack.length-1];
	}

	pop() {
		this.stack.pop();
	}
}
