import clsx from "clsx";
import { PropsWithChildren } from "react";

type DivProps = React.DetailedHTMLProps<React.HTMLAttributes<HTMLDivElement>, HTMLDivElement>;
type CategoryProps = {
    title: string,
}

export const Category: React.FC<PropsWithChildren<DivProps & CategoryProps>> = ({className, title, children, ...restProps}) => {
    return (
        <div className={clsx("Category", className)} {...restProps}>
            <h3 className="CategoryHeader">{title}</h3>
            <hr />
            <div className="CategoryContent">
                {children}
            </div>
        </div>
    );
};
