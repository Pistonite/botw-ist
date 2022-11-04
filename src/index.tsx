import React, { PropsWithChildren } from "react";
import ReactDOM from "react-dom/client";
import "ui/css";
import {App} from "./App";
import { reportWebVitalsAsync } from "data/web-vitals";
import { LanguageProvider } from "data/i18n";
import { ItemProvider } from "data/item";
import { StoreProvider } from "data/storage";
import { RuntimeProvider } from "data/runtime";
import { CrashScreen } from "ui/surfaces/CrashScreen";
import { Description } from "ui/components";
import { TooltipHost } from "ui/surfaces/Tooltip";

const root = ReactDOM.createRoot(
  document.getElementById("root") as HTMLElement
);

type CrashState = {
	error?: Error
}

class CatchCrash extends React.Component<PropsWithChildren, CrashState> {
	constructor(props: PropsWithChildren) {
		super(props);
		this.state = { error: undefined };
	}

	static getDerivedStateFromError(error: Error) {
		return { error };
	}

	render() {
		if (this.state.error) {
			return (
				<CrashScreen 
					primaryText="Oops, the simulator crashed. Please let the maintainers know."
					secondaryText={this.state.error}
				/>
			); 
		}

		return this.props.children; 
	}
}
root.render(
	<React.StrictMode>
		<CatchCrash>
			<StoreProvider>
				<RuntimeProvider>
					<LanguageProvider>
						<ItemProvider>
							<TooltipHost>
								<App />
							</TooltipHost>
						</ItemProvider>
					</LanguageProvider>
				</RuntimeProvider>
			</StoreProvider>
		</CatchCrash>
	</React.StrictMode>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitalsAsync();
