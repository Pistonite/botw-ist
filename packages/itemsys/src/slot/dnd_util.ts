import { logger } from "@pistonite/pure/log";

export const dndLog = logger("dnd", "#b2dc9b").default();

// /** Keep track of registered targets for dropping items */
// export class DropTargets {
//     private targets: Set<ItemDropTarget>;
//
//     constructor() {
//         this.targets = new Set<ItemDropTarget>();
//     }
//
//     /** Register a target, return a function to unregister */
//     public registerDropTarget(target: ItemDropTarget): () => void {
//         this.targets.add(target);
//         return () => {
//             this.targets.delete(target);
//         };
//     }
//
//     /** Invoke the handler on the target if the item is dropped on a registered target */
//     public dropItem(data: ItemDragData, clientX: number, clientY: number) {
//         const toRemove = [];
//         let foundHandler: ItemDropTarget["handler"] | undefined = undefined;
//         for (const target of this.targets) {
//             const { element, handler } = target;
//             if (!element.isConnected) {
//                 toRemove.push(target);
//                 continue;
//             }
//             const { top, left, right, bottom } = element.getBoundingClientRect();
//             if (top > clientY || left > clientX || right < clientX || bottom < clientY) {
//                 continue;
//             }
//             foundHandler = handler;
//             break;
//         }
//         for (const target of toRemove) {
//             this.targets.delete(target);
//         }
//         foundHandler?.(data);
//     }
// }
//
// /** Make the dragging div visible and position it according to client (x, y) relative to viewport */
// export const updateDraggingDiv = (
//     draggingRef: React.RefObject<HTMLDivElement>,
//     x: number,
//     y: number,
// ) => {
//     const dragging = draggingRef.current;
//     if (dragging) {
//         // this is correct because the container div should always be the same
//         // size as the viewport. Otherwise we need to adjust for the client
//         // rect for the container div
//         dragging.style.top = `${y - 36}px`;
//         dragging.style.left = `${x - 36}px`;
//         dragging.style.display = "unset";
//     }
// };
//
// /** Hide the dragging div */
// export const hideDraggingDiv = (draggingRef: React.RefObject<HTMLDivElement>) => {
//     const dragging = draggingRef.current;
//     if (dragging) {
//         dragging.style.display = "none";
//     }
// };
