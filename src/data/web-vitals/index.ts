/* eslint-disable no-console */
import { ReportHandler } from "web-vitals";
import reportWebVitals from "./reportWebVitals";

const RECOMMENDED = {
	"TTFB": [ "Time To First Byte", 200 /* milliseconds maybe */],
	"CLS": [ "Cumulative Layout Shift", 0.1],
	"LCP": [ "Largest Contentful Paint", 2500000 /* nanoseconds */],
	"FID": [ "First Input Delay", 100000 /* nanoseconds */],
	"FCP": [ "First Contentful Paint", 2500000 /* nanoseconds */],
};

const handleReport: ReportHandler = (report) => {
	const {name, value, delta} = report;
	console.log(`[Web Vitals] ${name} = ${value} (${delta>=0?"+":""}${delta})`);
	if (name in RECOMMENDED){
		const [fullName, max] = RECOMMENDED[name];
		if (value > max){
			console.warn(`[Web Vitals] ${fullName} exceeded recommended value of ${max}`);
			console.log(report);
		}
	}
};

export const reportWebVitalsAsync = async () => {
	reportWebVitals(handleReport);
};
