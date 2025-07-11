import { Text, Button, Tooltip } from "@fluentui/react-components";
import {
    DesktopOff20Regular,
    PersonRunning20Regular,
    ShoppingBagPause20Regular,
    ShoppingBag20Regular,
} from "@fluentui/react-icons";

import type { InvView_Screen } from "@pistonite/skybook-api";
import { useUITranslation } from "skybook-localization";

import { useUIStore } from "self::application";
import { getRandomBackgroundName } from "self::util";

export type ScreenIndicatorProps = {
    screen?: InvView_Screen;
    hasGlider: boolean;
};

export const ScreenIndicator: React.FC<ScreenIndicatorProps> = ({
    screen,
    hasGlider,
}) => {
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
    const background = useUIStore((state) => state.background);
    const setBackground = useUIStore((state) => state.setBackgroundName);

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
                        ? "secondary"
                        : "transparent"
                }
                onClick={() => {
                    if (screen !== "overworld") {
                        return;
                    }
                    const newBg = getRandomBackgroundName(
                        background,
                        hasGlider,
                    );
                    setBackground(newBg);
                }}
            />
        </Tooltip>
    );
};
