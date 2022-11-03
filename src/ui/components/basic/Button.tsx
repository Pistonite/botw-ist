import clsx from "clsx";
import { PropsWithChildren } from "react";

type ButtonProps = React.DetailedHTMLProps<React.ButtonHTMLAttributes<HTMLButtonElement>, HTMLButtonElement>;
type CustomButtonProps = {
    variableWidth?: boolean
}

export const Button: React.FC<PropsWithChildren<ButtonProps & CustomButtonProps>> = ({variableWidth, children, ...restProps})=>{
    const className = clsx(
        "Button",
        !variableWidth && "FixedWidth"
    )
    return <button className={className} {...restProps}>
        {children}
    </button>
};
