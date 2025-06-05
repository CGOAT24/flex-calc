<script lang="ts">
  const {plate, unit}: {plate: number, unit: string} = $props();

  const getStyle = () => {
    const key = `${plate}${unit}`;
    if (key === "2.5lb" || key === "1.25kg") {
      return `bg-[#777] w-[20px]`;
    }
    if (key === "5lb" || key === "2.5kg") {
      return `bg-[#666] w-[25px]`;
    }
    if (key === "10lb" || key === "5kg") {
      return `bg-[#555] w-[30px]`;
    }
    if (key === "25lb" || key === "10kg") {
      return `bg-[#444] w-[35px]`;
    }
    if (key === "35lb" || key === "15kg") {
      return `bg-[#333] w-[40px]`;
    }
    return `bg-[#222] w-[45px]`;
  }

</script>

<div class={`flex items-center justify-center h-[60px] rounded-md shadow-inner text-xs font-bold text-white ${getStyle()}`}>
    {plate}
</div>