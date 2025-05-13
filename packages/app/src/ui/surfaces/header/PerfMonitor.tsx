import { memo } from "react";
import { Text, Button, Tooltip, makeStyles } from "@fluentui/react-components";
import { TopSpeed20Regular } from "@fluentui/react-icons";
import numeral from "numeral";

import { useSessionStore } from "self::application/store";

const useStyles = makeStyles({
    line: {
        margin: 0,
    },
});

const PerfMonitorImpl: React.FC = () => {
    const ips = useSessionStore((state) => state.perfIps);
    const sps = useSessionStore((state) => state.perfSps);
    const stepIndex = useSessionStore((state) => state.stepIndex);
    const bytePos = useSessionStore((state) => state.bytePos);
    const contents = [
        `CurrStep  ${stepIndex}`,
        `BytePos   ${bytePos}`,
        `Insn/Sec  ${format(ips)}`,
        `Step/Sec  ${format(sps)}`,
    ];
    const c = useStyles();
    return (
        <Tooltip
            relationship="description"
            content={
                <div>
                    {contents.map((content, i) => (
                        <Text key={i} font="monospace" block>
                            <pre className={c.line}>{content}</pre>
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

const format = (x: number) => {
    if (x < 0.0001) {
        return "---";
    }
    return numeral(x).format("0.00a");
};

export const PerfMonitor = memo(PerfMonitorImpl);
