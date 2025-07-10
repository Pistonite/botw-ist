import { memo } from "react";
import { Text, Button, Tooltip } from "@fluentui/react-components";
import { TopSpeed20Regular } from "@fluentui/react-icons";

import { useSessionStore } from "self::application";
import { useStyleEngine } from "self::util";

const PerfMonitorImpl: React.FC = () => {
    const stepIndex = useSessionStore((state) => state.stepIndex);
    const bytePos = useSessionStore((state) => state.bytePos);
    const contents = [`CurrStep  ${stepIndex}`, `BytePos   ${bytePos}`];
    const m = useStyleEngine();
    return (
        <Tooltip
            relationship="description"
            content={
                <div>
                    {contents.map((content, i) => (
                        <Text key={i} font="monospace" block>
                            <pre className={m("margin-0")}>{content}</pre>
                        </Text>
                    ))}
                </div>
            }
            withArrow
            positioning="below"
            appearance="inverted"
        >
            <Button appearance="transparent" icon={<TopSpeed20Regular />} />
        </Tooltip>
    );
};

export const PerfMonitor = memo(PerfMonitorImpl);
