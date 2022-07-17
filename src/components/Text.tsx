import clsx from "clsx";
import React, { PropsWithChildren } from "react";

export const Header: React.FC<PropsWithChildren> = ({children})=>{
	return <h2>{children}</h2>;
};

export const SubHeader: React.FC<PropsWithChildren<{connected?:boolean}>> = ({connected, children})=>{
	return <h3 className={connected?"Reference2":"Reference"}>{children}</h3>;
};

export const SubTitle: React.FC<PropsWithChildren> = ({children})=>{
	return <h4 className="Reference">{children}</h4>;
};

export const BodyText: React.FC<PropsWithChildren<{emphasized?: boolean}>> = ({emphasized, children})=>{
	return <p className={clsx("Reference", emphasized && "Example")}>{children}</p>;
};

export const Emphasized: React.FC<PropsWithChildren> = ({children})=>{
	return <span className="Example">{children}</span>;
};
