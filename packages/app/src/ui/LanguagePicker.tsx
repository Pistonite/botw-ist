import {
    Menu,
    MenuButton,
    MenuItemRadio,
    MenuList,
    MenuPopover,
    MenuTrigger,
} from "@fluentui/react-components";
import { Globe20Regular } from "@fluentui/react-icons";
import { getLocalizedLanguageName, getSupportedLocales } from "@pistonite/pure/pref";
import i18next from "i18next";
import { useLocale } from "@pistonite/pure-react";

import { SupportedLocales } from "skybook-localization";

export const LanguagePicker: React.FC = () => {
    const locale = useLocale();
    return (
        <Menu
            checkedValues={{ locale: [locale] }}
            onCheckedValueChange={async (_, { checkedItems }) => {
                await i18next.changeLanguage(checkedItems[0]);
            }}
        >
            <MenuTrigger disableButtonEnhancement>
                <MenuButton appearance="subtle" icon={<Globe20Regular />} />
            </MenuTrigger>
            <MenuPopover>
                <MenuList>
                    {SupportedLocales.map((lang) => (
                        <MenuItemRadio key={lang} name="locale" value={lang}>
                            {getLocalizedLanguageName(lang)}
                        </MenuItemRadio>
                    ))}
                </MenuList>
            </MenuPopover>
        </Menu>
    );
};
