<script lang="ts">
	import '../app.css';

	let { children } = $props();

	let innerWidth = $state(0);
	let innerHeight = $state(0);

	const getZoomScale = (width: number, height: number) => {
		const baseWidth = 1280;
		const baseHeight = 720;

		const scaleX = width / baseWidth;
		const scaleY = height / baseHeight;
		const scale = Math.min(scaleX, scaleY);

		return scale;
	};
	const zoomScale = $derived(getZoomScale(innerWidth, innerHeight));
</script>

<svelte:window bind:innerWidth bind:innerHeight />

<div class="flex h-[720px] w-[1280px]" style="transform: scale({zoomScale});">
	{@render children()}
</div>
