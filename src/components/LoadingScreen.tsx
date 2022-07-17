import { PropsWithChildren } from "react";

// an over-engineered loading screen
export const LoadingScreen: React.FC<PropsWithChildren> = ({children})=>{
	return (
		<div style={{
			textAlign: "center",
			position: "absolute",
			top: 0,
			bottom: 0,
			left: 0,
			right: 0,
			backgroundColor: "#262626"
		}}>
			<span style={{
				color: "#00ffcc",
                
				lineHeight: "100vh",
				height: "100vh",
			}}>
				{children}
			</span>
		</div>
	);
};
