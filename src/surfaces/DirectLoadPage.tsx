import { PropsWithChildren, useMemo } from "react";
import { parse } from "query-string";
import { deserialize } from "data/serialize";
import { LoadingScreen } from "components/LoadingScreen";
import { BodyText, Header, SubHeader } from "components/Text";

const redirectToMainApp = ()=>{
	window.location.href = window.location.origin;
};

export const DirectLoadPage: React.FC<PropsWithChildren> = ({children}) => {
	const query = parse(window.location.search);
	const [commandTextToLoad, errorText] = useMemo(()=>{
		try {
			return [deserialize(query), false];
		} catch (e) {
			console.error(e);
			return [null, "Fail to deserialize. The URL may be corrupted."];
		}
	}, [query]);

	if(errorText){
		return (
			<LoadingScreen hasError multiLine>
				<Header>Error loading direct URL</Header>
				<SubHeader>{errorText}</SubHeader>
				<BodyText>The browser console may have useful information for debugging</BodyText>
				<BodyText emphasized>Press Continue to load existing data in the simulator instead</BodyText>
				<button className="MainButton" onClick={()=>{
					redirectToMainApp();
				}}>Continue</button>
			</LoadingScreen>
		);
	}

	if (!commandTextToLoad){
		return <>{children}</>;
	}

	if (!localStorage.getItem("HDS.CurrentCommandsText")){
		// If no data is in simulator (i.e. first time use), load data without warning
		localStorage.setItem("HDS.CurrentCommandsText", commandTextToLoad);
		redirectToMainApp();
	}

	return <LoadingScreen multiLine>
		<div className="OtherPageContent">
			<Header>Open Direct URL?</Header>
			<SubHeader>You are trying to open a direct URL. This will automatically load data into the simulator.</SubHeader>
			<BodyText emphasized>This will override existing data and cannot be reversed</BodyText>
			<button className="MainButton" onClick={()=>{
				localStorage.setItem("HDS.CurrentCommandsText", commandTextToLoad);
				redirectToMainApp();
			}}>Yes</button>
			<button className="MainButton" onClick={()=>{

				redirectToMainApp();
			}}>No</button>

			<div style={{marginTop: "50px", marginLeft: "10%", marginRight: "10%"}}>
				<BodyText>(Below is what the incoming data looks like)</BodyText>
				<textarea
					className="MainInput"
					spellCheck={false}
					value={commandTextToLoad}
				/>
			</div>

		</div>

	</LoadingScreen>;
};
