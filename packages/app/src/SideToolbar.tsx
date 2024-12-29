import { Card } from "@fluentui/react-components";
import { useIsShowingExtensionPanel } from "application/extensionStore";
import { ExtensionOpenButton } from "ui/ExtensionOpenButton";

export const SideToolbar: React.FC = () => {
    const showingExtensionPanel = useIsShowingExtensionPanel();
    return (
    <Card>
            Hi
            {
                !showingExtensionPanel && <ExtensionOpenButton />
            }
    </Card>
    );
}
