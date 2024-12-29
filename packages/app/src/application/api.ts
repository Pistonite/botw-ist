import { Application } from "@pistonite/skybook-extension-api";
import { useApplicationStore } from "./store";
import { debounce } from "@pistonite/pure/sync";

const setScript = debounce({
    fn: (script: string) => {
        useApplicationStore.setState({script});
    },
    interval: 200,
});

export class ApplicationApi implements Application {
    public async getScript() {
        return {val: useApplicationStore.getState().script};
    }
    public async setScript(script: string) {
        // await setScript(script);
        // return {};
        useApplicationStore.setState({script});
        return {};
    }
}
