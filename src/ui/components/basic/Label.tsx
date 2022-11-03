import { PropsWithChildren } from "react";


export const Label: React.FC<PropsWithChildren> = ({children}) => {
    return (
        <span>
            {children}
        </span>
    );
};
