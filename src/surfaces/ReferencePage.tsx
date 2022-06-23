import { ItemList } from "components/ItemList";
import { TitledList } from "components/TitledList";
import React from "react";

export const ReferencePage: React.FC = React.memo(()=>{



    return (
        <div style={{height: "100%", width: "100%", color: "white"}}>
            <TitledList title="Reference">
                <div style={{padding: 10}}>
                    <h2>Commands</h2>
                    <h3 className="Reference">Initialize X item1 Y item2 Z item3 ...</h3>
                    <h4 className="Reference">Used for initializing inventory before simulation</h4>
                    <p className="Reference">
                        Fully resets the inventory by clearing all items and set Count to 0, then forcefully write the item list to inventory.
                        This would reset any broken slot you already have, and any in-game checks that happen when adding items are disabled.
                        For example, the items will appear in the order you specify, not in the in-game tab order
                    </p>
                    <p className="Reference">
                        If you specify count &gt; 1 for unstackable items like weapon or sheika slate, multiple of that item would be added.
                        Game Data will be synced with Visible Inventory after the reset
                    </p>
                    <p className="Reference">
                        Note that this will not clear saves. You can use this command to initialize multiple saves
                    </p>
                    <p className="Reference Example">Example: Initialize 1 Apple 2 Axe 3 Slate 4 SpiritOrb</p>

                    <h3 className="Reference">Save / Save As NAME</h3>
                    <h4 className="Reference">Simulates a hard save or auto save action</h4>
                    <p className="Reference">
                        Writes Game Data to the corresponding save slot. The auto saves are specified by NAME. 
                        You can have as many auto saves as you want in the simulator.
                    </p>
                    
                    <p className="Reference Example">Example 1: Save</p>
                    <p className="Reference Example">Example 2: Save As MySave</p>
                    <p className="Reference">
                        Example 1 will save to the manual save slot, while example 2 will save to the slot named "MySave".
                        There cannot be spaces in the name. If "MySave" doesn't exist, a new slot is created
                    </p>

                    <h3 className="Reference">Reload (NAME)</h3>
                    <h4 className="Reference">Simulates reloading a save</h4>
                    <p className="Reference">
                        First, reads Game Data from the corresponding save slot. 
                        If NAME is not given, the manual save is used unless "Use" commands are used before this (see below).
                        If NAME is given, the corresponding save slot with that name is used
                    </p>
                    <p className="Reference">
                        After that, the first Count items in the visible inventory is removed, and Count is decreased accordingly. 
                        Then, each item slot in the Game Data is added to the inventory.
                    </p>
                    
                    <p className="Reference Example">Example 1: Reload</p>
                    <p className="Reference Example">Example 2: Reload MySave</p>

                    <h3 className="Reference">Use NAME</h3>
                    <h4 className="Reference">(Deprecated) Specify which save to load on the subsequent reload</h4>
                    <p className="Reference Example">
                        This command is only for backward compatibility. Use "Reload" instead
                    </p>
                    <p className="Reference">
                        Specify the save named NAME to be reloaded on the next "Reload" command
                    </p>
                    
                    <p className="Reference Example">Example: Use MySave</p>

                    <h3 className="Reference">Break X Slots</h3>
                    <h4 className="Reference">Simulate making X broken slots with hold smuggle glitch</h4>
                    <p className="Reference">
                        Decrease inventory Count by X
                    </p>
                    <p className="Reference">
                        This command does not automatically simulate the hold smuggle and sell process. 
                        It just changes count (i.e. make broken slots) with magic.
                    </p>
                    
                    <p className="Reference Example">Example: Break 4 Slots</p>
                </div>
                
            </TitledList>
        </div>
    )
});
