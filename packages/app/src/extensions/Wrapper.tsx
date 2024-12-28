import { useQuery } from "@tanstack/react-query";
import { getExtensionComponent } from "./registry";

export type ExtensionWrapperProps = {
    id: string;
}

export const ExtensionWrapper: React.FC<ExtensionWrapperProps> = ({ id }) => {
    const {isPending, data: ExtComp } = useQuery({
        queryKey: ["extension", id],
        queryFn: () => {
            return getExtensionComponent(id);
        },
    });
    if (isPending || !ExtComp) {
        return <div>Loading...</div>
    }
    return <ExtComp />;
}
