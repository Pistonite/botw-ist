import { PropsWithChildren } from "react";

type Props = {
    multiLine?: boolean;
    hasError?: boolean;
};

// an over-engineered loading screen
export const LoadingScreen: React.FC<PropsWithChildren<Props>> = ({
    multiLine,
    hasError,
    children,
}) => {
    return (
        <div
            style={{
                textAlign: "center",
                position: "absolute",
                top: 0,
                bottom: 0,
                left: 0,
                right: 0,
                backgroundColor: "#262626",
            }}
        >
            <span
                style={{
                    color: hasError ? "#ee7777" : "#00ffcc",

                    lineHeight: multiLine ? "default" : "100vh",
                    height: "100vh",
                }}
            >
                {children}
            </span>
        </div>
    );
};
