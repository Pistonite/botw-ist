import { Button, Description, Label } from "ui/components";

// an over-engineered crash screen

type CrashProps = {
	primaryText: string,
	secondaryText: string | Error
}

const redirectToMainApp = ()=>{
	window.location.href = window.location.origin;
};

export const CrashScreen: React.FC<CrashProps> = ({primaryText, secondaryText})=>{
	return (
		<div className="SpecialScreen" style={{
			textAlign: "center",
			position: "absolute",
			top: 0,
			bottom: 0,
			left: 0,
			right: 0,
			backgroundColor: "#010101"
		}}>
			<div style={{
				color: "#ffffff",
				fontSize: "40pt",
				position: "absolute",
				top: "30%",
				left: "calc( 50% - 40px )",
				right: "calc( 50% - 40px )",
				border: "2px solid #ffffff"
			}}>!</div>
			<div style={{
				textAlign: "center",
				position: "absolute",
				top: 0,
				bottom: 0,
				left: 0,
				right: 0,
				color: "#ffffff",
				marginTop: "49vh",
				height: "100vh",
			}}>
				<Label>{primaryText}</Label>
				<Description>{`${secondaryText}`}</Description>
				<Button onClick={redirectToMainApp}>Reload Simulator</Button>
			</div>
		</div>
	);
};
