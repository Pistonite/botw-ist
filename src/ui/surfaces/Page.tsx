import { PropsWithChildren } from "react"
import { Section } from "ui/components"

type PageProps = {
    title: string
}

export const Page: React.FC<PropsWithChildren<PageProps>> = ({title, children}) => {
    return (
        <Section className="Page" title={title}>
            <div className="PageContent">
                {children}
            </div>
        </Section>
    );
}
