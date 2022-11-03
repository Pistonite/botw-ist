import { PropsWithChildren } from "react";


export const Description: React.FC<PropsWithChildren> = ({children}) => {
    return (
        <p>
            {children}
        </p>
    );
};
