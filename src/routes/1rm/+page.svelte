<script lang="ts">
  import {displayWeightUnit, type Setting} from "../../types";
    import { COMMANDS } from "../../constants";
    import { invoke } from "@tauri-apps/api/core";
    const { data } = $props();

    let weight = $state(data.weight);
    let reps = $state(data.reps);
    let oneRM = $state(0);
    let weightUnit = $state(data.weightUnit);

    const calc = async () => {
      const request = { weight, reps };
      oneRM = +((await invoke(COMMANDS.ONE_RM, request)) as number).toFixed(1);
    }

    const fetch = async () => {
      const setting: Setting = await invoke(COMMANDS.GET_SETTINGS);
      weightUnit = setting.weight_unit;
    }
</script>
{#await fetch()}
    Loading...
{:then _}
<div class="w-full flex flex-col justify-center items-center my-3">
    <h1 class="mb-3 font-bold">1RM Calculator</h1>
    <label class="input input-bordered input-lg flex items-center justify-between w-11/12">
        <input bind:value={weight} type="number" step="5" class="[appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"/>
        {displayWeightUnit(weightUnit)}
    </label>
    <label class="input input-bordered input-lg flex items-center justify-between w-11/12 my-2">
        <input bind:value={reps} type="number" class="[appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none"/>
        Reps
    </label>
    <button onclick={calc} class="btn btn-primary w-11/12">Calculate</button>
    <div class="w-full flex items-center justify-center my-5">
        {#if oneRM > 0}
            Your 1RM is {oneRM} {displayWeightUnit(weightUnit)}
        {/if}
    </div>
</div>
{/await}