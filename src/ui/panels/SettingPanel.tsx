import { Button, Category, Control, Description, Label } from "ui/components";
import { Page } from "ui/surfaces";
import { useRuntime } from "data/runtime";

const cycleOnOffAuto = (value: boolean | "auto"): boolean | "auto" => {
	if (!value){
		return "auto";
	}
	if(value === "auto"){
		return true;
	}
	return false;
};

const settingToString = (value: boolean | "auto", on: string, off: string, auto = ""): string => {
	if (!value){
		return off;
	}
	if(value === "auto"){
		return auto;
	}
	return on;
};

const settingToButtonString = (value: boolean | "auto"): string => {
	return settingToString(value, "ON", "OFF", "AUTO");
};

export const SettingPanel: React.FC = () => {
	const { setting, setPage } = useRuntime();

	const isIconAnimated = setting("animatedIcon");
	const showSaves = setting("showSaves");
	const showGameData = setting("showGameData");
	const interlaceGameData = setting("interlaceGameData");

	return (
		<Page title="Setting">
			<Category title="Layout">
				<Button className="Small" onClick={()=>{
					setting("showSaves", cycleOnOffAuto(showSaves));
				}}>
					{settingToButtonString(showSaves)}
				</Button>
				<Label>{settingToString(showSaves, "Always showing saves", "Don't show saves", "Show when more than 1 save")}</Label>
				<Description>
                    Decide how the Save Data panel should be displayed
				</Description>

				<Button className="Small" onClick={()=>{
					setting("showGameData", cycleOnOffAuto(showGameData));
				}}>
					{settingToButtonString(showGameData)}
				</Button>
				<Label>{settingToString(showGameData, "Always showing Game Data", "Don't show Game Data", "Show when Game Data is desynced")}</Label>
				<Description>
                    Decide how the Game Data panel should be displayed
				</Description>

				<Control disabled={showGameData === false}>
					<Button className="Small" onClick={()=>{
						setting("interlaceGameData", !interlaceGameData);
					}}>
						{settingToButtonString(interlaceGameData)}
					</Button>
					<Label>Interlace Game Data with Inventory</Label>
					<Description className="Primary">
                        Toggle if Game Data should be interlaced with Visible Inventory.
					</Description>
					<Description>
                        When they are interlaced, you could see how the slots correspond during an Inventory Corruption.
                        (Has no effect if Game Data Panel is hidden.)
					</Description>
				</Control>

			</Category>
			<Category title="Items">
				<Button className="Small" onClick={()=>setting("animatedIcon", !isIconAnimated)}>
					{settingToButtonString(isIconAnimated)}
				</Button>
				<Label>{settingToString(isIconAnimated, "Show animated icons", "Show static icons")}</Label>
				<Description>
                    Toggle if icons should be animated for available items.
				</Description>
			</Category>
			<Category title="Advanced">
				<Control disabled>
					<Button className="Small">OFF</Button>
					<Label>Superuser Mode</Label>
					<Description className="Primary Error">
                        This option will be available in a future update
					</Description>
					<Description>
                        Enable editing of all commands.
                        Some commands are simplified for non-advanced users and are read-only
                        unless in superuser mode
					</Description>
				</Control>
			</Category>

			<Button className="Full" onClick={()=>setPage("#simulation")}>Close</Button>
		</Page>

	);
};
