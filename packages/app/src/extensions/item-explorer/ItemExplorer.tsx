import {
    Field,
    makeStyles,
    SearchBox,
    Body1,
    Checkbox,
} from "@fluentui/react-components";
import { useQuery } from "@tanstack/react-query";
import type { Result } from "@pistonite/pure/result";
import { useDeferredValue, useState } from "react";

import { type SearchResultNoScore, useUITranslation } from "skybook-localization";
import {
    CookEffect,
    ItemSlot,
    ItemTooltip,
    makeItemSlotInfo,
} from "skybook-item-system";

export type Searcher = {
    search(localized: boolean, query: string): Promise<Result<SearchResultNoScore[], string>>;
};

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

export const ItemExplorer: React.FC<{ searcher: Searcher }> = ({
    searcher,
}) => {
    const [value, setValue] = useState("");
    const [localized, setLocalized] = useState(false);

    const deferredValue = useDeferredValue(value);

    const { data } = useQuery({
        queryKey: ["item-explorer-search", localized, deferredValue],
        queryFn: () => searcher.search(localized, deferredValue),
    });

    const error = data?.err;
    const results = data?.val;
    const hasResults = results !== undefined && results.length > 0;

    const styles = useStyles();
    const t = useUITranslation();

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
