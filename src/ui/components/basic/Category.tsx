import { PropsWithChildren } from "react";

type CategoryProps = {
    title: string,
}

export const Category: React.FC<PropsWithChildren<CategoryProps>> = ({title, children}) => {
    return (
        <div>
            <h3>{title}</h3>
            {children}
        </div>
    );
};
