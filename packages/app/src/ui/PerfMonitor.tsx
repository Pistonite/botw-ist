import { memo } from "react";
import { Text, Button, Tooltip } from "@fluentui/react-components";
import { TopSpeed20Regular } from "@fluentui/react-icons";
import numeral from "numeral";

import { useSessionStore } from "self::application/store";

const PerfMonitorImpl: React.FC = () => {
    const ips = useSessionStore(state => state.perfIps);
    const sps = useSessionStore(state => state.perfSps);
    return (
        <Tooltip relationship="description" content={
            <div>
                <Text font="monospace" block>IPS: {format(ips)}</Text>
                <Text font="monospace" block>SPS: {format(sps)}</Text>
            </div>
        }
            withArrow
            positioning="below"
            appearance="inverted"
        >
        <Button appearance="transparent" icon={<TopSpeed20Regular />}/>
        </Tooltip>
    );
}

const format = (x: number) => {
    if (x < 0.0001) {
        return "---";
    }
    return numeral(x).format("0.00a");
}

export const PerfMonitor = memo(PerfMonitorImpl);
