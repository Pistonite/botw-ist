import clsx from "clsx";
import { PropsWithChildren } from "react";

type PProps = React.DetailedHTMLProps<React.HTMLAttributes<HTMLParagraphElement>, HTMLParagraphElement>;

export const Description: React.FC<PropsWithChildren<PProps>> = ({className, children, ...restProps}) => {
    return (
        <p className={clsx(
            "Description",
            (restProps as any)["disabled"] && "Disabled",
            className
        )} {...restProps}>
            {children}
        </p>
    );
};
