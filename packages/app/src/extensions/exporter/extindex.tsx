import {
    extLog,
    type FirstPartyExtension,
    FirstPartyExtensionAdapter,
} from "self::util";

import { Exporter } from "./exporter.tsx";
import { serial } from "@pistonite/pure/sync";

export class ExporterExtension
    extends FirstPartyExtensionAdapter
    implements FirstPartyExtension
{
    private component: React.FC;
    constructor(standalone: boolean) {
        super(standalone);
        this.component = () => {
            return (
                <Exporter
                    getScript={() => this.getScriptFromApp()}
                    getDirectUrl={async () => {
                        const url = await this.getDirectUrl();
                        if (url.err) {
                            return undefined;
                        }
                        return url.val;
                    }}
                />
            );
        };
    }
    public get Component() {
        return this.component;
    }

    private async getScriptFromApp(): Promise<string | undefined> {
        const app = this.app;
        if (!app) {
            return undefined;
        }
        const script = await app.getScript();
        if (script.err) {
            extLog.error("failed to get script from app to export");
            extLog.error(script.err);
            return undefined;
        }
        return script.val;
    }

    private getDirectUrl = serial({
        fn: (checkCancel) => async () => {
            const script = await this.getScriptFromApp();
            if (script === undefined) {
                return undefined;
            }
            let response: Response;
            try {
                response = await fetch("/api/encode", {
                    method: "POST",
                    body: script,
                });
            } catch (e) {
                extLog.error("failed to encode the script from server");
                extLog.error(e);
                return undefined;
            }
            checkCancel();
            if (!response.ok || response.status !== 200) {
                extLog.error(
                    `failed to encode the script from server: server responded with status ${response.status}`,
                );
                return undefined;
            }
            try {
                const encoded = await response.text();
                return window.location.origin + "?v4=" + encoded;
            } catch (e) {
                extLog.error("failed to encode the script from server");
                extLog.error(e);
                return undefined;
            }
        },
    });
}
