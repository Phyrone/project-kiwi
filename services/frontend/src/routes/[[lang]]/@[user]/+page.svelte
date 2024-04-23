<script lang="ts">
	import type { PageData } from './$types';
	import UserSelectorDisplay from '$components/UserSelectorDisplay.svelte';

	export let data: PageData;
</script>
<UserSelectorDisplay user_selector={data.parsed_user}/>
{JSON.stringify(data, null, 2)}
