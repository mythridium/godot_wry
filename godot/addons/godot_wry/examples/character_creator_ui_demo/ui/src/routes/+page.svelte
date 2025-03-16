<script>
	import { cn } from '$lib/utils';
	import { backOut } from 'svelte/easing';
	import { scale } from 'svelte/transition';
	import {
		CheckFat,
		Empty,
		Eyeglasses,
		Eyes,
		Pants,
		Scissors,
		Smiley,
		Sneaker,
		TShirt
	} from 'phosphor-svelte';
	import ColorIcon from '$lib/icons/color-icon.svelte';

	const tabs = [
		{
			label: 'Skin Tone',
			icon: Smiley
		},
		{
			label: 'Hair',
			icon: Scissors
		},
		{
			label: 'Eyes',
			icon: Eyes
		},
		{
			label: 'Top',
			icon: TShirt
		},
		{
			label: 'Bottom',
			icon: Pants
		},
		{
			label: 'Accessories',
			icon: Eyeglasses
		},
		{
			label: 'Shoes',
			icon: Sneaker
		}
	];

	let activeTabIdx = $state(0);

	const colors = ['#696669', '#926F52', '#569FA4', '#8B9977', '#5478BF', '#908F9B'];
	let activeColor = $state(colors[5]);
</script>

<main class="ml-auto flex flex-col items-center justify-between px-40 py-24">
	<ul class="ml-5 flex">
		{#each tabs as tab, i}
			{@const isActive = i === activeTabIdx}
			<li class="relative -ml-5 flex justify-center">
				<button
					class={cn(
						'flex size-20 cursor-pointer items-center justify-center rounded-full bg-cyan-700 text-cyan-100/80 transition-all hover:-translate-y-0.5',
						isActive && 'text-cyan-50'
					)}
					onclick={() => {
						activeTabIdx = i;
						window.ipc.postMessage(
							JSON.stringify({
								type: 'change_tab',
								tab: tab.label.toLowerCase().replaceAll(' ', '_')
							})
						);
					}}
				>
					<tab.icon size={40} weight="bold" />
				</button>
				{#if isActive}
					<span
						class="absolute -top-4 z-20 w-[max-content] rounded-xl bg-cyan-50 px-2.5 py-px text-xl font-extrabold text-cyan-700"
						transition:scale={{ duration: 300, start: 0.75, opacity: 0, easing: backOut }}
					>
						{tab.label}
					</span>
				{/if}
			</li>
		{/each}
	</ul>

	<div class="flex gap-5">
		<button
			class="flex h-20 w-40 cursor-pointer items-center justify-center rounded-4xl bg-cyan-50 text-cyan-700/20 transition-all hover:-translate-y-0.5"
		>
			<Empty size={40} weight="bold" />
		</button>

		<button
			class="relative flex h-20 w-40 cursor-pointer items-center justify-center rounded-4xl bg-cyan-50 text-cyan-700/20 outline-4 outline-rose-500 transition-all hover:-translate-y-0.5"
		>
			<span
				class="absolute -top-1.5 -right-1.5 flex size-7 items-center justify-center rounded-full bg-rose-500 text-rose-50"
			>
				<CheckFat size={15} weight="fill" class="mb-0.5" />
			</span>
			<img src="/placeholder-shoes.webp" alt="" class="h-full" />
		</button>
	</div>

	<div class="flex gap-0.5">
		{#each colors as color, i}
			{@const isActive = color === activeColor}
			<div class={cn('relative flex', i % 2 !== 0 && 'translate-y-2')}>
				<ColorIcon
					class={cn(
						'transition-all hover:-translate-y-0.5 [&_path]:cursor-pointer [&_path]:stroke-0 [&_path]:transition-all',
						isActive && '[&_path]:stroke-4'
					)}
					style="color:{color}"
					onclick={() => (activeColor = color)}
				/>
				{#if isActive}
					<span
						class="absolute -top-1.5 -right-0.5 flex size-6 items-center justify-center rounded-full bg-rose-500 text-rose-50"
						transition:scale={{ duration: 300, start: 0.75, opacity: 0, easing: backOut }}
					>
						<CheckFat size={12} weight="fill" class="mb-px" />
					</span>
				{/if}
			</div>
		{/each}
	</div>

	<button
		class="cursor-pointer rounded-4xl bg-rose-500 px-10 py-4 text-4xl font-bold text-white transition-all hover:scale-102 hover:rotate-2"
	>
		Confirm
	</button>
</main>

<img
	src="/ref.png"
	alt=""
	class="pointer-events-none absolute top-0 left-0 h-dvh w-dvw object-contain opacity-0"
/>
