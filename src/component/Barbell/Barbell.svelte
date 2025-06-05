<script lang="ts">
    import Plate from "./Plate.svelte";

    const { plates, unit }: { plates: number[], unit: string } = $props();
</script>
<div class="relative flex items-center h-[100px]">
    <div class="absolute left-0 top-1/2 -translate-y-1/2 flex flex-row-reverse space-x-0.5 space-x-reverse">
        {#each plates as plate}
            <Plate {plate} {unit} />
        {/each}
    </div>
    <div class="w-[300px] h-[20px] bg-gray-600 -z-10"></div>
</div>