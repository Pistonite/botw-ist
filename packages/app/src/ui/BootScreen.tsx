import { useState } from "react";

export type BootScreenState =
"OpenSetupOrUseDefaultImage"
| "UseCustomOrUseDefaultImage"
| "SetupDialog"
| "InitializingCustom"
| "Error"
;

export type OpenSetupOrDefaultPromptType = 
"LocalVersionMismatch"
| "LocalNoImage"
| "DirectLoadVersionMismatch"
| "DirectLoadNoImage"
| "DirectLoadRequesting"
| "InitializeError"
;


export type BootScreenProps = {
    /** State of the boot flow when initially showing the screen */
    initialState: BootScreenState,
    /** If the initial state is OpenSetupOrUseDefaultImage, this is the prompt type */
    openSetupOrDefaultPromptType?: OpenSetupOrDefaultPromptType,
}
export const BootScreen: React.FC<BootScreenProps> = ({initialState, openSetupOrDefaultPromptType}) => {
    const [machineState, setMachineState] = useState(initialState);
    const [promptType, setPromptType] = useState<OpenSetupOrDefaultPromptType>(openSetupOrDefaultPromptType || "LocalNoImage");
};
