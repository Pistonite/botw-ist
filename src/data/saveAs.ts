import { saveAs as save } from "file-saver";

export const saveAs = (content: string, filename: string): void =>{
	const blob = new Blob([content], {
		type: "text/plain;charset=utf-8"
	});
	save(blob, filename);
};
