<script lang="ts">

	import { stringify_user_selector, type UserSelector } from '$lib/selectos';

	export let user_selector: UserSelector;
</script>
{stringify_user_selector(user_selector)}