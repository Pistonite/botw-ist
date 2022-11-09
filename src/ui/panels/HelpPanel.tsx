import { Version } from "data/util";
import { Page } from "ui/surfaces";

export const HelpPanel: React.FC = () => {
	return (
		<Page title="Help">
			<code>botw-ist {Version}</code>
			<br/>
			still working on this page
			<br></br>
			Helpful reading for understanding IST: <a href="https://restite.org/reload/#">https://restite.org/reload</a> by savage13

		</Page>
	);
};
