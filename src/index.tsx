import React from "react";
import ReactDOM from "react-dom/client";
import "ui/css";
import {App} from "./App";
import { reportWebVitalsAsync } from "data/web-vitals";
import { LanguageProvider } from "data/i18n";
import { ItemProvider } from "data/item";
import { DirectLoadPage } from "surfaces/DirectLoadPage";

const root = ReactDOM.createRoot(
  document.getElementById("root") as HTMLElement
);
root.render(
	<React.StrictMode>
		<LanguageProvider>
			<DirectLoadPage>
				<ItemProvider>
					<App />
				</ItemProvider>
			</DirectLoadPage>
		</LanguageProvider>
	</React.StrictMode>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitalsAsync();
