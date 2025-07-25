import { LazyStore } from "@tauri-apps/plugin-store";
import { DATA, DEFAULT, STORE } from "../../constants";

export const load = async () => {
	const store = new LazyStore(STORE);

	const weight = (await store.get(DATA.ONE_RM_WEIGHT)) ?? 0;
	const reps = (await store.get(DATA.ONE_RM_REPS)) ?? 0;
	const weightUnit = DEFAULT.WEIGHT_UNIT;

	return { reps, weight, weightUnit };
};