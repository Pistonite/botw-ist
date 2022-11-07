import clsx from "clsx";
import { PropsWithChildren } from "react";

type DivProps = React.DetailedHTMLProps<React.HTMLAttributes<HTMLDivElement>, HTMLDivElement>;
type HeadingProps = React.DetailedHTMLProps<React.HTMLAttributes<HTMLHeadingElement>, HTMLHeadingElement>;
type SectionProps = {
    titleText: string | React.ReactNode,
	titleProps?: HeadingProps
};

export const Section: React.FC<PropsWithChildren<DivProps & SectionProps>> = ({className, titleText, titleProps, children, ...restProps}) => {
	return (
		<div className={clsx("Section", className)} {...restProps} >
			<h3 {...titleProps} className="SectionHeader">
				{titleText}
			</h3>
			<div className="SectionContent">
				{children}
			</div>
		</div>
	);
};
