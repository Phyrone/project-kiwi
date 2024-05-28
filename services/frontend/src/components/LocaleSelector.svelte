<script lang="ts">
	import Localised from '$components/Localised.svelte';
	import { locales } from '$lib/i18n';

	let selectedLocale = 'auto';
	$: console.log(selectedLocale);
</script>

<select bind:value={selectedLocale}>
	<option value="auto">
		<Localised key="common:sel-default" />
	</option>
	{#each locales as locale}
		<option value={locale}>
			<Localised
				key={`meta:name`}
				values={{
					lng: locale
				}}
			/>
		</option>
	{/each}
</select>
