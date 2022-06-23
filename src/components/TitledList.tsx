import { PropsWithChildren } from "react"

type TitledListProps = PropsWithChildren<{
    title: string
}>

export const TitledList: React.FC<TitledListProps> = ({title, children}) => {
    return (
        <>
            <h3 className="ListHeader" style={{
                height: 20,
                border: "1px solid red",
                boxSizing: "content-box"
            }}>
                {title}
            </h3>
            <div style={{ 
                height: "calc( 100% - 40px )",
                overflowY: "auto"}}
            >
                {children}
            </div>
        </>
    );
}