import clsx from "clsx"
import { ItemList, ItemListProps } from "components/ItemList"
import { Command } from "core/Command"
import { itemToType } from "core/Item"
import { ItemStack, ItemType } from "core/ItemStack"
import { parseCommand } from "core/Parser"

import React, { useEffect, useState } from "react"

type DisplayPaneProps = {
    command: string,
    displayIndex: number,
    stacks: ItemStack[],
    numBroken: number,
    editCommand: (c: Command)=>void
}

const stacksToItemListProps = (stacks: ItemStack[], numBroken: number): [ItemListProps, ItemListProps, ItemListProps] => {
  const materials = stacks.filter(stack=>itemToType(stack.item)==ItemType.Material);
  const meals = stacks.filter(stack=>itemToType(stack.item)==ItemType.Meal);
  const keyItems = stacks.filter(stack=>itemToType(stack.item)==ItemType.Key);
  return [
    {
      items: stacksToNameAndCount(materials),
      numBroken: Math.max(0, numBroken - keyItems.length - meals.length )
    },
    {
      items: stacksToNameAndCount(meals),
      numBroken: Math.max(0, numBroken - keyItems.length)
    },{
      items: stacksToNameAndCount(keyItems),
      numBroken
    },
  ]
}

const stacksToNameAndCount = (stacks: ItemStack[]): ItemListProps["items"] => {
  return stacks.map(({item, count})=>({name: item, count}));
}

export const DisplayPane: React.FC<DisplayPaneProps> = ({command,editCommand,displayIndex, stacks, numBroken})=>{
    const [commandString, setCommandString] = useState<string>("");
    const [hasError, setHasError] = useState<boolean>(false);
    const [materialListProps, mealListProps, keyItemListProps] = stacksToItemListProps(stacks, numBroken);
    useEffect(()=>{
      if(commandString!==command){
        setCommandString(command);
        setHasError(false);
      }
      
    }, [command, displayIndex]);

    return <div id="DisplayPane" style={{
        width: "calc( 100% - 300px - 5px )",
        float: "right",
        border: "1px solid black",
        boxSizing: "content-box"
       } }>
        <div style={{
        marginBottom: 2,
        boxSizing: "content-box",
        height: "50px"
       } }>
        <input className={clsx("Calamity", hasError && "InputError")} style={{
          marginTop: 2,
          width: "80%",
          height: "40px",
          fontSize: "20pt",
          
        }}value={commandString}
        placeholder="Type command here..."
        onChange={(e)=>{
          const cmdString = e.target.value;
          setCommandString(cmdString);
          const parsedCommand = parseCommand(cmdString);
          if(parsedCommand){
            editCommand(parsedCommand);
            setHasError(false);
          }else{
            setHasError(true);
          }
        }}></input>
        <button onClick={()=>{
          alert(`Available Commands:
Initialize X Item1 Y Item2 Z Item3 ...
Break X Slots - add X broken slots
Save
Reload
Sort Key/Material - sort key items or material
Get/Add/Cook/Pickup X ITEM
Remove/Drop/Sell X ITEM From Slot Y
Remove/Sell/Eat MEAL From Slot X

Limitations:
When you reload without altering inventory, things become weird. It won't be handled correctly and the commands will become red
  `);
        }}>Reference</button>
        </div>
        
      <div style={{
        borderTop: "1px solid black",
        borderBottom: "1px solid black",
        marginBottom: 2,
        boxSizing: "content-box",
        height: "calc( ( 99vh - 60px ) / 3)",
        overflowY: "auto"
       } }>
        <ItemList {...materialListProps}/>
      </div>
      <div style={{
        borderTop: "1px solid black",
        boxSizing: "content-box",
        borderBottom: "1px solid black",
        marginBottom: 2,
        height: "calc( ( 99vh - 60px ) / 3)",
        overflowY: "auto"
       } }>
        <ItemList {...mealListProps}/>
      </div>
      <div style={{
        borderTop: "1px solid black",
        boxSizing: "content-box",
        height: "calc( ( 99vh - 60px ) / 3)",
        overflowY: "auto"
       } }>
        <ItemList {...keyItemListProps}/>
      </div>
      

      </div>
}
