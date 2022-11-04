import clsx from "clsx";
import { PropsWithChildren } from "react";

type ButtonProps = React.DetailedHTMLProps<React.ButtonHTMLAttributes<HTMLButtonElement>, HTMLButtonElement>;

export const Button: React.FC<PropsWithChildren<ButtonProps>> = ({
    className, children, ...restProps
})=>{
    return (
        <button className={clsx("Button", className)} {...restProps}>
            {children}
        </button>
    );
};
