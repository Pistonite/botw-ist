import { type Result, errstr } from "@pistonite/pure/result";
import { debounce } from "@pistonite/pure/sync";

import {
    type SearchResultNoScore,
    translateGenericError,
} from "skybook-localization";

import {
    FirstPartyExtensionAdapter,
    type FirstPartyExtension,
} from "../FirstParty.ts";

import { ItemExplorer, type Searcher } from "./ItemExplorer.tsx";

export class ItemExplorerExtension
    extends FirstPartyExtensionAdapter
    implements FirstPartyExtension, Searcher
{
    public search: (
        localized: boolean,
        query: string,
    ) => Promise<Result<SearchResultNoScore[], string>>;
    private component: React.FC;

    constructor(standalone: boolean) {
        super(standalone);
        this.search = debounce({
            fn: async (
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
                const app = this.app;
                if (!app) {
                    return { val: [] };
                }
                const items = await app.resolveItem(query, localized, 0);
                if ("err" in items) {
                    console.log(items.err);
                    return {
                        err: translateGenericError(errstr(items.err)),
                    };
                }
                return items.val;
            },
            interval: 100,
        });

        this.component = () => {
            return <ItemExplorer searcher={this} />;
        };
    }

    public get Component() {
        return this.component;
    }
}
