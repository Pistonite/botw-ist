import { deserialize } from "data/serialize";
import { parse } from "query-string";
import { useEffect } from "react";

export const useDirectLoader = (setTemporaryCommandData: (data: string[]|null)=>void, setEditing: (editing: boolean)=>void) => {
    useEffect(()=>{
        const query = parse(window.location.search);
        try {
            const commandTextToLoad = deserialize(query);

            if(commandTextToLoad){
                const commandDataToLoad = commandTextToLoad.split("\n");
                setTemporaryCommandData(commandDataToLoad);
                setEditing(false);
                return;
            }

            // If no temporary data is found, Allow editing & saving
            setTemporaryCommandData(null);
            setEditing(true);
            
        } catch(e: any){
            throw(new Error(`deserialize direct url: ${e || "unknown error"}`))
        }
        
    }, [window.location.search]);
}
