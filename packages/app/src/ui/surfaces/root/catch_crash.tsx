import React, { type PropsWithChildren } from "react";

import { crashApp } from "self::application";
import { log } from "self::util";

type CrashState = {
    error?: Error;
};

export class CatchCrash extends React.Component<PropsWithChildren, CrashState> {
    private crashCalled: boolean;
    constructor(props: PropsWithChildren) {
        super(props);
        this.state = { error: undefined };
        this.crashCalled = false;
    }

    static getDerivedStateFromError(error: Error) {
        return { error };
    }

    override render() {
        if (this.state.error) {
            if (this.crashCalled) {
                return null;
            }
            this.crashCalled = true;
            setTimeout(() => {
                log.error("rendering crash screen with error:");
                log.error(this.state.error);
                crashApp();
            }, 0);
            return null;
        }

        return this.props.children;
    }
}
