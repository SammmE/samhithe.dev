<script lang="ts">
	interface Props {
		rows?: number;
		columns?: number;
	}

	let { rows = 5, columns = 4 }: Props = $props();
</script>

<div class="space-y-3">
	{#each Array(rows) as _, i}
		<div class="bg-slate-800 rounded-lg p-4 border border-slate-700 animate-pulse">
			<div class="flex gap-4 items-center">
				{#each Array(columns) as _, j}
					<div class="flex-1">
						<div class="h-4 bg-slate-700 rounded" style="width: {80 + Math.random() * 20}%"></div>
					</div>
				{/each}
			</div>
		</div>
	{/each}
</div>
