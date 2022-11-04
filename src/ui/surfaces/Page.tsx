import { PropsWithChildren } from "react";
import { Section } from "ui/components";

type PageProps = {
    title: string | React.ReactNode
}

export const Page: React.FC<PropsWithChildren<PageProps>> = ({title, children}) => {
	return (
		<Section className="Page" titleText={title}>
			<div className="PageContent">
				{children}
			</div>
		</Section>
	);
};
