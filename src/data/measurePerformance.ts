const PRECISION = 4; // 4 sig-figs
type AnyFunction = <T>(...args: T[])=>void;
export const measurePerformance = performance ? (tag: string, func: AnyFunction) => {
	const start = performance.now();
	func();
	const duration = performance.now() - start;
	console.log(`${tag}${duration.toPrecision(PRECISION)}ms`); // eslint-disable-line no-console
} : (_tag: string, func: AnyFunction)=>func();
