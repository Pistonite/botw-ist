import { TitledList } from "components/TitledList";
import { saveAs } from "data/FileSaver";
import { useRef, useState } from "react";

type OptionPageProps = {
	interlaceInventory: boolean,
	setInterlaceInventory: (value: boolean)=>void,
	commandText: string,
	setCommandText: (value: string)=>void,
}

export const OptionPage: React.FC<OptionPageProps> = ({
	interlaceInventory,
	setInterlaceInventory,
	commandText,
	setCommandText
}) => {
	const [currentText, setCurrentText] = useState<string>(commandText);
	const [fileName, setFileName] = useState<string>("");
	const uploadRef = useRef<HTMLInputElement>(null);

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
 
					<h3 className="Reference">
						Interlace Inventory with GameData 
						<button className="MainButton" onClick={()=>{
							setInterlaceInventory(!interlaceInventory);
						}}>
							{interlaceInventory ? "ON" : "OFF"}
						</button>
					</h3>
					<h4 className="Reference">
						Toggle whether Visible Inventory should be displayed separetely from Game Data or interlaced.
					</h4>

					<h3 className="Reference">Import / Export</h3>
					<h4 className="Reference">
						You can also directly copy, paste, or edit the commands here
					</h4>
					<p className="Reference">
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
						
					</p>

				</div>
			</TitledList>
		</div>
	);
};
