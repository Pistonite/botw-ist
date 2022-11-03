import { PropsWithChildren } from "react";

type DivProps = React.DetailedHTMLProps<React.HTMLAttributes<HTMLDivElement>, HTMLDivElement>;
type SectionProps = {
    title: string
};

export const Section: React.FC<PropsWithChildren<DivProps | SectionProps>> = ({title, children, ...restProps}) => {
	return (
		<div {...restProps}>
			<h3 className="SectionHeader">
				{title}
			</h3>
			<div className="SectionContent">
				{children}
			</div>
		</div>
	);
};
