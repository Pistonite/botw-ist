import { BodyText, SubHeader, SubTitle } from "components/Text";
import { TitledList } from "components/TitledList";
import { saveAs } from "data/FileSaver";
import { serialize } from "data/serialize";
import { useEffect, useMemo, useRef, useState } from "react";

const URL_MAX = 2048;

type OptionPageProps = {
	interlaceInventory: boolean,
	setInterlaceInventory: (value: boolean)=>void,
	isIconAnimated: boolean,
	setIsIconAnimated: (value: boolean)=>void,
	commandText: string,
	setCommandText: (value: string)=>void,
}

export const OptionPage: React.FC<OptionPageProps> = ({
	interlaceInventory,
	setInterlaceInventory,
	isIconAnimated,
	setIsIconAnimated,
	commandText,
	setCommandText
}) => {
	const [currentText, setCurrentText] = useState<string>(commandText);
	const [fileName, setFileName] = useState<string>("");
	const [showDirectUrl, setShowDirectUrl] = useState<boolean>(false);
	const [showCopiedMessage, setShowCopiedMessage] = useState<boolean>(false);

	const uploadRef = useRef<HTMLInputElement>(null);

	const directUrl = useMemo(()=>{
		const serializedCommands = serialize(commandText);
		const query = new URLSearchParams(serializedCommands).toString();
		return `${window.location.origin}/?${query}`;
	}, [commandText]);

	const directUrlLength = directUrl.length;

	useEffect(()=>{
		setShowCopiedMessage(false);
	}, [currentText]);

	return (
		<div className="OtherPage">

			<input ref={uploadRef} id="Upload" type="File" hidden onChange={(e)=>{
				const files = e.target.files;
				if(files?.length && files[0]){
					const file = files[0];
					const fileName = file.name.endsWith(".txt") ? file.name.substring(0, file.name.length-4) : file.name;
					setFileName(fileName);
					file.text().then(text=>{
						setCurrentText(text);
						setCommandText(text);
					});
				}
			}}/>

			<TitledList title="Options">
				<div className="OtherPageContent">
					<SubHeader>
						Interlace Inventory with GameData
						<button className="MainButton" onClick={()=>{
							setInterlaceInventory(!interlaceInventory);
						}}>
							{interlaceInventory ? "ON" : "OFF"}
						</button>
					</SubHeader>
					<SubTitle>Toggle whether Visible Inventory should be displayed separetely from Game Data or interlaced.</SubTitle>

					<SubHeader>
						Enable Animated Item Icons
						<button className="MainButton" onClick={()=>{
							setIsIconAnimated(!isIconAnimated);
						}}>
							{isIconAnimated ? "ON" : "OFF"}
						</button>
					</SubHeader>
					<SubTitle>Toggle whether items such as the champion abilities or Travel Medallion use animated or still icons.</SubTitle>

					<SubHeader>Text Import / Export</SubHeader>
					<SubTitle>You can also directly copy, paste, or edit the commands here</SubTitle>
					<BodyText>
						<button className="MainButton" onClick={()=>{
							if(uploadRef.current){
								uploadRef.current.click();
							}
						}}>
							Import
						</button>
						<button className="MainButton" onClick={()=>{
							saveAs(currentText, fileName+".txt" || "dupe.txt");
						}}>
							Export
						</button>
						<input
							className="MainInput"
							spellCheck={false}
							value={fileName}
							onChange={(e)=>{
								setFileName(e.target.value);
							}}
							placeholder="File name"
						/>
						<textarea
							className="MainInput"
							spellCheck={false}
							value={currentText}
							onChange={(e)=>{
								setCurrentText(e.target.value);
							}}

						/>
						{
							currentText !== commandText &&
							<>
								<button className="MainButton" onClick={()=>{
									setCommandText(currentText);
								}}>
									Save
								</button>
								<span className="Example">Don't forget to save changes</span>
							</>
						}
					</BodyText>

					<SubHeader>Direct URL</SubHeader>
					<SubTitle>Use this to open the simulator with the steps automatically loaded.</SubTitle>
					<div>
						{
							currentText !== commandText ?
								<BodyText emphasized>
									You must save the changes above to access the updated URL
								</BodyText>
								:
								<>
									{
										directUrlLength > URL_MAX && <BodyText emphasized>
											Warning: The URL is too long ({directUrlLength} characters) and may not work in certain browsers. Export as file instead if you encounter any problems.
										</BodyText>
									}
									<p className="Reference" style={{
										fontSize: "10pt",
										color: "#aaaaaa",

										...!showDirectUrl && {
											textOverflow: "ellipsis",
											overflowX: "hidden",
											whiteSpace: "nowrap",
										}
									}}>
										{directUrl}
									</p>

									<BodyText>
										<button className="MainButton" onClick={()=>{
											setShowDirectUrl(!showDirectUrl);
										}}>{showDirectUrl ? "Hide" : "Expand"}</button>
										<button className="MainButton" disabled={currentText !== commandText} onClick={()=>{
											window.navigator.clipboard.writeText(directUrl);
											setShowCopiedMessage(true);
										}}>
										Copy
										</button>
										{
											showCopiedMessage && <span className="Example">Link copied!</span>
										}

									</BodyText>
								</>
						}
					</div>
				</div>
			</TitledList>
		</div>
	);
};
