import { Dropdown, makeStyles, Option } from "@fluentui/react-components";
import { useQuery } from "@tanstack/react-query";

import { useUITranslation } from "skybook-localization";

import { getExtensionComponent } from "./registry";
import { connectExtensionToApp } from "application/extensionManager";

export type ExtensionWindowProps = {
    primary: boolean;
    ids: string[];
    currentId: string;
    onSelect: (id: string) => void;
}

const useStyles = makeStyles({
    container: {
        display: "flex",
        flexDirection: "column",
        height: "100%",
    },
    extension: {
        flex: 1,
    }
});


export type ExtensionWrapperProps = {
    id: string;
}

export const ExtensionWrapper: React.FC<ExtensionWrapperProps> = ({ id }) => {
    const {isPending, data: ExtComp } = useQuery({
        queryKey: ["extension", id],
        queryFn: () => {
            return getExtensionComponent(id);
        },
    });
    if (isPending || !ExtComp) {
        return <div>Loading...</div>
    }
    return <ExtComp 
        standalone={false} 
        connect={connectExtensionToApp}
         />;
}
