import { AmountAll, AmountAllType } from "core/command/ItemStackArg";
import { Item, ItemStack, ItemType } from "data/item";
import { circularForEachFromIndex, inPlaceFilter } from "data/util";
import { RemoveOption } from "./options";
import { SlotsCore } from "./SlotsCore";

// REMOVE function
// This function does not have IST related logic. It is purely made up by the simulator for easy access
// stack is the item to remove
//  Will try to delete matching the metadata first. If count cannot be satisfied, then it will continue to match without metadata
// count is the number of "items" to remove, or All
//  For stackable items, count respects the stack size
//  For unstackble items, count is the number of slots. However, if option.forceStackableFood is set, all food will be treated as stackble
// returns the total amount removed
export const remove = (core: SlotsCore, stackToRemove: ItemStack, count: number | AmountAllType, option: Partial<RemoveOption> = {}): number => {

    const {
        // The slot of matched item to start processing the remove.
        // For example, if start slot = 1, the second slot will be processed first, and it will wrap to the first slot at the end
        startSlot,
        // When true, corrupted food will be treated as stackable
        // This is to handle the difference between eat and sell/remove
        forceStackableFood,
        // When true, delete empty arrow slots
        forceDeleteZeroSlot
    } = {
        startSlot: 0,
        forceStackableFood: false,
        forceDeleteZeroSlot: false,
        ...option
    };

    // the slot indices to be deleted in the end
    const slotIndexToRemove: number[] = [];
    // the slot indices to process the removal. Order matters. Duplicate OK
    const slotsToRemoveFrom: number[] = [];
    let countLeft = count;
    // countLeft could be "All", so we need another variable to track how many are removed
    let removedCount = 0;
    const stacks = core.internalSlots;
    const specialIsStackable = (item: Item) => {
        if(forceStackableFood && item.type === ItemType.Food){
            return true;
        }
        return item.stackable;
    }

    const matchedSlots: number[][] = [[],[],[],[]];
    // we want to match in this order:
    // 1. Everything matches
    stacks.forEach((currentStack,i)=>{
        if(currentStack.equals(stackToRemove)){
            matchedSlots[0].push(i);
        }
    });

    // 2. Everything matches except stack size/durability
    stacks.forEach((currentStack,i)=>{
        if(currentStack.equalsExcept(stackToRemove, "count")){
            matchedSlots[1].push(i);
        }
    });

    // 3. Everything matches except stack size/durability and equipped/unequipped
    // this is because when specifying an equipment, it will have a default durability and default equipped=false
    // being equipped does not make the item different from the user's perspective
    stacks.forEach((currentStack,i)=>{
        if(currentStack.equalsExcept(stackToRemove, "count", "equipped")){
            matchedSlots[2].push(i);
        }
    });

    // last: only item matches
    stacks.forEach((currentStack,i)=>{
        if(currentStack.item === stackToRemove.item){
            matchedSlots[3].push(i);
        }
    });

    // For each matched set, circular process it and add to the big remove list
    matchedSlots.forEach(matchedArray=>{
        if(startSlot>=matchedArray.length){
            //if slot is greater, user probably didn't intend to remove like this. skip.
            return;
        }
        circularForEachFromIndex(matchedArray, startSlot, i=>slotsToRemoveFrom.push(i));
    })

    for(let j = 0;j<slotsToRemoveFrom.length && (countLeft === AmountAll || countLeft > 0);j++){
        const i = slotsToRemoveFrom[j];
        const currentStack = stacks[i];
        if(currentStack.count === 0){
            // since indices can be duplicated, the stack could already be empty
            continue;
        }
        if(specialIsStackable(currentStack.item)){
            if(countLeft !== AmountAll){
                // Note that the equal case must be in the else clause
                // because when forceDeleteZeroSlot = 0, it needs to push the index to the delete list
                if(currentStack.count > countLeft){
                    // this stack is enough
                    removedCount+=countLeft;
                    stacks[i] = currentStack.modify({count: currentStack.count - countLeft});
                    countLeft = 0;
                }else{
                    // this stack is not enough
                    stacks[i] = currentStack.modify({count: 0});
                    removedCount += currentStack.count;
                    countLeft -= currentStack.count;
                    if(forceDeleteZeroSlot){
                        slotIndexToRemove.push(i);
                    }
                }
            }else{
                // removing all stackable
                removedCount += currentStack.count;
                stacks[i] = currentStack.modify({count: 0});
                if(forceDeleteZeroSlot){
                    slotIndexToRemove.push(i);
                }
            }
        }else{
            // countLeft is definitely > 0 because of loop condition, no need to check
            // Also make the count 0 so it's skipped in case of duplicates
            removedCount += 1;
            stacks[i] = currentStack.modify({count: 0});
            if(forceDeleteZeroSlot){
                slotIndexToRemove.push(i);
            }
            if(countLeft !== AmountAll){
                countLeft--;
            }
        }

    }

    if(slotIndexToRemove.length > 0){
        slotIndexToRemove.sort();
        let j = 0;
        inPlaceFilter(stacks, (_,i)=>{
            if(j<slotIndexToRemove.length && i===slotIndexToRemove[j]){
                j++;
                return false;
            }
            return true;
        })
    }

    core.removeZeroStackExceptArrows();
    return removedCount;
};
