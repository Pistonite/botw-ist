import clsx from "clsx";
import React, { useCallback, useContext, useEffect, useRef, useState, PropsWithChildren } from "react";

type SetTooltipFunction = (screenX: number, screenY: number, tooltip: React.ReactNode[]) => void;
const TooltipContext = React.createContext<SetTooltipFunction>(()=>{/* empty */});
export const TooltipHost: React.FC<PropsWithChildren> = ({children}) => {
	const toolTipDivRef = useRef<HTMLDivElement>(null);
	const [tooltip, setTooltip] = useState<React.ReactNode[]>([]);
	const [tooltipX, setTooltipX] = useState<number>(0);
	const [tooltipY, setTooltipY] = useState<number>(0);
	const setScreenTooltip = useCallback((x: number, y: number, tooltips: React.ReactNode[])=>{
		setTooltipX(x+10);
		setTooltipY(y+10);
		setTooltip(tooltips);
	}, []);

	useEffect(()=>{
		if(toolTipDivRef.current){
			const rect = toolTipDivRef.current.getBoundingClientRect();
			if (rect.bottom > window.innerHeight){
				
				setTooltipY(tooltipY - rect.height- 20);
				
			}
			if( rect.right > window.innerWidth){
				setTooltipX(tooltipX - rect.width -20);
			}
		}
	}, [
		tooltipX, 
		tooltipY, 
		toolTipDivRef.current && toolTipDivRef.current.getBoundingClientRect().width,
		toolTipDivRef.current && toolTipDivRef.current.getBoundingClientRect().height,
	]);

	return <TooltipContext.Provider value={setScreenTooltip}>
		{children}
		{
			tooltip.length > 0 &&
			<div ref={toolTipDivRef} className="TooltipWindow" style={{
				position: "absolute",
				left: tooltipX,
				top: tooltipY,
			}}>
				{
					tooltip.map((t,i)=><p className={clsx("TooltipLine", i==0 && "TooltipHeader")} key={i}>{t}</p>)
				}
			</div>
		}
	</TooltipContext.Provider>;
};

const useSetTooltip = ()=>useContext(TooltipContext);

type TooltipProps = {
	title: React.ReactNode[]
}

export const Tooltip: React.FC<PropsWithChildren<TooltipProps>> = ({title, children}) => {
	const [coord, setCoord] = useState<[number, number]>([-1,-1]);
	const setTooltip = useSetTooltip();
	useEffect(()=>{
		if(coord[0] < 0 || coord[1] < 0){
			setTooltip(0,0,[]);
			return;
		}
		setTooltip(coord[0], coord[1], title);
	}, [setTooltip, coord, title]);

	return (
		<span onMouseMove={(e)=>{
			setCoord([e.clientX, e.clientY]);
		}} onMouseLeave={()=>{
			setCoord([-1,-1]);
		}}>
			{children}
		</span>
	);
};
