import { useState } from "react";
import {
    Field,
    makeStyles,
    SearchBox,
    Body1,
    Checkbox,
} from "@fluentui/react-components";
import { useQuery } from "@tanstack/react-query";
import type { Result } from "@pistonite/pure/result";
import { useDebounce } from "@uidotdev/usehooks";

import {
    type SearchResultNoScore,
    useUITranslation,
} from "skybook-localization";
import {
    type CookEffect,
    StandaloneItemSlotWithTooltip,
} from "skybook-item-system";

import { Code, Interpolate } from "self::ui/components";
import { useStyleEngine } from "self::util";

export type Searcher = {
    search(
        localized: boolean,
        query: string,
    ): Promise<Result<SearchResultNoScore[], string>>;
};

const useStyles = makeStyles({
    container: {
        padding: "8px",
    },
    resultsContainer: {
        paddingTop: "10px",
    },
    resultsScroll: {
        marginTop: "8px",
    },
});

type ItemExplorerProps = {
    searcher: Searcher;
    cheap?: boolean;
    disableAnimation?: boolean;
};

export const ItemExplorer: React.FC<ItemExplorerProps> = ({
    searcher,
    cheap,
    disableAnimation,
}) => {
    const [value, setValue] = useState("");
    const [localized, setLocalized] = useState(false);

    const deferredValue = useDebounce(value, 200);

    const { data } = useQuery({
        queryKey: ["item-explorer-search", localized, deferredValue],
        queryFn: () => searcher.search(localized, deferredValue),
    });

    const error = data?.err;
    const results = data?.val;
    const hasResults = results !== undefined && results.length > 0;

    const m = useStyleEngine();
    const c = useStyles();
    const t = useUITranslation();

    const $SearchBox = (
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
    );
    const $LocalizedCheckbox = (
        <Field>
            <Checkbox
                label={t("item_explorer.label.search_localized")}
                checked={localized}
                onChange={(_, { checked }) => {
                    setLocalized(!!checked);
                }}
            />
        </Field>
    );

    const $SearchTip = localized ? (
        <Interpolate
            quote={<Code>{`"`}</Code>}
            code_example={<Code>{"fr:espadon royal"}</Code>}
        >
            {t("item_explorer.desc.search_tip_localized")}
        </Interpolate>
    ) : (
        <Interpolate
            hyphen={<Code>{"-"}</Code>}
            example={<Code>{"royal-claymore"}</Code>}
        >
            {t("item_explorer.desc.search_tip_ident")}
        </Interpolate>
    );

    const $Results = hasResults && (
        <div className={m("overflow-y-auto flex-1", c.resultsScroll)}>
            <div
                className={m(
                    "flex flex-wrap max-h-0 overflow-visible",
                    c.resultsContainer,
                )}
            >
                {results.map(({ actor, cookEffect }, i) => (
                    <StandaloneItemSlotWithTooltip
                        key={i}
                        actor={actor}
                        effect={cookEffect as CookEffect}
                        cheap={cheap}
                        disableAnimation={disableAnimation}
                    />
                ))}
            </div>
        </div>
    );

    return (
        <div className={m("flex-col h-100 border-box", c.container)}>
            {$SearchBox}
            {$LocalizedCheckbox}
            <Body1 block>{$SearchTip}</Body1>
            {$Results}
            {!hasResults && !!value && (
                <div className={m("flex flex-1 flex-center")}>
                    <Body1>{t("item_explorer.label.no_results")}</Body1>
                </div>
            )}
        </div>
    );
};
