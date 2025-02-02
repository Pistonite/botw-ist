import {
    Field,
    makeStyles,
    SearchBox,
    Body1,
    Checkbox,
} from "@fluentui/react-components";
import { useApplication } from "application/useApplication.ts";
import type { ExtensionComponentProps } from "../types.ts";
import { useDeferredValue, useEffect, useState } from "react";
import type { SearchResult as ItemSearchResult } from "skybook-localization";
import { translateUI, useUITranslation } from "skybook-localization";
import { FirstPartyExtensionAdapter } from "extensions/FirstPartyAdapter.ts";
import { debounce } from "@pistonite/pure/sync";
import type { Application } from "@pistonite/skybook-extension-api";
import type { Result } from "@pistonite/pure/result";
import { errstr } from "@pistonite/pure/result";
import { useQuery } from "@tanstack/react-query";
import {
    CookEffect,
    ItemSlot,
    ItemTooltip,
    makeItemSlotInfo,
} from "skybook-item-system";

type SearchResult = Omit<ItemSearchResult, "score">;

const search = debounce({
    fn: async (
        app: Application,
        localized: boolean,
        query: string,
    ): Promise<Result<SearchResult[], string>> => {
        if (!query) {
            return { val: [] };
        }
        if (query.startsWith("<") && query.endsWith(">")) {
            return {
                val: [
                    {
                        actor: query.slice(1, -1),
                        cookEffect: 0,
                    },
                ],
            };
        }
        const items = await app.resolveItem(query, localized, 0);
        if ("err" in items) {
            return {
                err: translateUI("generic.error.internal", {
                    error: errstr(items.err),
                }),
            };
        }
        return items.val;
    },
    interval: 100,
});

const useStyles = makeStyles({
    container: {
        padding: "8px",
        boxSizing: "border-box",
        height: "100%",
        display: "flex",
        flexDirection: "column",
    },
    results: {
        display: "flex",
        flexWrap: "wrap",
        maxHeight: 0,
        overflow: "visible",
    },
    resultsScroll: {
        marginTop: "8px",
        overflowY: "auto",
        flex: 1,
    },
    noResults: {
        display: "flex",
        flex: 1,
        alignItems: "center",
        justifyContent: "center",
    },
});

export const Component: React.FC<ExtensionComponentProps> = ({
    standalone,
    connect,
}) => {
    const app = useApplication();
    const [value, setValue] = useState("");
    const [localized, setLocalized] = useState(false);

    const deferredValue = useDeferredValue(value);

    const { data } = useQuery({
        queryKey: ["item-explorer-search", localized, deferredValue],
        queryFn: () => search(app, localized, deferredValue),
    });

    const error = data?.err;
    const results = data?.val;
    const hasResults = results !== undefined && results.length > 0;

    const styles = useStyles();
    const t = useUITranslation();

    useEffect(() => {
        return connect(new FirstPartyExtensionAdapter(standalone));
    }, [standalone, connect]);

    return (
        <div className={styles.container}>
            <Field
                validationState={error ? "error" : "none"}
                validationMessage={error}
            >
                <SearchBox
                    placeholder={t("item_explorer.label.search_placeholder")}
                    value={value}
                    onChange={(_, { value }) => {
                        setValue(value);
                    }}
                />
            </Field>
            <Field>
                <Checkbox
                    label={t("item_explorer.label.search_localized")}
                    checked={localized}
                    onChange={(_, { checked }) => {
                        setLocalized(!!checked);
                    }}
                />
            </Field>
            <Body1 block>
                {localized
                    ? t("item_explorer.desc.search_tip_localized")
                    : t("item_explorer.desc.search_tip_ident")}
            </Body1>
            {hasResults && (
                <div className={styles.resultsScroll}>
                    <div className={styles.results}>
                        {results.map((result, i) => {
                            const { actor, cookEffect } = result;
                            const info = makeItemSlotInfo(actor, {
                                modEffectId: cookEffect || CookEffect.None,
                            });
                            return (
                                <ItemTooltip info={info} key={i}>
                                    <ItemSlot info={info} />
                                </ItemTooltip>
                            );
                        })}
                    </div>
                </div>
            )}
            {!hasResults && (
                <div className={styles.noResults}>
                    <Body1>{t("item_explorer.label.no_results")}</Body1>
                </div>
            )}
        </div>
    );
};
