import { createMockItems } from "data/test";
import { getElixir, addElixir } from "./elixir";
import { CookEffect } from "./type";

describe("data/item/elixir", ()=>{
	const Elixirs = [
		"FoodElixir",
		"FoodHeartyElixir",
		"FoodEnergizingElixir",
		"FoodEnduringElixir",
		"FoodHastyElixir",
		"FoodFireproofElixir",
		"FoodSpicyElixir",
		"FoodChillyElixir",
		"FoodElectroElixir",
		"FoodMightyElixir",
		"FoodToughElixir",
		"FoodSneakyElixir",
	];
	const Effects = [
		CookEffect.None,
		CookEffect.Hearty,
		CookEffect.Energizing,
		CookEffect.Enduring,
		CookEffect.Hasty,
		CookEffect.Fireproof,
		CookEffect.Chilly,
		CookEffect.Spicy,
		CookEffect.Electro,
		CookEffect.Mighty,
		CookEffect.Tough,
		CookEffect.Sneaky,
	];
	it("converts elixirs correctly", ()=>{

		expect(Elixirs.length).toEqual(Effects.length);
		const items = createMockItems(Elixirs);
		for(let i=0;i<Elixirs.length;i++){
			addElixir(items[Elixirs[i].toLowerCase()], Effects[i]);
		}

		for(let i=0;i<Elixirs.length;i++){
			const convertTo = Elixirs[i];
			const convertEffect = Effects[i];

			const actual = getElixir(convertEffect);
			expect(actual).toBe(items[convertTo.toLowerCase()]);
		}

	});

});
