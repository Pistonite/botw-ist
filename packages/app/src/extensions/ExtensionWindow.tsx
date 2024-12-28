import { Dropdown, Option } from "@fluentui/react-components";
import { useQuery } from "@tanstack/react-query";

import { useUITranslation } from "skybook-localization";

import { getExtensionComponent } from "./registry";

export type ExtensionWindowProps = {
    primary: boolean;
    ids: string[];
    currentId: string;
    onSelect: (id: string) => void;
}

export const ExtensionWindow: React.FC<ExtensionWindowProps> = ({ primary: isPrimary, ids, currentId, onSelect }) => {
    const t = useUITranslation();
    return (
        <div>
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
            <div key={i} data-extension-id={id} style={{
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
    return <ExtComp />;
}
