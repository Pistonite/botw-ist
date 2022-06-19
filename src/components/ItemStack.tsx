type ItemStackProps = {
  name: string,
  count: number,
  isBroken: boolean
};

export const ItemStack: React.FC<ItemStackProps> = ({name, count, isBroken})=>{
	return <span>[{name} x{count}]{isBroken && " (broken)"}&nbsp;<br/></span>;
};
