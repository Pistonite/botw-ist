import type { ExtensionApp } from "@pistonite/skybook-api";
import { type Result, errstr } from "@pistonite/pure/result";
import { debounce } from "@pistonite/pure/sync";

import { type SearchResultNoScore, translateUI } from "skybook-localization";

import {
    FirstPartyExtensionAdapter,
    type FirstPartyExtension,
} from "../FirstParty.ts";

import { ItemExplorer } from "./ItemExplorer.tsx";

export class ItemExplorerExtension
    extends FirstPartyExtensionAdapter
    implements FirstPartyExtension
{
    private doSearch: (
        app: ExtensionApp,
        localized: boolean,
        query: string,
    ) => Promise<Result<SearchResultNoScore[], string>>;
    private component: React.FC;

    constructor(standalone: boolean) {
        super(standalone);
        this.doSearch = debounce({
            fn: async (
                app: ExtensionApp,
                localized: boolean,
                query: string,
            ): Promise<Result<SearchResultNoScore[], string>> => {
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

        this.component = () => {
            return <ItemExplorer extension={this} />;
        };
    }

    public get Component() {
        return this.component;
    }

    public async search(
        localized: boolean,
        query: string,
    ): Promise<Result<SearchResultNoScore[], string>> {
        if (!this.app) {
            return { val: [] };
        }
        return await this.doSearch(this.app, localized, query);
    }
}
