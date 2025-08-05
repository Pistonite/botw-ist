import { Text, Button, Tooltip } from "@fluentui/react-components";
import {
    Beach20Filled,
    Beach20Regular,
    BoxMultipleArrowRight20Filled,
    HandMultiple20Filled,
} from "@fluentui/react-icons";

import { useUITranslation } from "skybook-localization";

/** Icon to indicate holding state in inventory */
export const HoldingIcon: React.FC = () => {
    const t = useUITranslation();
    return (
        <Tooltip
            relationship="label"
            content={
                <>
                    <Text weight="bold" block size={200}>
                        {t(`main.visible_inventory.holding.title`)}
                    </Text>
                    {t(`main.visible_inventory.holding.desc`)}
                </>
            }
            withArrow
            positioning="below"
        >
            <Button
                style={{
                    color: "#ffee00",
                    borderColor: "#ffee00",
                }}
                shape="circular"
                icon={<HandMultiple20Filled />}
            />
        </Tooltip>
    );
};

/** Icon to indicate arrowless smuggle state */
export const ArrowlessSmuggleIcon: React.FC = () => {
    const t = useUITranslation();
    return (
        <Tooltip
            relationship="label"
            content={
                <>
                    <Text weight="bold" block size={200}>
                        {t(`main.visible_inventory.arrowless_smuggle.title`)}
                    </Text>
                    {t(`main.visible_inventory.arrowless_smuggle.desc`)}
                </>
            }
            withArrow
            positioning="below"
        >
            <Button
                style={{
                    color: "#ffee00",
                    borderColor: "#ffee00",
                }}
                shape="circular"
                icon={<BoxMultipleArrowRight20Filled />}
            />
        </Tooltip>
    );
};

/** Icon to indicate trial mode*/
export const TrialModeIcon: React.FC = () => {
    const t = useUITranslation();
    return (
        <Tooltip
            relationship="label"
            content={
                <>
                    <Text weight="bold" block size={200}>
                        {t(`main.visible_inventory.trial.title`)}
                    </Text>
                    {t(`main.visible_inventory.trial.desc`)}
                </>
            }
            withArrow
            positioning="below"
        >
            <Button
                shape="circular"
                icon={<Beach20Regular />}
            />
        </Tooltip>
    );
};
