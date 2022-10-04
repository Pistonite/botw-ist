import { PropsWithChildren } from "react";

type Props = {
	multiLine?: boolean, 
}

// an over-engineered loading screen
export const LoadingScreen: React.FC<PropsWithChildren<Props>> = ({multiLine, children})=>{
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
                
				lineHeight: multiLine?"default":"100vh",
				height: "100vh",
			}}>
				{children}
			</span>
		</div>
	);
};
