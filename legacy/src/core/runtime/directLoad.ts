import queryString from "query-string";
import { useEffect } from "react";
import { deserialize } from "data/storage";

export const useDirectLoader = (
    setTemporaryCommandData: (data: string[] | null) => void,
    setEditing: (editing: boolean) => void,
) => {
    useEffect(() => {
        const query = queryString.parse(window.location.search);
        try {
            const commandTextToLoad = deserialize(query);

            if (commandTextToLoad) {
                const commandDataToLoad = commandTextToLoad.split("\n");
                setTemporaryCommandData(commandDataToLoad);
                setEditing(false);
                return;
            }

            // If no temporary data is found, Allow editing & saving
            setTemporaryCommandData(null);
            setEditing(true);
        } catch (e) {
            throw new Error(`deserialize direct url: ${e || "unknown error"}`);
        }
    }, [window.location.search]);
};
