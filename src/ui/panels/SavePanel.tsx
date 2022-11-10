import { Command } from "core/command";
import { SimulationState } from "core/SimulationState";
import { useRuntime } from "data/runtime";
import { GetSetPair } from "data/util";
import { CommandItem, ItemList, Section } from "ui/components";
import { ContextMenuState } from "ui/types";

type SavePanelProps = {
	simulationState: SimulationState | null,
	showSaves: boolean,
}
& GetSetPair<"selectedSaveName", string>;

export const SavePanel: React.FC<SavePanelProps> = ({
    simulationState,
    selectedSaveName,
    setSelectedSaveName
}) => {
    const { setPage, setting } = useRuntime();
    const isIconAnimated = setting("animatedIcon");
    return (
        <div style={{
            display: "flex",
            height: "100%"
        }}>
            <div style={{
                width: 300,
                flexShrink: 0,
                height: "100%"
            }}>
                <Section titleText="Saves" style={{
                    
                    
                    border: "1px solid black",
                    boxSizing: "border-box",
                    height: "100%",
                    overflowY: "auto"
                }}
                >

                    {
                        !!simulationState &&
                            <ol>
                                {
                                    !!simulationState.getManualSave() &&
                                    <CommandItem
                                        onClick={()=>{
                                            setSelectedSaveName("");
                                            setPage("#simulation");
                                        }}
                                        useListItem
                                        isSelected={selectedSaveName===""}

                                    >
                                Manual Save
                                    </CommandItem>
                                }

                                {
                                    Object.entries(simulationState.getNamedSaves()).map(([name, _gamedata], i)=>
                                        <CommandItem
                                            key={i}
                                            onClick={()=>{
                                                setSelectedSaveName(name);
                                                setPage("#simulation");
                                            }}
                                            isSelected={selectedSaveName===name}
                                            useListItem
                                        >
                                            {name}
                                        </CommandItem>
                                    )
                                }
                            </ol>
                    }

                </Section>
            </div>
            
            
            <div style={{
                height: "100%",
                overflowY: "hidden",
                color: "white",
                backgroundColor: "#262626",
                flexGrow: 1
            } }>
                {
                    simulationState &&
                        <Section titleText="Save Data" style={{
                            height:"100%"
                        }}>
                            {
                                (()=>{
                                    if (selectedSaveName === ""){
                                        const manualSave = simulationState.getManualSave();
                                        if(manualSave){
                                            return <ItemList slots={manualSave.getDisplayedSlots(isIconAnimated)}/>;
                                        }
                                    }else if(selectedSaveName){
                                        const namedSaves = simulationState.getNamedSaves();
                                        if(selectedSaveName in namedSaves){
                                            const save = namedSaves[selectedSaveName];
                                            return <ItemList slots={save.getDisplayedSlots(isIconAnimated)}/>;
                                        }
                                    }
                                    return null;
                                })()
                            }
                        </Section>
                }

            </div>
			
			
        </div>
    )
}
