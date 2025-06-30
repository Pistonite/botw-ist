import { Text, Button, Tooltip } from "@fluentui/react-components";
import {
    DesktopOff20Regular,
    PersonRunning20Regular,
    ShoppingBagPause20Regular,
    ShoppingBag20Regular,
} from "@fluentui/react-icons";

import type { InvView_Screen } from "@pistonite/skybook-api";
import { useUITranslation } from "skybook-localization";

export type ScreenIndicatorProps = {
    screen?: InvView_Screen;
};

export const ScreenIndicator: React.FC<ScreenIndicatorProps> = ({ screen }) => {
    const screenReal = screen || "none";
    let icon;
    switch (screenReal) {
        case "overworld": {
            icon = <PersonRunning20Regular />;
            break;
        }
        case "inventory": {
            icon = <ShoppingBagPause20Regular />;
            break;
        }
        case "shop": {
            icon = <ShoppingBag20Regular />;
            break;
        }
        default: {
            icon = <DesktopOff20Regular />;
        }
    }

    const t = useUITranslation();
    return (
        <Tooltip
            content={
                <>
                    <Text weight="bold" block size={200}>
                        {t(`main.screen.${screenReal}`)}
                    </Text>
                    {t(`main.screen.${screenReal}.desc`)}
                </>
            }
            relationship="label"
            withArrow
            positioning="below"
        >
            <Button
                icon={icon}
                appearance={
                    screenReal !== "none" && screenReal !== "overworld"
                        ? "primary"
                        : "transparent"
                }
            />
        </Tooltip>
    );
};
