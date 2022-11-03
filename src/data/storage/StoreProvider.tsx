import React, { PropsWithChildren, useCallback, useContext, useEffect, useState } from "react";
import { produce } from "immer";

type StoreSetting = {
    showGameData: boolean,
    showSaves: boolean,
    interlaceGameData: boolean,
    showSuperCommand: boolean,
    animatedIcon: boolean,
};

const loadOrDefault = function <T>(key: string, defaultValue: T) {
    const jsonString = localStorage.getItem(key);
    if(!jsonString){
        return defaultValue;
    }
    try{
        return JSON.parse(jsonString)
    }catch(e){
        console.error(e)
        return defaultValue;
    }
}

const store = function <T>(key: string, value: T) {
    localStorage.setItem(key, JSON.stringify(value));
}

const DefaultSetting: StoreSetting = {
    showGameData: true,
    showSaves: false,
    interlaceGameData: false,
    showSuperCommand: false,
    animatedIcon: true,
}

export type SettingFunction = <T extends keyof StoreSetting>(key: T, value?: StoreSetting[T]) => StoreSetting[T];

export type Store = {
    setting: SettingFunction,
    commandData: string[],
    setCommandData: (data: string[]) => void,
};

const StoreContext = React.createContext<Store>({} as Store);

// The old command key
const LegacyCommandKey = "HDS.CurrentCommandsText";
const KEY_COMMAND_DATA = "botw-ist-data";
const KEY_SETTING = "botw-ist-setting";
export const StoreProvider: React.FC<PropsWithChildren> = ({children}) => {
    const [settingState, setSettingState] = useState<StoreSetting>(loadOrDefault(KEY_SETTING, DefaultSetting));

    const settingFunction: SettingFunction = useCallback((key, value)=>{
        if (value !== undefined){
            const newState = produce(settingState, newState => {
                newState[key] = value;
            })
            setSettingState(newState);
        }
        return settingState[key];
    }, [settingState]);

    useEffect(()=>{
        store(KEY_SETTING, settingState);
    }, [settingState]);

    const [commandData, setCommandData] = useState<string[]>(loadOrDefault(KEY_COMMAND_DATA, []));
    // Remove legacy data and convert it to new form
    useEffect(()=>{
        const legacyData = localStorage.load(LegacyCommandKey);
        if (legacyData){
            const commands = legacyData.split("\n");
            setCommandData(commands);
        }
        localStorage.removeItem(LegacyCommandKey);
    }, []);

    return (
        <StoreContext.Provider value={{
            setting: settingFunction,
            commandData,
            setCommandData
        }}>
            {children}
        </StoreContext.Provider>
    );
}

export const useStore = () => useContext(StoreContext);
