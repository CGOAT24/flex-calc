<script lang="ts">
  const {plate, unit}: {plate: number, unit: string} = $props();
</script>

<div class={`flex items-center justify-center h-[60px] rounded-md shadow-inner text-xs font-bold text-white plate`} data-size="{plate}{unit}">
    {plate}
</div>

<style>
    .plate[data-size="2.5lb"] { width: 20px; background: #777; }
    .plate[data-size="5lb"]  { width: 25px; background: #666; }
    .plate[data-size="10lb"] { width: 30px; background: #555; }
    .plate[data-size="25lb"] { width: 35px; background: #444; }
    .plate[data-size="35lb"] { width: 40px; background: #333; }
    .plate[data-size="45lb"] { width: 45px; background: #222; }

    .plate[data-size="1.25kg"] { width: 20px; background: #777; }
    .plate[data-size="2.5kg"]  { width: 25px; background: #666; }
    .plate[data-size="5kg"] { width: 30px; background: #555; }
    .plate[data-size="10kg"] { width: 35px; background: #444; }
    .plate[data-size="15kg"] { width: 40px; background: #333; }
    .plate[data-size="20kg"] { width: 45px; background: #222; }
</style>