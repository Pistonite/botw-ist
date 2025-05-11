import { useState } from "react";

/**
 * The Crash Screen
 *
 * At this screen, the app is in an unstable state, so we try
 * to limit the number of components this depends on.
 *
 * The user has the option to change the script (in case
 * they get stuck in a crash loop), then they must restart the app.
 */
export const CrashScreen: React.FC = () => {
    const localization = useCrashLocalization();
    const [script, setScript] = useState(readApplicationScriptInCrash);
    return (
        <div
            className="SpecialScreen"
            style={{
                fontFamily: `-apple-system, BlinkMacSystemFont, "Segoe UI", "Roboto", "Oxygen", "Ubuntu", "Cantarell", "Fira Sans", "Droid Sans", "Helvetica Neue", sans-serif`,
                textAlign: "center",
                position: "absolute",
                top: 0,
                bottom: 0,
                left: 0,
                right: 0,
                backgroundColor: "#010101",
            }}
        >
            <div
                style={{
                    color: "#ffffff",
                    fontSize: "40pt",
                    position: "absolute",
                    top: "20px",
                    left: "calc( 50% - 40px )",
                    right: "calc( 50% - 40px )",
                    border: "2px solid #ffffff",
                }}
            >
                !
            </div>
            <div
                style={{
                    textAlign: "center",
                    position: "absolute",
                    top: 0,
                    bottom: 0,
                    left: 0,
                    right: 0,
                    color: "#ffffff",
                    marginTop: "10vh",
                    height: "100vh",
                }}
            >
                <p>
                    {localization.title} Please report on{" "}
                    <a
                        href="https://github.com/Pistonite/botw-ist/issues"
                        target="_blank"
                        rel="noreferrer"
                    >
                        GitHub
                    </a>
                </p>
                <p>{localization.desc}</p>
                <div>
                    <textarea
                        style={{ width: "80vw" }}
                        value={script}
                        onChange={(e) => {
                            setScript(e.target.value);
                        }}
                        rows={20}
                    />
                </div>
                <div
                    style={{
                        display: "flex",
                        gap: "4px",
                        justifyContent: "center",
                    }}
                >
                    <button
                        onClick={() => {
                            setRecoveryScript(script);
                            window.location.reload();
                        }}
                    >
                        {localization.button_save_reload}
                    </button>
                    <button
                        onClick={() => {
                            window.location.reload();
                        }}
                    >
                        {localization.button_reload}
                    </button>
                </div>
            </div>
        </div>
    );
};

const useCrashLocalization = () => {
    const localization = {
        title: "A fatal error occured in Skybook.",
        desc: "If you were using the local script, you can edit it below to try to prevent a crash loop",
        button_save_reload: "Save script and reload",
        button_reload: "Reload without changing script",
    };
    try {
        const local = localStorage.getItem("Skybook.CrashScreenLocalization");
        if (local) {
            const parsed = JSON.parse(local);
            return {
                ...localization,
                ...parsed,
            };
        }
    } catch (e) {
        console.error("Error reading crash localization", e);
    }
    return localization;
};

const readApplicationScriptInCrash = () => {
    try {
        const local = localStorage.getItem("Skybook.Application");
        if (!local) {
            return "";
        }
        const application = JSON.parse(local);
        return application?.state?.savedScript || "";
    } catch (e) {
        console.error("Error reading application script in crash", e);
        return "#### Error reading application script in crash ####";
    }
};

const setRecoveryScript = (script: string) => {
    localStorage.setItem("Skybook.RecoveryScript", script);
};
