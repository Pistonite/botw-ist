import { createContext, useContext } from "react";
import { Application } from "@pistonite/skybook-extension-api";

export const ApplicationContext = createContext<Application>({} as unknown as Application);

export const useApplication = () => {
    return useContext(ApplicationContext);
}
