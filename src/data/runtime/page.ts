import { useState, useCallback, useEffect } from "react";

export const usePage = (): [string, (hash: string)=>void] => {
    const [page, setPage] = useState<string>("#simulation");
    // Function for consumers to set page
    const setRuntimePage = useCallback((hash: string)=>{
		window.location.hash = hash;
		setPage(hash);
	}, []);
    // Set page when url is edited
	useEffect(()=>{
		setPage(window.location.hash || "#simulation");
	}, [window.location.hash]);

    return [page, setRuntimePage];
}
