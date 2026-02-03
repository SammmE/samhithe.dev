<script lang="ts">
	import { onMount } from 'svelte';
	import { toast } from 'svelte-sonner';
	import { getLogs, createLog, updateLog, deleteLog } from '$lib/api';
	import type { Log, LogCreateInput, LogUpdateInput } from '$lib/types';
	import Modal from '$lib/components/Modal.svelte';
	import LogForm from '$lib/components/LogForm.svelte';
	import TableSkeleton from '$lib/components/TableSkeleton.svelte';
	import { Plus, Pencil, Trash2, Eye, Calendar, Search, ChevronDown } from 'lucide-svelte';

	let logs = $state<Log[]>([]);
	let isLoading = $state(true);
	let searchQuery = $state('');

	// Pagination state
	let currentPage = $state(1);
	let pageLimit = $state(20);
	let isLoadingMore = $state(false);
	let hasMore = $state(true);

	// Modals
	let showCreateModal = $state(false);
	let showEditModal = $state(false);
	let showViewModal = $state(false);
	let editingLog = $state<Log | null>(null);
	let viewingLog = $state<Log | null>(null);
	let isSubmitting = $state(false);

	// Loading states for individual actions
	let deletingIds = $state<Set<number>>(new Set());

	// Filtered logs based on search
	let filteredLogs = $derived(
		logs.filter(
			(log) =>
				log.content.toLowerCase().includes(searchQuery.toLowerCase()) ||
				log.id.toString().includes(searchQuery)
		)
	);

	async function loadInitialLogs() {
		try {
			isLoading = true;
			currentPage = 1;
			const data = await getLogs(1, pageLimit);
			logs = data;
			hasMore = data.length >= pageLimit;
		} catch (error) {
			toast.error('Failed to load logs');
			console.error(error);
		} finally {
			isLoading = false;
		}
	}

	async function loadMoreLogs() {
		if (!hasMore || isLoadingMore) return;

		try {
			isLoadingMore = true;
			const nextPage = currentPage + 1;
			const data = await getLogs(nextPage, pageLimit);
			
			// Append new logs to existing list
			logs = [...logs, ...data];
			currentPage = nextPage;
			
			// If we received fewer items than the limit, we've reached the end
			hasMore = data.length >= pageLimit;
		} catch (error) {
			toast.error('Failed to load more logs');
			console.error(error);
		} finally {
			isLoadingMore = false;
		}
	}

	async function handleCreateLog(data: LogCreateInput) {
		try {
			isSubmitting = true;
			const newLog = await createLog(data);
			// Prepend new log to the beginning
			logs = [newLog, ...logs];
			toast.success('Log created successfully');
			showCreateModal = false;
		} catch (error) {
			toast.error('Failed to create log');
			console.error(error);
		} finally {
			isSubmitting = false;
		}
	}

	async function handleUpdateLog(data: LogUpdateInput) {
		if (!editingLog) return;

		try {
			isSubmitting = true;
			const updated = await updateLog(editingLog.id, data);
			logs = logs.map((l) => (l.id === updated.id ? updated : l));
			toast.success('Log updated successfully');
			showEditModal = false;
			editingLog = null;
		} catch (error) {
			toast.error('Failed to update log');
			console.error(error);
		} finally {
			isSubmitting = false;
		}
	}

	async function handleDeleteLog(log: Log) {
		if (!confirm('Are you sure you want to delete this log?')) {
			return;
		}

		try {
			deletingIds.add(log.id);
			deletingIds = deletingIds; // Trigger reactivity
			await deleteLog(log.id);
			logs = logs.filter((l) => l.id !== log.id);
			toast.success('Log deleted successfully');
		} catch (error) {
			toast.error('Failed to delete log');
			console.error(error);
		} finally {
			deletingIds.delete(log.id);
			deletingIds = deletingIds;
		}
	}

	function openEditModal(log: Log) {
		editingLog = log;
		showEditModal = true;
	}

	function openViewModal(log: Log) {
		viewingLog = log;
		showViewModal = true;
	}

	function closeCreateModal() {
		showCreateModal = false;
	}

	function closeEditModal() {
		showEditModal = false;
		editingLog = null;
	}

	function closeViewModal() {
		showViewModal = false;
		viewingLog = null;
	}

	function truncateContent(content: string, maxLength: number = 100): string {
		if (content.length <= maxLength) return content;
		return content.substring(0, maxLength) + '...';
	}

	function formatDate(dateString: string): string {
		const date = new Date(dateString);
		return date.toLocaleDateString('en-US', {
			year: 'numeric',
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	onMount(() => {
		loadInitialLogs();
	});
</script>

<div class="max-w-7xl">
	<!-- Header -->
	<div class="flex items-center justify-between mb-8">
		<div>
			<h1 class="text-3xl font-bold text-slate-100 mb-2">Logs</h1>
			<p class="text-slate-400">Manage your portfolio logs</p>
		</div>
		<button
			onclick={() => (showCreateModal = true)}
			class="flex items-center gap-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg transition-colors"
		>
			<Plus size={18} />
			Add Log
		</button>
	</div>

	<!-- Search Bar -->
	<div class="mb-6">
		<div class="relative">
			<Search class="absolute left-3 top-1/2 -translate-y-1/2 text-slate-400" size={20} />
			<input
				type="text"
				bind:value={searchQuery}
				placeholder="Search logs by content or ID..."
				class="w-full pl-10 pr-4 py-3 bg-slate-800 border border-slate-700 rounded-lg text-slate-100 placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
			/>
		</div>
	</div>

	<!-- Logs Table -->
	{#if isLoading}
		<TableSkeleton rows={5} columns={4} />
	{:else if filteredLogs.length === 0}
		<div class="bg-slate-800 rounded-lg p-12 text-center border border-slate-700">
			<p class="text-slate-400 mb-4">
				{searchQuery ? 'No logs found matching your search' : 'No logs yet'}
			</p>
			{#if !searchQuery}
				<button
					onclick={() => (showCreateModal = true)}
					class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg transition-colors"
				>
					Create Your First Log
				</button>
			{/if}
		</div>
	{:else}
		<div class="bg-slate-800 rounded-lg border border-slate-700 overflow-hidden">
			<div class="overflow-x-auto">
				<table class="w-full">
					<thead class="bg-slate-900 border-b border-slate-700">
						<tr>
							<th class="px-6 py-4 text-left text-xs font-medium text-slate-400 uppercase tracking-wider">
								ID
							</th>
							<th class="px-6 py-4 text-left text-xs font-medium text-slate-400 uppercase tracking-wider">
								Content
							</th>
							<th class="px-6 py-4 text-left text-xs font-medium text-slate-400 uppercase tracking-wider">
								Views
							</th>
							<th class="px-6 py-4 text-left text-xs font-medium text-slate-400 uppercase tracking-wider">
								Created
							</th>
							<th class="px-6 py-4 text-right text-xs font-medium text-slate-400 uppercase tracking-wider">
								Actions
							</th>
						</tr>
					</thead>
					<tbody class="divide-y divide-slate-700">
						{#each filteredLogs as log}
							<tr class="hover:bg-slate-700/50 transition-colors">
								<td class="px-6 py-4 whitespace-nowrap text-sm text-slate-300">
									#{log.id}
								</td>
								<td class="px-6 py-4">
									<div class="text-sm text-slate-100 max-w-2xl">
										{truncateContent(log.content, 150)}
									</div>
								</td>
								<td class="px-6 py-4 whitespace-nowrap text-sm text-slate-300">
									<div class="flex items-center gap-1">
										<Eye size={14} class="text-slate-500" />
										{log.view_count}
									</div>
								</td>
								<td class="px-6 py-4 whitespace-nowrap text-sm text-slate-400">
									<div class="flex items-center gap-1">
										<Calendar size={14} class="text-slate-500" />
										{formatDate(log.created_at)}
									</div>
								</td>
								<td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
									<div class="flex items-center justify-end gap-2">
										<!-- View Full Content -->
										<button
											onclick={() => openViewModal(log)}
											class="p-2 text-slate-400 hover:text-blue-400 hover:bg-slate-700 rounded transition-colors"
											title="View Full Content"
										>
											<Eye size={16} />
										</button>

										<!-- Edit -->
										<button
											onclick={() => openEditModal(log)}
											class="p-2 text-slate-400 hover:text-emerald-400 hover:bg-slate-700 rounded transition-colors"
											title="Edit"
										>
											<Pencil size={16} />
										</button>

										<!-- Delete -->
										<button
											onclick={() => handleDeleteLog(log)}
											disabled={deletingIds.has(log.id)}
											class="p-2 text-slate-400 hover:text-red-400 hover:bg-slate-700 rounded transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
											title="Delete"
										>
											<Trash2 size={16} />
										</button>
									</div>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</div>

		<!-- Results Count -->
		<div class="flex items-center justify-between mt-4">
			<p class="text-sm text-slate-400">
				Showing {filteredLogs.length} of {logs.length} log{logs.length !== 1 ? 's' : ''}
			</p>
			
			<!-- Load More Button -->
			{#if hasMore && !searchQuery}
				<button
					onclick={loadMoreLogs}
					disabled={isLoadingMore}
					class="flex items-center gap-2 px-4 py-2 bg-slate-700 hover:bg-slate-600 text-slate-300 font-medium rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
				>
					{#if isLoadingMore}
						<div class="w-4 h-4 border-2 border-slate-400 border-t-transparent rounded-full animate-spin"></div>
						<span>Loading...</span>
					{:else}
						<ChevronDown size={18} />
						<span>Load More</span>
					{/if}
				</button>
			{/if}
		</div>
	{/if}
</div>

<!-- Create Modal -->
<Modal open={showCreateModal} title="Create Log" onClose={closeCreateModal} size="2xl">
	<LogForm onSubmit={handleCreateLog} onCancel={closeCreateModal} isLoading={isSubmitting} />
</Modal>

<!-- Edit Modal -->
<Modal open={showEditModal} title="Edit Log" onClose={closeEditModal} size="2xl">
	<LogForm
		initialData={editingLog}
		onSubmit={handleUpdateLog}
		onCancel={closeEditModal}
		isLoading={isSubmitting}
	/>
</Modal>

<!-- View Full Content Modal -->
<Modal open={showViewModal} title="Log Details" onClose={closeViewModal} size="lg">
	{#if viewingLog}
		<div class="space-y-4">
			<div class="flex items-center justify-between text-sm text-slate-400">
				<div class="flex items-center gap-4">
					<span>ID: #{viewingLog.id}</span>
					<span class="flex items-center gap-1">
						<Eye size={14} />
						{viewingLog.view_count} views
					</span>
				</div>
				<span class="flex items-center gap-1">
					<Calendar size={14} />
					{formatDate(viewingLog.created_at)}
				</span>
			</div>
			<div class="bg-slate-900 rounded-lg p-4 border border-slate-700">
				<p class="text-slate-100 whitespace-pre-wrap">{viewingLog.content}</p>
			</div>
		</div>
	{/if}
</Modal>
