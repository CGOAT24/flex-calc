<script lang="ts">
    import {displayWeightUnit} from "../types";
    const {plate = $bindable() } = $props();
</script>
<input type="checkbox" name={plate.weight+plate.weight_unit} bind:checked={plate.enabled} >
<label for={plate.weight+plate.weight_unit}>{plate.weight}{displayWeightUnit(plate.weight_unit, plate.weight)}</label>