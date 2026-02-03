<script lang="ts">
	import { onMount } from 'svelte';
	import { toast } from 'svelte-sonner';
	import { getStats, forceSync } from '$lib/api';
	import type { Stats } from '$lib/types';
	import StatCard from '$lib/components/StatCard.svelte';
	import { Clock, Database, Cpu, FileStack, RefreshCw } from 'lucide-svelte';

	let stats = $state<Stats | null>(null);
	let isLoading = $state(true);
	let isSyncing = $state(false);

	async function loadStats() {
		try {
			isLoading = true;
			stats = await getStats();
		} catch (error) {
			toast.error('Failed to load stats');
			console.error(error);
		} finally {
			isLoading = false;
		}
	}

	async function handleForceSync() {
		try {
			isSyncing = true;
			await forceSync();
			toast.success('View buffer synced successfully');
			// Reload stats after sync
			await loadStats();
		} catch (error) {
			toast.error('Failed to sync view buffer');
			console.error(error);
		} finally {
			isSyncing = false;
		}
	}

	function formatUptime(seconds: number): string {
		const days = Math.floor(seconds / 86400);
		const hours = Math.floor((seconds % 86400) / 3600);
		const minutes = Math.floor((seconds % 3600) / 60);

		if (days > 0) {
			return `${days}d ${hours}h`;
		} else if (hours > 0) {
			return `${hours}h ${minutes}m`;
		} else {
			return `${minutes}m`;
		}
	}

	onMount(() => {
		loadStats();
	});
</script>

<div class="max-w-7xl">
	<!-- Header -->
	<div class="flex items-center justify-between mb-8">
		<div>
			<h1 class="text-3xl font-bold text-slate-100 mb-2">Dashboard</h1>
			<p class="text-slate-400">System statistics and overview</p>
		</div>
		<button
			onclick={handleForceSync}
			disabled={isSyncing}
			class="flex items-center gap-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
		>
			<RefreshCw size={18} class={isSyncing ? 'animate-spin' : ''} />
			{isSyncing ? 'Syncing...' : 'Force Sync'}
		</button>
	</div>

	<!-- Stats Grid -->
	{#if isLoading}
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
			{#each Array(4) as _}
				<div class="bg-slate-800 rounded-lg p-6 border border-slate-700 animate-pulse">
					<div class="h-4 bg-slate-700 rounded w-24 mb-3"></div>
					<div class="h-8 bg-slate-700 rounded w-32"></div>
				</div>
			{/each}
		</div>
	{:else if stats}
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
			<StatCard
				title="Uptime"
				value={formatUptime(stats.uptime_seconds)}
				icon={Clock}
				color="blue"
			/>
			<StatCard
				title="Memory Usage"
				value="{stats.memory_usage_mb.toFixed(2)} MB"
				icon={Database}
				color="emerald"
			/>
			<StatCard
				title="Engine"
				value={stats.engine}
				icon={Cpu}
				color="purple"
			/>
			<StatCard
				title="Buffered Views"
				value={stats.buffered_views_size}
				icon={FileStack}
				color="amber"
			/>
		</div>
	{:else}
		<div class="text-center py-12">
			<p class="text-slate-400">Failed to load stats</p>
			<button
				onclick={loadStats}
				class="mt-4 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg transition-colors"
			>
				Retry
			</button>
		</div>
	{/if}

	<!-- Additional Info -->
	<div class="mt-8 bg-slate-800 rounded-lg p-6 border border-slate-700">
		<h2 class="text-xl font-semibold text-slate-100 mb-4">About</h2>
		<div class="space-y-2 text-slate-300">
			<p>
				<span class="text-slate-400">Backend:</span> Rust/Axum running on
				<code class="text-blue-400 bg-slate-900 px-2 py-1 rounded">localhost:3000</code>
			</p>
			<p>
				<span class="text-slate-400">Authentication:</span> Header-based (X-Admin-Password)
			</p>
			<p>
				<span class="text-slate-400">Last Updated:</span>
				{new Date().toLocaleString()}
			</p>
		</div>
	</div>
</div>

