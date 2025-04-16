<script lang="ts">
	import { COMMANDS } from "../../constants";
	import { invoke } from "@tauri-apps/api/core";
	import {displayWeightUnit} from "../../types";

	const { data } = $props();

	let weightUnit = $state(data.weightUnit);
	let weights = $state(weightUnit === "kg" ? data.weights.kg : data.weights.lb);

	const updateWeightUnit = async () => {
		const request = { unit: weightUnit[0].toUpperCase() + weightUnit[1] };
		const response: string = await invoke(COMMANDS.CHANGE_WEIGHT_UNIT, request);
		weightUnit = response.toLowerCase();
		weights = weightUnit === "kg" ? data.weights.kg : data.weights.lb;
	};
</script>
<div class="w-full flex flex-col justify-center items-center my-3">
	<h1 class="mb-3 font-bold">Settings</h1>
	<div class="flex w-11/12 flex-col">
		<div class="divider divider-start">Weight unit</div>
		<select class="select select-bordered w-full max-w-xs self-end" bind:value={weightUnit} onchange={updateWeightUnit}>
			<option value="lb">Lb</option>
			<option value="kg">Kg</option>
		</select>
	</div>
	<div class="flex w-11/12 flex-col">
		<div class="divider divider-start">Available plates</div>
		<div class="self-end">
			{#each weights as weight}
				<div>
					<input type="checkbox" name={weight.toString()} value="Bike">
					<label for={weight.toString()}>{weight}{displayWeightUnit(weightUnit, weight)}</label>
				</div>
			{/each}
		</div>
	</div>
</div>