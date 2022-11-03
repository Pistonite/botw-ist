import { BodyText, SubHeader, SubTitle } from "components/Text";
import { Section } from "ui/components/Section";
import { saveAs } from "data/FileSaver";
import { serialize } from "data/serialize";
import { useEffect, useMemo, useRef, useState } from "react";
import { Button, Category, Description, Label, Separator } from "ui/components";
import { Version } from "data/const";
import { Page } from "ui/surfaces";

//Category
//Label
//Description
export const SettingPage: React.FC = () => {
	return (
        <Page title="Setting">
            <Category title="Layout">
                <Button>ON</Button>
                <Label>Show Save Files</Label>
                <Description>
                    Show the Saves panel in simulation tab.
                    Useful if your script utilizes more than one saves.
                    Otherwise you can turn it off to have more space.
                </Description>
                <Separator />
                <Button>ON</Button>
                <Label>Show Game Data</Label>
                <Description>
                    Show the Game Data panel in simulation tab.
                    Useful in situations where Game Data is desynced from visible inventory,
                    which is the core mechanics behind inventory corruption.
                    You can turn it off if it is too confusing. Otherwise you should leave it on.
                </Description>
                <Button>ON</Button>
                <Label>Interlace Game Data with Inventory</Label>
                <Description>
                    Merge the Game Data Panel with Visible Inventory Panel,
                    so you could see how the slots correspond.
                    Has no effect if Game Data Panel is hidden.
                </Description>
            </Category>
            <Category title="Items">
            <Button>ON</Button>
                <Label>Show Animated Icons</Label>
                <Description>
                    Show animated icons for available items.
                </Description>
            </Category>
            <Category title="Advanced">
            <Button>OFF</Button>
                <Label>Superuser Mode</Label>
                <Description>
                    Enable editing of all commands.
                    Some commands are simplified for non-advanced users and are read-only
                    unless in superuser mode
                </Description>
            </Category>
        </Page>

	);
};
