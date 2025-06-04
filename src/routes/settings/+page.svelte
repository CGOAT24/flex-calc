<script lang="ts">
	import { COMMANDS } from "../../constants";
	import { invoke } from "@tauri-apps/api/core";
	import {displayWeightUnit, type PlateSetting, type Setting} from "../../types";

	let setting: Setting | undefined = $state();
	let weightUnit = $state("");
	let weights: PlateSetting[] = $state([]);

	const fetch = async () => {
		setting = await invoke(COMMANDS.GET_SETTINGS);
		weightUnit = setting?.weight_unit.toLowerCase() ?? "";
		weights = setting?.plates ?? [];
	}

	const updateWeightUnit = async () => {
		const request = { unit: weightUnit[0].toUpperCase() + weightUnit[1] };
		const response: string = await invoke(COMMANDS.CHANGE_WEIGHT_UNIT, request);
		weightUnit = response.toLowerCase();
	};

	const togglePlate = async (event) => {

	}
</script>
{#await fetch()}
	Loading...
{:then _}
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
			<div class="flex flex-row justify-around">
				<div>
					{#each weights as plate, i}
						<div>
							<input type="checkbox" name={plate.weight+plate.weight_unit} bind:checked={weights[i].enabled} onchange={togglePlate}>
							<label for={plate.weight+plate.weight_unit}>{plate.weight}{displayWeightUnit(plate.weight_unit, plate.weight)}</label>
						</div>
					{/each}
				</div>
				<!--
				<div>
					{#each weights.filter(x => x.weight_unit === "Kg") as plate}
						<div>
							<input type="checkbox" name={plate.weight+plate.weight_unit} bind:checked={plate.enabled} onchange={() => togglePlate(plate)}>
							<label for={plate.weight+plate.weight_unit}>{plate.weight}{displayWeightUnit(plate.weight_unit, plate.weight)}</label>
						</div>
					{/each}
				</div>
				-->
			</div>
			<div class="self-start">

			</div>
			<div class="self-end">

			</div>
		</div>
	</div>
{/await}
