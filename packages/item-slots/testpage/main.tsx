import { createRoot } from "react-dom/client";

import { Sprite } from "skybook-item-sprites";


const App: React.FC = () => {
    const actor = "Weapon_Sword_070";
    return (
        <div style={{backgroundColor: "#222"}}>
            <Sprite actor={actor}/>
            <Sprite actor={actor} lowRes/>
            <Sprite actor={actor} deactivated/>
            <Sprite actor={actor} powered/>
            <Sprite actor={actor} deactivated lowRes/>
            <Sprite actor={actor} powered lowRes/>
            <Sprite actor={"Item_Cook_C_17"} effect="ExGutsMaxUp"/>
            <Sprite actor={"Item_Cook_C_17"} effect="ExGutsMaxUp" lowRes/>
        </div>
    );
};

const root = document.getElementById('root');
if (root) {
    createRoot(root).render(<App />);
}

