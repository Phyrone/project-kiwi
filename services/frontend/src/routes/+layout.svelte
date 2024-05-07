<script lang="ts">
	import '$styles/app.scss';
	import {Toaster} from "svelte-sonner"
</script>
<Toaster richColors={true}/>
<slot />
