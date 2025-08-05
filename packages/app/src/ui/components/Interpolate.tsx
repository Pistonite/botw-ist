import { type PropsWithChildren, memo, type ReactNode } from "react";

export type InterpolateProps = Record<string, ReactNode>;

/** Little port of the old Interpolate in react-i18next since it works better for us */
const InterpolateImpl: React.FC<PropsWithChildren<InterpolateProps>> = ({ children, ...props }) => {
    if (typeof children !== "string") {
        return <>{children}</>;
    }
    const parts = children
        .split(/({{[^}]+}})/g)
        .filter((x) => x)
        .map((part, i) => {
            if (part.startsWith("{{") && part.endsWith("}}")) {
                const key = part.slice(2, -2);
                if (key in props) {
                    return <span key={`${key}-${i}`}>{props[key]}</span>;
                }
            }
            return part;
        });

    return <>{parts}</>;
};

export const Interpolate = memo(InterpolateImpl);
