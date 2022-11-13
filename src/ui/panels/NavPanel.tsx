import { Button } from "ui/components";
import { Tooltip } from "ui/surfaces";
import { useRuntime } from "core/runtime";

export const NavPanel: React.FC = ()=>{
	const { page, setPage, editing, saving, warnReadOnly } = useRuntime();
	const status = editing
		? saving
			? ""
			:
			<Tooltip title={["Not Saved", "Any change you make won't be saved. You can enable saving in the \"Script\" tab"]}>
                Not Saving
			</Tooltip>

		:
		<Tooltip title={["Read Only", "Data loaded by direct URL is read only by default. You can enable editing in the \"Script\" tab"]}>
            Read Only
		</Tooltip>
    ;
	return (
		<nav>
			<div style={{
				display: "flex",
				backgroundColor: warnReadOnly?"#ff8800":"#262626",
				color: "#ffffff",
				height: 40,
				alignItems: "center",
			}}>
				<Button className="Full" onClick={()=>{
					setPage(page === "#setting" ? "#simulation" : "#setting");
				}}>Setting</Button>
				<Button className="Full" onClick={()=>{
					setPage("#simulation");
				}}>Simulation</Button>
				<Button className="Full" onClick={()=>{
					setPage("#options");
				}}>Script</Button>
				<Button className="Full" onClick={()=>{
					setPage("#reference");
				}}>Commands</Button>
				<Button className="Full" onClick={()=>{
					setPage("#items");
				}}>Items</Button>
				<Button onClick={()=>{
					setPage("#help");
				}}>HELP</Button>

				<div style={{
					padding: "0 10px",
					color: warnReadOnly?"#000000":"#ffffff",
					cursor: "default"
				}}>
					{status}
				</div>

			</div>
		</nav>
	);

};
