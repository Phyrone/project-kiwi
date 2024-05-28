<script lang="ts">
	import '$styles/app.scss';
	import { Toaster } from 'svelte-french-toast';
</script>

<Toaster containerStyle="top: max(env(safe-area-inset-top), 16px);" />
<slot />
