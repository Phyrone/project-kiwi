<script lang="ts">
import {User2Icon} from 'lucide-svelte'
</script>
<div class="h-16 shadow-2xl bg-base-300 grid place-items-center @container/sidebar-header">
	<div class="w-full flex justify-center @[4rem]:join @[4rem]:px-1">
		<button class="btn join-item py-0 px-2 @[4rem]:px-3">
			<User2Icon size={24} class=""/>
		</button>
		<button class="btn flex-auto join-item hidden @[4rem]:inline-flex">
			User
		</button>
	</div>
</div>
