import React, { PropsWithChildren, useCallback, useContext, useState } from "react";
import { SettingFunction, useStore } from "data/storage";
import { GetSetPair } from "data/types";
import { useWarnOnClose } from "./warnOnClose";
import { usePage } from "./page";
import { useDirectLoader } from "./directLoad";
import { useWarnReadOnly } from "./warnReadOnly";

export type Runtime = {
    setting: SettingFunction,
    editing: boolean,
    enableEditing: ()=>void
}
& GetSetPair<"saving", boolean>
& GetSetPair<"commandData", string[]>
& GetSetPair<"page", string>
& GetSetPair<"warnReadOnly", boolean>;

const RuntimeContext = React.createContext<Runtime>({} as Runtime);

export const RuntimeProvider: React.FC<PropsWithChildren> = ({children}) => {
	const [editing, setEditing] = useState<boolean>(false);
	const [temporaryCommandData, setTemporaryCommandData] = useState<string[]|null>(null);
	const {setting, commandData, setCommandData} = useStore();

	// Warn user if they are doing an editing action in read only mode
	const [warnReadOnly, setWarnReadOnly] = useWarnReadOnly();

	// Attempt to load temporary data whenever query string changes
	useDirectLoader(setTemporaryCommandData, setEditing);
	// When temporary data are loaded, editing will be disabled at first
	// The user can choose to enable editing without saving
	// Or overwrite data in storage and enable editing and saving
	const enableEditing = useCallback(()=>setEditing(true), []);
	const saving = temporaryCommandData == null;
	const setSaving = useCallback((enable: boolean)=>{
		if(enable){
			if(!temporaryCommandData){
				return; // No temp data, saving already enabled
			}
			setEditing(true);
			setCommandData(temporaryCommandData);
			setTemporaryCommandData(null);
		}else{
			setTemporaryCommandData(commandData);
		}
	}, [temporaryCommandData, commandData]);
	const runtimeCommandData = temporaryCommandData ?? commandData;
	const setRuntimeCommandData = useCallback((data: string[])=>{
		if(!editing){
			setWarnReadOnly(true);
			return; // editing temporary data is disabled
		}
		if(saving){
			setCommandData(data);
		}else{
			setTemporaryCommandData(data);
		}
	}, [editing, saving]);

	// Warn user if they close the window with unsaved data
	useWarnOnClose(editing, temporaryCommandData, commandData);
	// Detect page
	const [page, setPage] = usePage();

	return (
		<RuntimeContext.Provider value={{
			setting,
			editing,
			saving,
			commandData: runtimeCommandData,
			setCommandData: setRuntimeCommandData,
			enableEditing,
			setSaving,
			page,
			setPage,
			warnReadOnly,
			setWarnReadOnly
		}}>
			{children}
		</RuntimeContext.Provider>
	);
};

export const useRuntime = ()=> useContext(RuntimeContext);
