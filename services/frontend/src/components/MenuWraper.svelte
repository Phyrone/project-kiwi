<script lang="ts">
	import type SlideoutT from 'slideout';
	import { browser } from '$app/environment';
	import { WIDTH_SCREEN_2XL, WIDTH_SCREEN_SM } from '$lib/consts.ts';
	import { onDestroy, onMount } from 'svelte';
	import Navbar from '$routes/(app)/Navbar.svelte';
	import { menu_open as lg_open } from '$lib/store.ts';
	import { writable, type Writable } from 'svelte/store';


	let Slideout: typeof SlideoutT | undefined = undefined;

	let ready: boolean = false;
	$: ready = !!Slideout;
	let width: number = WIDTH_SCREEN_2XL;


	let use_mobile_menu: boolean = true;
	$: use_mobile_menu = width < WIDTH_SCREEN_SM;

	let open:Writable<boolean> ;
	$: open = use_mobile_menu ?  writable(false) : lg_open;

	let slideout: SlideoutT | undefined = undefined;
	let body_element: HTMLElement | null = null;
	let menu_element: HTMLElement | null = null;
	$:{
		if (Slideout && use_mobile_menu && body_element && menu_element) {
			if (!slideout) {
				$open = false;
				slideout = new Slideout({
					menu: menu_element,
					panel: body_element,
					side: 'left',
					padding: 256,
					tolerance: 70,
					touch: true
				});
				slideout.on('open', () => $open = true);
				slideout.on('close', () => $open = false);
			}
		} else if (slideout) {
			slideout.destroy();
			slideout = undefined;
		}
	}

	$:if (slideout) {
		if ($open) {
			slideout.open();
		} else {
			slideout.close();
		}
	}


	onMount(async () => {
		if (browser) {
			try {
				Slideout = await import('slideout').then((e) => e.default);
			} catch (e) {
				console.error('Failed to init slideout for potential mobile', e);
			}
		}
	});

	onDestroy(() => {
		slideout?.destroy();
		slideout = undefined;
	});


</script>


<svelte:window bind:innerWidth={width} />

<style lang="scss">

</style>
<div class="min-h-screen sm:flex" class:flex={!ready}>
	<nav bind:this={menu_element}
			 class:slideout-menu={slideout}
			 class="bg-base-300 p-safe transition-all @container/sidemenu"
			 class:sm:w-12={!$open || !ready}
			 class:sm:w-64={$open}
	>
		<slot open={$open} name="menu" />
	</nav>
	<main bind:this={body_element} class:slideout-panel={slideout}
				class="min-h-screen py-safe pr-safe flex flex-col sm:flex-auto ">
		<Navbar bind:open={$open} />
		<slot open={$open} />
	</main>
</div>
