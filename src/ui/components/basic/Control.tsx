import React from "react";
import { PropsWithChildren } from "react";

type ControlProps = {
    disabled?: boolean
}

export const Control: React.FC<PropsWithChildren<ControlProps>> = ({disabled, children}) => {
    if(!disabled){
        return <>{children}</>;
    }
    const newChildren = React.Children.map(children, child => {
        // Checking isValidElement is the safe way and avoids a
        // typescript error too.
        if (React.isValidElement(child)) {
          return React.cloneElement(child, { disabled } as any);
        }
        return child;
    });
    return <>{newChildren}</>;
}
