import { useEffect, useMemo, useRef, useState } from "react";
import { Button, Category, Control, Description, Label } from "ui/components";
import { Page, CommandTextArea } from "ui/surfaces";
import { MemoizedParser } from "core/command";
import { useRuntime } from "core/runtime";
import { saveAs } from "data/saveAs";
import { useSearchItem } from "data/item";
import { serialize } from "data/storage";
import { arrayShallowEqual } from "data/util";

const URL_MAX = 2048;

const parser = new MemoizedParser();

export const ScriptOptionPanel: React.FC = () => {
    const {
        editing,
        saving,
        commandData,
        setCommandData,
        enableEditing,
        setSaving,
    } = useRuntime();
    //const commandText = useMemo(()=>commandData.join("\n"), [commandData]);

    const [currentText, setCurrentText] = useState<string[]>(commandData);
    const [fileName, setFileName] = useState<string>("");
    const [showDirectUrl, setShowDirectUrl] = useState<boolean>(false);
    const [showCopiedMessage, setShowCopiedMessage] = useState<boolean>(false);

    const searchItem = useSearchItem();
    const codeblocks = useMemo(() => {
        const commands = parser.parseCommands(currentText, searchItem);
        return commands.map((c) => c.codeBlocks);
    }, [currentText, searchItem]);

    const uploadRef = useRef<HTMLInputElement>(null);

    const directUrl = useMemo(() => {
        const serializedCommands = serialize(currentText.join("\n"));
        const query = new URLSearchParams(serializedCommands).toString();
        // hard-coding the "legacy" path here
        return `${window.location.origin}/legacy/?${query}`;
    }, [currentText]);

    const directUrlLength = directUrl.length;

    useEffect(() => {
        setShowCopiedMessage(false);
    }, [currentText]);

    const unsaved = useMemo(
        () => !arrayShallowEqual(currentText, commandData),
        [currentText, commandData],
    );

    return (
        <Page title="Script Options">
            <input
                ref={uploadRef}
                id="Upload"
                type="File"
                hidden
                onChange={(e) => {
                    const files = e.target.files;
                    if (files?.length && files[0]) {
                        const file = files[0];
                        const fileName = file.name.endsWith(".txt")
                            ? file.name.substring(0, file.name.length - 4)
                            : file.name;
                        setFileName(fileName);
                        file.text().then((text) => {
                            const splitted = text
                                .replaceAll("\r", "")
                                .split("\n");
                            setCurrentText(splitted);
                            setCommandData(splitted);
                        });
                    }
                }}
            />
            <Category title="Editing and Saving">
                <Control disabled={editing}>
                    <Button className="Full" onClick={enableEditing}>
                        Enable
                    </Button>
                    <Label>
                        {editing
                            ? "Editing is already enabled"
                            : "Editing is disabled"}
                    </Label>
                    <Description>
                        {!editing &&
                            "Editing is disabled to prevent modifying the script by accident. Click \"Enable\" to allow editing. Saving won't be enabled so your own script won't be overriden"}
                    </Description>
                </Control>
                <Control disabled={!editing}>
                    <Button className="Full" onClick={() => setSaving(!saving)}>
                        {saving ? "Disable" : "Enable"}
                    </Button>
                    <Label>
                        {saving ? "Saving is enabled" : "Saving is disabled"}
                    </Label>
                    {!editing && (
                        <Description className="Primary">
                            Editing must be enabled first to change if saving
                            should be enabled
                        </Description>
                    )}
                    {editing && window.location.search && (
                        <Description className="Primary Important">
                            Warning: It looks like you might be editing a script
                            from a direct URL. If you enable save, the script
                            will overwrite the existing saved script.
                        </Description>
                    )}
                    <Description>
                        When saving is enabled, the script is saved as you make
                        changes. You can disable saving when making experimental
                        changes.
                    </Description>
                </Control>
            </Category>

            <Category title="Import and Export">
                <Control disabled={!editing}>
                    <Button
                        className="Full Action"
                        disabled={!unsaved}
                        onClick={() => {
                            setCommandData(currentText);
                        }}
                    >
                        Save
                    </Button>
                    <Button
                        className="Full"
                        onClick={() => {
                            if (uploadRef.current) {
                                uploadRef.current.click();
                            }
                        }}
                    >
                        Import
                    </Button>
                    <Button
                        className="Full"
                        onClick={() => {
                            saveAs(
                                currentText.join("\n"),
                                fileName + ".txt" || "IST-Export.txt",
                            );
                        }}
                    >
                        Export
                    </Button>
                    <input
                        style={{ width: "calc( 100% - 340px )" }}
                        className="MainInput"
                        spellCheck={false}
                        value={fileName}
                        onChange={(e) => {
                            setFileName(e.target.value);
                        }}
                        placeholder="File name"
                    />
                    {/* <div> */}
                    <CommandTextArea
                        className="MainInput MultiLineInput"
                        value={currentText}
                        setValue={setCurrentText}
                        blocks={codeblocks}
                        stopPropagation
                    />
                    {/* </div> */}

                    <Description className="Error">
                        {editing
                            ? ""
                            : "You need to enable editing to change the script here"}
                    </Description>
                </Control>

                <Description className="Primary Paragraph">
                    You can use a direct URL to open the simulator with the
                    steps automatically loaded.
                </Description>
                {unsaved && (
                    <Description className="Primary Important">
                        Warning: The script above contains unsaved changes. The
                        URL will reflect the unsaved script instead of what is
                        in the simulator.
                    </Description>
                )}
                {directUrlLength > URL_MAX && (
                    <Description className="Primary Important">
                        Warning: The URL is too long ({directUrlLength}{" "}
                        characters) and may not work in certain browsers. Export
                        as file instead if you encounter any problems.
                    </Description>
                )}
                <Button
                    onClick={() => {
                        window.navigator.clipboard.writeText(directUrl);
                        setShowCopiedMessage(true);
                    }}
                >
                    Copy Direct URL
                </Button>
                {showCopiedMessage && (
                    <Label className="Highlight">Link copied!</Label>
                )}

                <Description />
                <Description className="Primary">
                    If the copy button is not working properly, you can also
                    expand and manually copy the URL below
                </Description>
                <Button
                    className="Full"
                    onClick={() => {
                        setShowDirectUrl(!showDirectUrl);
                    }}
                >
                    {showDirectUrl ? "Hide" : "Expand"}
                </Button>

                <Description
                    style={{
                        wordBreak: "break-all",
                        //
                        ...(!showDirectUrl && {
                            width: "calc( 90vw - 400px )",
                            textOverflow: "ellipsis",
                            overflowX: "hidden",
                            whiteSpace: "nowrap",
                        }),
                    }}
                >
                    {directUrl}
                </Description>
            </Category>
        </Page>
    );
};
