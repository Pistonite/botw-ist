import React, { type PropsWithChildren } from "react";

import { crashApp } from "self::application/store";

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
                console.error(
                    "rendering crash screen with error:",
                    this.state.error,
                );
                crashApp();
            }, 0);
            return null;
        }

        return this.props.children;
    }
}
