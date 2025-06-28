import { useQuery } from "@tanstack/react-query";

import { connectLocalExtensionToApp } from "self::application/extension";
import { getExtension } from "self::extensions";
import { useStyleEngine } from "self::ui/functions";

export type ExtensionWindowProps = {
    /** Ids of the extensions loaded in this window */
    ids: string[];

    /** Id of the currently displayed extension */
    currentId: string;
};

export const ExtensionWindow: React.FC<ExtensionWindowProps> = ({
    ids,
    currentId,
}) => {
    const m = useStyleEngine();
    return (
        <div className={m("flex-1 wh-100 min-h-0")}>
            {ids.map((id, i) => (
                <div
                    key={i}
                    data-extension-id={id}
                    className={m("wh-100")}
                    style={{
                        display: id === currentId ? "block" : "none",
                    }}
                >
                    <ExtensionWrapper id={id} />
                </div>
            ))}
        </div>
    );
};

export type ExtensionWrapperProps = {
    id: string;
};

/**
 * Component that wraps and loads an extension component with the given id.
 */
export const ExtensionWrapper: React.FC<ExtensionWrapperProps> = ({ id }) => {
    const { isPending, data: ExtComp } = useQuery({
        queryKey: ["extension", id],
        queryFn: async () => {
            const extension = await getExtension(
                id,
                false,
                connectLocalExtensionToApp,
            );
            return extension?.Component;
        },
    });
    if (isPending || !ExtComp) {
        // TODO: better loading UI
        return <div>Loading...</div>;
    }
    return <ExtComp />;
};
