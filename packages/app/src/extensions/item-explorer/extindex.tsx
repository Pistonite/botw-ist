import { useSyncExternalStore } from "react";
import { type Result, errstr } from "@pistonite/pure/result";
import { debounce } from "@pistonite/pure/sync";
import { type Cell, cell } from "@pistonite/pure/memory";
import type { WxPromise } from "@pistonite/workex";

import type { ActorSpriteProps } from "botw-item-assets";
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

    private iconSettings: Cell<Pick<ActorSpriteProps, "cheap" | "disableAnimation">>;

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
                    return {
                        err: translateGenericError(errstr(items.err)),
                    };
                }
                return items.val;
            },
            interval: 100,
        });

        this.iconSettings = cell({
            initial: {
                cheap: false,
                disableAnimation: false,
            }
        });

        const subscribe = (cb: (x: Pick<ActorSpriteProps, "cheap" | "disableAnimation">) => void) => {
            return this.iconSettings.subscribe(cb);
        };

        this.component = () => {
            // eslint-disable-next-line react-hooks/rules-of-hooks
            const { cheap, disableAnimation } = useSyncExternalStore(subscribe, () => {
                return this.iconSettings.get();
            })
            return <ItemExplorer searcher={this} cheap={cheap} disableAnimation={disableAnimation}/>;
        };
    }

    public get Component() {
        return this.component;
    }

  public override async onIconSettingsChanged(enableHighRes: boolean, enableAnimations: boolean): WxPromise<void> {
        this.iconSettings.set({
            cheap: !enableHighRes,
            disableAnimation: !enableAnimations,
        });
        return {};
      
  }
}
