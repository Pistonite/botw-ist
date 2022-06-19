import { Command, CommandBreakSlots, CommandInitialize, CommandNothing, CommandReload, CommandSave } from 'core/Command';
import { Inventory } from 'core/Inventory';
import React, { useEffect, useMemo, useState } from 'react';

import './App.css';
import { CommandItem } from './components/CommandItem';
import { ItemStack as ISC } from './components/ItemStack';

import { ItemStack, ItemType } from 'core/ItemStack';
import { DisplayPane } from 'surfaces/DisplayPane';
import { Item } from 'core/Item';

export const App: React.FC =  () => {
  const [commands, setCommands] = useState<Command[]>([
    new CommandInitialize([
      {
        item: Item.Diamond,
        count: 5,
      },
      {
        item: Item.Slate,
        count: 1,
      },
      {
        item: Item.Glider,
        count: 1,
      },
      {
        item: Item.SpiritOrb,
        count: 4,
      }
    ]),
    new CommandBreakSlots(4),
    new CommandReload(),
    new CommandSave(),
    new CommandReload()
  ]);
  const [displayIndex, setDisplayIndex] = useState<number>(0);
  const [contextMenuX, setContextMenuX] = useState<number>(0);
  const [contextMenuY, setContextMenuY] = useState<number>(0);
  const [contextMenuShowing, setContextMenuShowing] = useState<boolean>(false);
  const [contextIndex, setContextIndex] = useState<number>(-1);
  console.log(Item.Diamond === "Diamond");
  console.log(Item);
  // compute props
  const inventories = useMemo(()=>{
    const inventories: Inventory[] = [];
    const inv = new Inventory();
    commands.forEach(c=>{
      c.execute(inv);
      inventories.push(inv.clone());
    });
    return inventories;
  }, [commands]);

  useEffect(()=>{
    window.onkeydown=(e)=>{
      if(e.code==="ArrowDown"){
        setDisplayIndex(Math.min(commands.length-1, displayIndex+1));
      }else if(e.code==="ArrowUp"){
        setDisplayIndex(Math.max(0, displayIndex-1));
      }
    }
  }, [commands, displayIndex]);


  return (
    <div className='Calamity'
    >
      
      <div id="CommandList" style={{
        width: "300px",
        height: "calc( 100vh - 5px )",
        overflowY: "auto",
        
        float: "left",
        border: "1px solid black",
        boxSizing: "content-box"
       } }>
        <ul style={{
          listStyleType: "none",
          paddingInlineStart: 0
        }}>
          {
            commands.map((c,i)=>(
              <CommandItem 
                onClick={()=>setDisplayIndex(i)} 
                onContextMenu={(x,y)=>{
                  setContextIndex(i)
                  setContextMenuX(x);
                  setContextMenuY(y);
                  setContextMenuShowing(true);
                }}
                key={i} 
                isSelected={displayIndex===i}
                isContextSelected={contextIndex===i}
                error={inventories[i].isInaccurate()}
              >
                {c.getDisplayString()}
              </CommandItem>
            ))
          }
          <CommandItem onClick={()=>{
            const arrCopy = [...commands];
            arrCopy.push(new CommandNothing());
            setCommands(arrCopy);
          }}>(new)</CommandItem>
         
        </ul>
      </div>
      
      <DisplayPane 
        displayIndex={displayIndex}
        command={commands[displayIndex].getDisplayString()} 
        stacks={inventories[displayIndex].getSlots()} 
        numBroken={inventories[displayIndex].getNumBroken()} 
        editCommand={(c)=>{
          const arrCopy = [...commands];
          arrCopy[displayIndex] = c;
          setCommands(arrCopy);
        }}/> 

      {
        contextMenuShowing && <div style={{
          position: "absolute",
          top: 0,
          left: 0,
          width: "100vw",
          height: "100vh",
        }} onClick={()=>{
          setContextMenuShowing(false);
          setContextIndex(-1);
        }} onContextMenu={(e)=>{
          setContextMenuShowing(false);
          setContextIndex(-1);
          e.preventDefault();
        }}>
          <div style={{
            position: "absolute",
            top: contextMenuY,
            left: contextMenuX,
            width: "200px",
            backgroundColor: "white",
            border: "1px solid black"
          }}>
            <ul style={{
              margin: 0,
            listStyleType: "none",
            paddingInlineStart: 0
          }}>
            <CommandItem onClick={()=>{
              const arrCopy = [...commands];
              arrCopy.splice(contextIndex, 0, new CommandNothing());
              setCommands(arrCopy);
              setContextMenuShowing(false);
                setContextIndex(-1);
            }}>Insert Above</CommandItem>
            <CommandItem error onClick={()=>{
              if(confirm("Delete?")){
                setCommands(commands.filter((_,i)=>i!==contextIndex));
                setContextMenuShowing(false);
                setContextIndex(-1);
              }
            }}>Delete</CommandItem>
          </ul>
          </div>
        </div>
      }
    </div>
  );
}
