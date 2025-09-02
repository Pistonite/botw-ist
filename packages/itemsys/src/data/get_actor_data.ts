import { ActorDataMap } from "../generated/actor_data_map.ts";
import { DefaultActorData, type ActorData } from "./default_actor_data.ts";

/** Get the data property of the actor, or default if the actor doesn't have the property */
export const getActorParam = <K extends keyof ActorData>(actor: string, key: K): ActorData[K] => {
    const data = ActorDataMap[actor];
    if (!data || !(key in data)) {
        return DefaultActorData[key];
    }
    return (data as ActorData)[key];
};

/** Check if the actor has the property */
export const hasActorParam = <K extends keyof ActorData>(actor: string, key: K): boolean => {
    return key in ActorDataMap[actor];
};
