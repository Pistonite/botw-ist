import { gzip, ungzip } from "pako";

const ZLIB_OPTIONS = {
	level: 9
} as const;

type SerializedCommands = { r: string } | { c: string }; // r for raw and c for compressed

export const serialize = (commandsString: string): SerializedCommands => {
	const compressed = compressString(commandsString);
	if (commandsString.length < compressed.length){
		return { r: commandsString };
	}
	return { c: compressed };
};

export const deserialize = (serializedCommands: Partial<SerializedCommands>): string | null => {
	if ( "r" in serializedCommands && serializedCommands.r){
		return serializedCommands.r;
	}
	if ( "c" in serializedCommands && serializedCommands.c){
		return decompressString(serializedCommands.c);
	}
	return null;
};

export const compressString = (uncompressedString: string): string => {
	const uncompressedBytes = Buffer.from(uncompressedString, "utf8");
	const compressedBytes = gzip(uncompressedBytes, ZLIB_OPTIONS);
	return Buffer.from(compressedBytes).toString("base64");
};

export const decompressString = (decompressedString: string): string => {
	const compressedBytes = Buffer.from(decompressedString, "base64");
	const uncompressedBytes = ungzip(compressedBytes, {
		to: "string",
		...ZLIB_OPTIONS
	});
	return Buffer.from(uncompressedBytes).toString("utf8");
};
