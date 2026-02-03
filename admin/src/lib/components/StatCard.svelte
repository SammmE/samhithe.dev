<script lang="ts">
	import type { ComponentType } from 'svelte';

	interface Props {
		title: string;
		value: string | number;
		icon: ComponentType;
		color?: 'blue' | 'emerald' | 'amber' | 'purple';
	}

	let { title, value, icon: Icon, color = 'blue' }: Props = $props();

	const colorClasses = {
		blue: 'text-blue-400 bg-blue-500/10',
		emerald: 'text-emerald-400 bg-emerald-500/10',
		amber: 'text-amber-400 bg-amber-500/10',
		purple: 'text-purple-400 bg-purple-500/10'
	};
</script>

<div class="bg-slate-800 rounded-lg p-6 border border-slate-700 hover:border-slate-600 transition-colors">
	<div class="flex items-start justify-between">
		<div class="flex-1">
			<p class="text-sm font-medium text-slate-400 mb-1">{title}</p>
			<p class="text-2xl font-bold text-slate-100">{value}</p>
		</div>
		<div class="p-3 rounded-lg {colorClasses[color]}">
			<Icon size={24} />
		</div>
	</div>
</div>
