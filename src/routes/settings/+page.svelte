<script lang="ts">
	import {displayWeightUnit} from "../../types";

	const { data } = $props();

	let availableWeights = $state(data.availableWeights);
	let weightUnit = $state(data.weightUnit);
	let weights = $state(weightUnit === "Kg" ? data.weights.kg : data.weights.lb);

	const updateWeightUnit = async () => {
		console.log(weightUnit)
	}

	$effect(() => {
		weights = weightUnit === "Kg" ? data.weights.kg : data.weights.lb;
	})
</script>
<div class="w-full flex flex-col justify-center items-center my-3">
	<h1 class="mb-3 font-bold">Settings</h1>
	<div class="flex w-11/12 flex-col">
		<div class="divider divider-start">Weight unit</div>
		<select class="select select-bordered w-full max-w-xs" bind:value={weightUnit} onchange={updateWeightUnit}>
			<option value="Lb">Lb</option>
			<option value="Kg">Kg</option>
		</select>
		<div class="divider divider-start">Available weights</div>
		{#each weights as weight}
			<label class="label cursor-pointer">
				<span class="label-text">{weight}{displayWeightUnit(weightUnit, weight)}</span>
				<input type="checkbox" checked={availableWeights.some(x => x === weight)} class="checkbox" />
			</label>
		{/each}
	</div>
</div>