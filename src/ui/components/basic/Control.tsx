import React, { PropsWithChildren } from "react";

type ControlProps = {
    disabled?: boolean;
};

export const Control: React.FC<PropsWithChildren<ControlProps>> = ({
    disabled,
    children,
}) => {
    if (!disabled) {
        return <>{children}</>;
    }
    const newChildren = React.Children.map(children, (child) => {
        if (React.isValidElement(child)) {
            return React.cloneElement(child, { disabled } as any); // eslint-disable-line @typescript-eslint/no-explicit-any
        }
        return child;
    });
    return <>{newChildren}</>;
};
