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

export const ExtensionWindow: React.FC<ExtensionWindowProps> = ({ primary, ids, currentId, onSelect }) => {
    const t = useUITranslation();
    const styles = useStyles();
    return (
        <div className={styles.container}>
        <Dropdown
            value={t(`extension.${currentId}.name`)}
            selectedOptions={[currentId]}
            onOptionSelect={(_, {optionValue}) => {
                console.log(optionValue)
            }}
        >
            {
                ids.map((id, i) => (
                <Option key={i} value={id}>{t(`extension.${id}.name`)}</Option>
                ))
            }
        </Dropdown>
        {
            ids.map((id, i) => (
            <div key={i} data-extension-id={id} className={styles.extension} style={{
                        display: id === currentId ? "block" : "none",
                    }}>
                <ExtensionWrapper id={id} />
            </div>
            ))
        }
        </div>
    );

}


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
