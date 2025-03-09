import { useState } from "react";

import { RuntimeInitArgs, RuntimeInitParams } from "@pistonite/skybook-api";

export type CustomImageSetupDialogProps = {
    /** Initial value of the use custom image by default checkbox */
    initialUseByDefault: boolean,
    /** Initial runtime parameters */
    initialParams: RuntimeInitParams,

    onOutput: (output: CustomImageSetupDialogOutput) => void,
}

export type CustomImageSetupDialogOutput = {
    type: "custom",
    params: RuntimeInitParams,
    buffer: Uint8Array,
    isUseByDefault: boolean,
} | {
    type: "default",
    deleteCustom: boolean,
}

export const CustomImageSetupDialog: React.FC<CustomImageSetupDialogProps> = ({
    initialUseByDefault,
    initialParams,
}) => {
    const [isUseByDefault, setIsUseByDefault] = useState(initialUseByDefault);
};
