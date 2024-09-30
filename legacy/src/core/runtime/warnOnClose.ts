import { useEffect } from "react";

export const useWarnOnClose = (
    editing: boolean,
    temporaryCommandData: string[] | null,
    commandData: string[],
) => {
    useEffect(() => {
        if (editing && temporaryCommandData) {
            if (temporaryCommandData.join("\n") !== commandData.join("\n")) {
                window.onbeforeunload = () =>
                    "You will lose unsaved data if you close now. Are you sure?";
                return;
            }
        }
        window.onbeforeunload = () => undefined;
    }, [editing, temporaryCommandData]);
};
