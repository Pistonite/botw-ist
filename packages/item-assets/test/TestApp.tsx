import { PropsWithChildren } from "react";
import { ActorSprite, ModifierSprite } from "../src";

export const App: React.FC = () => {
    const actors = [
        "Weapon_Sword_070",
        "Weapon_Sword_502",
        "Obj_DungeonClearSeal",
        "Obj_HeroSoul_Gerudo",
        "Obj_DLC_HeroSoul_Gerudo",
        "Obj_WarpDLC",
    ];
    const sizes = [16, 32, 48, 64, 96];
    const modifiers = [
        "AddGuard",
        "AddLife",
        "RapidFire",
        "DNE",
        "ClimbSpeedUp",
    ];
    const modifierSizes = [10, 20, 32, 40, 64];

    return (
        <div
            style={{
                color: "white",
                backgroundColor: "#000c",
            }}
        >
            {actors.map((actor) =>
                sizes.map((size) => (
                    <ActorMatrix key={actor + size} actor={actor} size={size} />
                )),
            )}
            {modifiers.map((modifier) =>
                modifierSizes.map((size) => (
                    <Container key={modifier + size} size={size}>
                        <ModifierSprite status={modifier} size={size} />
                    </Container>
                )),
            )}
        </div>
    );
};

type ActorMatrixProps = {
    actor: string;
    size: number;
};

const ActorMatrix: React.FC<ActorMatrixProps> = ({ actor, size }) => {
    return (
        <>
            <div>
                {actor} {size}px
            </div>
            <div
                style={{
                    display: "flex",
                }}
            >
                <Container size={size}>
                    <ActorSprite actor={actor} size={size} />
                </Container>
                <Container size={size}>
                    <ActorSprite actor={actor} size={size} cheap />
                </Container>
                <Container size={size}>
                    <ActorSprite actor={actor} size={size} disableAnimation />
                </Container>
                <Container size={size}>
                    <ActorSprite actor={actor} size={size} badlyDamaged />
                </Container>
                <Container size={size}>
                    <ActorSprite actor={actor} size={size} cheap badlyDamaged />
                </Container>
                <Container size={size}>
                    <ActorSprite
                        actor={actor}
                        size={size}
                        disableAnimation
                        badlyDamaged
                    />
                </Container>
                <Container size={size}>
                    <ActorSprite actor={actor} size={size} powered />
                </Container>
                <Container size={size}>
                    <ActorSprite actor={actor} size={size} cheap powered />
                </Container>
                <Container size={size}>
                    <ActorSprite
                        actor={actor}
                        size={size}
                        disableAnimation
                        powered
                    />
                </Container>
                <Container size={size}>
                    <ActorSprite
                        actor={actor}
                        size={size}
                        badlyDamaged
                        powered
                    />
                </Container>
                <Container size={size}>
                    <ActorSprite
                        actor={actor}
                        size={size}
                        cheap
                        badlyDamaged
                        powered
                    />
                </Container>
                <Container size={size}>
                    <ActorSprite
                        actor={actor}
                        size={size}
                        disableAnimation
                        badlyDamaged
                        powered
                    />
                </Container>
                <Container size={size}>
                    <ActorSprite actor={actor} size={size} deactive />
                </Container>
                <Container size={size}>
                    <ActorSprite actor={actor} size={size} deactive cheap />
                </Container>
                <Container size={size}>
                    <ActorSprite actor={actor} size={size} blank />
                </Container>
                <Container size={size}>
                    <ActorSprite actor={actor} size={size} blank cheap />
                </Container>
            </div>
        </>
    );
};

const Container: React.FC<PropsWithChildren<{ size: number }>> = ({
    size,
    children,
}) => {
    return (
        <div
            style={{
                border: "1px solid red",
                width: size,
                height: size,
            }}
        >
            {children}
        </div>
    );
};
