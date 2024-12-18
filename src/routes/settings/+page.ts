import { LazyStore } from "@tauri-apps/plugin-store";
import { DATA, DEFAULT, STORE, WEIGHTS } from "../../constants";

export const load = async () => {
	const store = new LazyStore(STORE);

	const weightUnit: string = (await store.get(DATA.WEIGHT_UNIT)) ?? DEFAULT.WEIGHT_UNIT;
	const weights: { kg: number[], lb: number[] } = { kg: WEIGHTS.KG, lb: WEIGHTS.LB };
	const availableWeights: number[] = (await store.get(DATA.AVAILABLE_WEIGHTS)) ?? [];

	return { weightUnit, weights, availableWeights };
};