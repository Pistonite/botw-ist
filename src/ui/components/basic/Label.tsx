import clsx from "clsx";
import { PropsWithChildren } from "react";

type SpanProps = React.DetailedHTMLProps<React.HTMLAttributes<HTMLSpanElement>, HTMLSpanElement>

export const Label: React.FC<PropsWithChildren<SpanProps>> = ({ className, children, ...restProps}) => {
    return (
        <span className={clsx("Label", className)} {...restProps}>
            {children}
        </span>
    );
};
