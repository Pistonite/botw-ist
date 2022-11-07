//https://medium.com/@fsufitch/is-javascript-array-sort-stable-46b90822543f
export const stableSort = <T>(array: T[], cmp: (a:T, b:T) => number): void => {
	const stabilizedThis: [T, number][] = array.map((el, index) => [el, index]);
	const stableCmp = (a: [T, number], b: [T, number]) => {
		const order = cmp(a[0], b[0]);
		if (order != 0) {return order;}
		return a[1] - b[1];
	};
	stabilizedThis.sort(stableCmp);

	for (let i=0; i<array.length; i++) {
		array[i] = stabilizedThis[i][0];
	}
};

interface Equalable<A> {
	equals(a: A): boolean;
}

// Compare 2 arrays by invoking B's equals method using A as input
export const arrayEqual = <A, B extends Equalable<A>>(arrayA: A[], arrayB: B[]): boolean => {
	if(arrayA === arrayB as any){
		return true;
	}
	if(arrayA.length !== arrayB.length){
		return false;
	}
	for (let i=0;i<arrayA.length;i++){
		if(!arrayB[i].equals(arrayA[i])){
			return false;
		}
	}
	return true;
}

export const arrayShallowEqual = <A>(arrayA: A[], arrayB: A[]): boolean => {
	if(arrayA === arrayB as any){
		return true;
	}
	if(arrayA.length !== arrayB.length){
		return false;
	}
	for (let i=0;i<arrayA.length;i++){
		if(arrayB[i] !== arrayA[i]){
			return false;
		}
	}
	return true;
}
