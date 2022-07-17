import { PropsWithChildren } from "react";

// an over-engineered crash screen
export const CrashScreen: React.FC<PropsWithChildren> = ({children})=>{
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
			<span style={{
				color: "#ffffff",
                
				lineHeight: "100vh",
				height: "100vh",
			}}>
				{children}
			</span>
		</div>
	);
};
