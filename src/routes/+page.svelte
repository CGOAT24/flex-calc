<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { COMMANDS } from "../constants";
  import {type CalcResponse, displayWeightUnit, type PlateCount, weightUnitToEnum} from "../types";
  import { error } from '@tauri-apps/plugin-log';
  import Barbell from "../component/Barbell/Barbell.svelte";

  const { data } = $props();

  let totalWeight = $state(data.totalWeight);
  let barWeight = $state(data.barWeight);
  let weightUnit = $state(data.weightUnit);

  let result: PlateCount[] = $state([]);
  let errorMessage: string | undefined = $state(undefined);

  const calc = async () => {
    errorMessage = undefined;
    if (totalWeight > 0) {
      try {
        const request = { total_weight: totalWeight, bar_weight: barWeight, unit: weightUnitToEnum(weightUnit) };
        const response: CalcResponse = await invoke(COMMANDS.CALC_WEIGHTS, { request });
        result = response.plates.toSorted((x, y) => y.weight - x.weight);
      }
      catch (err) {
        errorMessage = String(err);
        await error(errorMessage);
        result = [];
      }
    }
    else {
      result = [];
    }
  }
</script>
<div class="w-full flex flex-col justify-center items-center my-3">
  <h1 class="mb-3 font-bold">Weight to plates</h1>
  <div class="w-11/12">
    <h2>Weight:</h2>
    <label class="input input-bordered input-lg flex items-center justify-between w-full">
      <input bind:value={totalWeight} type="number" step="5" class="[appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"/>
      {displayWeightUnit(weightUnit)}
    </label>
  </div>
  <div class="w-11/12 my-2">
    <h2>Bar:</h2>
    <label class="input input-bordered input-lg flex items-center justify-between w-full">
      <input bind:value={barWeight} type="number" step="5" class="[appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"/>
      {displayWeightUnit(weightUnit)}
    </label>
  </div>
  <button onclick={calc} class="btn btn-primary w-11/12 mt-2">Calculate</button>
</div>
{#if result.length > 0}
  {@const plates = result.flatMap(x => Array(x.count / 2).fill(Number(x.weight)))}
  <Barbell {plates} unit={weightUnit}/>
{:else if errorMessage}
  {errorMessage}
{/if}