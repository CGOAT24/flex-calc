<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { COMMANDS } from "../constants";
  import {type CalcResponse, displayWeightUnit, type PlateCount, WeightUnit} from "../types";
  import { warn, debug, trace, info, error } from '@tauri-apps/plugin-log';

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
        const request = { total_weight: totalWeight, bar_weight: barWeight, unit: weightUnit };
        const response: CalcResponse = await invoke(COMMANDS.CALC_WEIGHTS, { request });
        result = response.plates.toSorted((x, y) => y.weight - x.weight);
      }
      catch (err) {
        await error(err + "");
        errorMessage = err + "";
        result = [];
      }
    }
    else {
      result = [];
    }
  }
</script>
<div class="w-full flex flex-col justify-center items-center my-3">
  <h1 class="mb-3 font-bold">Plates Calculator</h1>
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
  <table class="table flex flex-col justify-center items-center w-11/12">
    <thead>
    <tr>
      <th>Weight ({displayWeightUnit(weightUnit)})</th>
      <th>Count</th>
    </tr>
    </thead>
    <tbody>
    {#each result as plate}
      <tr>
        <td>{plate.weight}</td>
        <td>{plate.count}</td>
      </tr>
    {/each}
    </tbody>
  </table>
{:else if errorMessage}
  {errorMessage}
{/if}