import { ItemStack } from "./ItemStack";

export type ItemListProps = {
    items: {name: string, count: number}[],
    numBroken: number
}

export const ItemList: React.FC<ItemListProps> = ({items, numBroken}) => {
    return <>
    {
        items.map(({name, count}, i)=>{
            const broken = i+numBroken >= items.length;
            return <ItemStack key={i} name={name} count={count} isBroken={broken}/>
        })
    }
    </>;
}

