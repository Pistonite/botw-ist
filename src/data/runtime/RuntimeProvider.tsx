import React, { PropsWithChildren, useCallback, useContext, useState } from "react";
import { SettingFunction, useStore } from "data/storage";

export type Runtime = {
    setting: SettingFunction,
    editing: boolean,
    saving: boolean,
    commandData: string[],
    setCommandData: (data: string[])=>void
    enableEditing: ()=>void,
    enableSaving: ()=>void,
};

const RuntimeContext = React.createContext<Runtime>({} as Runtime);

export const RuntimeProvider: React.FC<PropsWithChildren> = ({children}) => {
    const [editing, setEditing] = useState<boolean>(false);
    const [temporaryCommandData, setTemporaryCommandData] = useState<string[]|null>(null);
    const {setting, commandData, setCommandData} = useStore();

    // When temporary data are loaded, editing will be disabled at first
    // The user can choose to enable editing without saving
    // Or overwrite data in storage and enable editing and saving
    const enableEditing = useCallback(()=>setEditing(true), []);
    const saving = temporaryCommandData == null;
    const enableSaving = useCallback(()=>{
        if(!temporaryCommandData){
            return; // No temp data, saving already enabled
        }
        setEditing(true);
        setCommandData(temporaryCommandData);
        setTemporaryCommandData(null);
    }, [editing]);
    const runtimeCommandData = temporaryCommandData ?? commandData;
    const setRuntimeCommandData = useCallback((data: string[])=>{
        if(!editing){
            return; // editing temporary data is disabled
        }
        if(saving){   
            setCommandData(data);
        }else{
            setTemporaryCommandData(data);
        }
    }, [editing, saving]);

    return (
        <RuntimeContext.Provider value={{
            setting,
            editing,
            saving,
            commandData: runtimeCommandData,
            setCommandData: setRuntimeCommandData,
            enableEditing,
            enableSaving
        }}>
            {children}
        </RuntimeContext.Provider>
    );
}

export const useRuntime = ()=> useContext(RuntimeContext);
