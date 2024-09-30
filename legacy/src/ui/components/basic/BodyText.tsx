import clsx from "clsx";
import { PropsWithChildren } from "react";

export const BodyText: React.FC<
    PropsWithChildren<{ emphasized?: boolean }>
> = ({ emphasized, children }) => {
    return (
        <p className={clsx("Reference", emphasized && "Example")}>{children}</p>
    );
};
