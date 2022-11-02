import { CrashScreen } from "components/CrashScreen";
import { measurePerformance } from "data/measurePerformance";
import { LoadingScreen } from "components/LoadingScreen";
import React, { PropsWithChildren, useCallback, useContext, useEffect, useState } from "react";

/*
 * Provide the language file to component tree
 * For now, it loads en.lang.yaml statically and provides it. If we continue to support more languages, the language detection and lazy-loading will happen here
 */

type TranslateFunction = (key: string)=>string;
type FlatLangMap = { [flatKey: string]: string};

const LanguageContext = React.createContext<TranslateFunction>(x=>x);

const UnlocalizedSet = new Set();
export const useI18n = () => useContext(LanguageContext);
export const LanguageProvider: React.FC<PropsWithChildren> = ({children}) => {
	const [error, setError] = useState<boolean>(false);
	const [flatLangMap, setFlatLangMap] = useState<FlatLangMap|null>(null);

	useEffect(()=>{
		const loadFuncAync = async () => {
			try{
				setFlatLangMap(await loadFlatLangMapAsync());
			}catch(e){
				console.error(e);
				setError(true);
				setFlatLangMap(null);
			}
		};
		measurePerformance("Load Language: ", ()=>{
			loadFuncAync();
		});

	},[]);

	const translationFunction = useCallback((key: string)=>{
		if(!flatLangMap){
			throw new Error("Translation function should not be supplied before lang map is loaded");
		}
		if(!(key in flatLangMap)){
			if(!UnlocalizedSet.has(key)){
				// in the future, need to add fallback logic (i.e. on the spot load the default lang file and return new translation later)
				console.warn(`Unlocalized: ${key}`); // eslint-disable-line no-console
				UnlocalizedSet.add(key);
			}

			return key;
		}
		return flatLangMap[key];
	}, [flatLangMap]);

	if(!flatLangMap){
		if(error){
			return <CrashScreen>An error has occured while loading language</CrashScreen>;
		}else{
			return <LoadingScreen>Loading language...</LoadingScreen>;
		}
	}

	return (
		<LanguageContext.Provider value={translationFunction}>
			{children}
		</LanguageContext.Provider>
	);

};

type StringTree = typeof import("*.lang.yaml").default;

const loadFlatLangMapAsync = async ():Promise<FlatLangMap> => {
	// In the future, add language detection here
	const langMapModule = await import("./en.lang.yaml");
	const langMap = langMapModule["default"];
	// Flatten the lang map
	const flatMap: FlatLangMap = {};
	for(const rootKey in langMap){
		flattenInTo(rootKey, langMap[rootKey], flatMap);
	}

	return flatMap;
};

const flattenInTo = (key: string, obj: StringTree | string, outMap: FlatLangMap) => {
	if(typeof obj === "string"){
		outMap[key] = obj;
		return;
	}
	for(const nextKey in obj){
		flattenInTo(key+"."+nextKey, obj[nextKey], outMap);
	}
};
