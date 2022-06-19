import FileSaverFunction from "./FileSaver";

export const saveAs = (content: string, filename: string): void =>{
	const blob = new Blob([content], {
		type: "text/plain;charset=utf-8"
	});
	FileSaverFunction(blob, filename);
};
