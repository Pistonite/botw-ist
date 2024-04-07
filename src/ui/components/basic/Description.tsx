import clsx from "clsx";
import { PropsWithChildren } from "react";

type PProps = React.DetailedHTMLProps<
    React.HTMLAttributes<HTMLParagraphElement>,
    HTMLParagraphElement
>;
type DescriptionProps = {
    useDiv?: boolean;
};
export const Description: React.FC<
    PropsWithChildren<PProps & DescriptionProps>
> = ({ useDiv, className, children, ...restProps }) => {
    const anyRestProps = restProps as any; // eslint-disable-line @typescript-eslint/no-explicit-any
    if (useDiv) {
        return (
            <div
                className={clsx(
                    "Description",
                    anyRestProps.disabled && "Disabled",
                    className,
                )}
                {...restProps}
            >
                {children}
            </div>
        );
    }
    return (
        <p
            className={clsx(
                "Description",
                anyRestProps.disabled && "Disabled",
                className,
            )}
            {...restProps}
        >
            {children}
        </p>
    );
};
