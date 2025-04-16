<script lang="ts">
	import { page } from '$app/state';
	import "../app.css";

    const { children } = $props();

    let currentRoute: string | null = $state(null);

	$effect(() => {
      currentRoute = page.route.id;
    })
</script>
<main class="w-full flex flex-col justify-center items-center">
    {@render children?.()}
</main>
<div class="btm-nav">
    <a href="/" class:active={currentRoute === "/"}><img src="/icons/calculate.svg" class="" alt="calculator"></a>
    <a href="/1rm" class:active={currentRoute === "/1rm"}><img src="/icons/star.svg" class="" alt="1rm calculator"></a>
    <a href="/settings" class:active={currentRoute === "/settings"}><img src="/icons/settings.svg" class="" alt="settings"></a>
</div>
