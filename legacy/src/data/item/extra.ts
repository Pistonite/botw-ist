import { CookEffect, ExData } from "./type";

export class ExDataImpl implements ExData {
    public hearts = 0;
    public sellPrice = 0;
    public cookEffect = CookEffect.None;
    public get modifierType(): number {
        return this.sellPrice;
    }
    public set modifierType(v: number) {
        this.sellPrice = v;
    }
    public get modifierValue(): number {
        return this.hearts;
    }
    public set modifierValue(v: number) {
        this.hearts = v;
    }
}
