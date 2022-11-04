import clsx from "clsx";
import { PropsWithChildren } from "react";

type PProps = React.DetailedHTMLProps<React.HTMLAttributes<HTMLParagraphElement>, HTMLParagraphElement>;

export const Description: React.FC<PropsWithChildren<PProps>> = ({className, children, ...restProps}) => {

	const anyRestProps = restProps as any; // eslint-disable-line @typescript-eslint/no-explicit-any
	return (
		<p className={clsx(
			"Description",
			anyRestProps.disabled && "Disabled",
			className
		)} {...restProps}>
			{children}
		</p>
	);
};
