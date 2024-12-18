import { LazyStore } from "@tauri-apps/plugin-store";
import {DATA, DEFAULT, STORE} from "../constants";

export const load = async () => {
	const store = new LazyStore(STORE);
	const totalWeight: number = (await store.get(DATA.TOTAL_WEIGHT)) ?? DEFAULT.TOTAL_WEIGHT;
	const barWeight: number = (await store.get(DATA.BAR_WEIGHT)) ?? DEFAULT.BAR_WEIGHT;
	const weightUnit: string = (await store.get(DATA.WEIGHT_UNIT)) ?? DEFAULT.WEIGHT_UNIT;

	return { totalWeight, barWeight, weightUnit };
};