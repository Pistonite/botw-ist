import { createRoot } from "react-dom/client";

import { Sprite } from "skybook-item-sprites";


const App: React.FC = () => {
    const actor = "Weapon_Sword_070";
    return (
        <div style={{
            backgroundColor: "#000c",
            display: "flex",
            alignContent: "flex-start",
            flexWrap: "wrap",
            gap: "8px",
        }}>
            <Sprite actor={actor} badlyDamaged/>
            <Sprite actor={actor} cheap/>
            <Sprite actor={"Weapon_Lsword_031"} badlyDamaged/>
            <Sprite actor={"Weapon_Lsword_031"} cheap/>
            <Sprite actor={actor} deactive/>
            <Sprite actor={actor} powered badlyDamaged/>
            <Sprite actor={actor} deactive cheap/>
            <Sprite actor={actor} powered cheap/>
            <Sprite actor={"Item_Cook_C_17"} effect="ExGutsMaxUp"/>
            <Sprite actor={"Item_Cook_C_17"} effect="ExGutsMaxUp" cheap/>
            <Sprite actor={"Obj_HeroSoul_Gerudo"} />
            <Sprite actor={"Obj_HeroSoul_Gerudo"} cheap />
            <Sprite actor={"Obj_HeroSoul_Gerudo"} deactive />
            <Sprite actor={"Obj_HeroSoul_Gerudo"} deactive cheap />
            <Sprite actor={"Obj_WarpDLC"} />
            <Sprite actor={"Obj_WarpDLC"} cheap />
            <Sprite actor={"Obj_DLC_HeroSoul_Zora"} />
            <Sprite actor={"Obj_DLC_HeroSoul_Zora"} cheap />
            <Sprite actor={"Obj_DLC_HeroSoul_Zora"} deactive />
            <Sprite actor={"Obj_DLC_HeroSoul_Zora"} deactive cheap />
            <Sprite actor={"Obj_DungeonClearSeal"} />
            <Sprite actor={"Obj_DungeonClearSeal"} cheap />
            <Sprite actor={"Obj_DungeonClearSeal"} blank/>
            <Sprite actor={"Obj_DungeonClearSeal"} cheap blank/>
        </div>
    );
};

const root = document.getElementById('root');
if (root) {
    createRoot(root).render(<App />);
}

