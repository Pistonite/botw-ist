import { useEffect, useState } from "react";

export const useWarnReadOnly = () => {
    const state = useState(false);
    const [warnReadOnly, setWarnReadOnly] = state;
    useEffect(()=>{
        if (warnReadOnly){
            const handle = setTimeout(()=>setWarnReadOnly(false), 100);
            return ()=>clearTimeout(handle);
        }
    }, [warnReadOnly]);
    return state;
};
