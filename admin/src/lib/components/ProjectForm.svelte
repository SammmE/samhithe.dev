<script lang="ts">
	import type { Project, ProjectCreateInput, ProjectUpdateInput } from '$lib/types';

	interface Props {
		initialData?: Project | null;
		onSubmit: (data: ProjectCreateInput) => Promise<void> | void;
		onCancel: () => void;
		isLoading?: boolean;
	}

	let { initialData = null, onSubmit, onCancel, isLoading = false }: Props = $props();

	let title = $state(initialData?.title || '');
	let repo_url = $state(initialData?.repo_url || '');
	let demo_url = $state(initialData?.demo_url || '');
	let priority = $state(initialData?.priority ?? 0);

	let errors = $state<Record<string, string>>({});

	function validate() {
		const newErrors: Record<string, string> = {};

		if (!title.trim()) {
			newErrors.title = 'Title is required';
		}

		if (!repo_url.trim()) {
			newErrors.repo_url = 'Repository URL is required';
		} else if (!repo_url.match(/^https?:\/\/.+/)) {
			newErrors.repo_url = 'Repository URL must be a valid URL';
		}

		if (demo_url && !demo_url.match(/^https?:\/\/.+/)) {
			newErrors.demo_url = 'Demo URL must be a valid URL';
		}

		if (priority < 0 || priority > 255) {
			newErrors.priority = 'Priority must be between 0 and 255';
		}

		errors = newErrors;
		return Object.keys(newErrors).length === 0;
	}

	function handleSubmit(e: Event) {
		e.preventDefault();

		if (!validate()) {
			return;
		}

		const data: ProjectCreateInput = {
			title: title.trim(),
			repo_url: repo_url.trim(),
			demo_url: demo_url.trim() || undefined,
			priority: priority
		};

		onSubmit(data);
	}
</script>

<form onsubmit={handleSubmit} class="space-y-4">
	<!-- Title -->
	<div>
		<label for="title" class="block text-sm font-medium text-slate-300 mb-2">
			Title <span class="text-red-400">*</span>
		</label>
		<input
			id="title"
			type="text"
			bind:value={title}
			placeholder="My Awesome Project"
			disabled={isLoading}
			class="w-full px-4 py-2 bg-slate-900 border border-slate-700 rounded-lg text-slate-100 placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent disabled:opacity-50 disabled:cursor-not-allowed"
		/>
		{#if errors.title}
			<p class="mt-1 text-sm text-red-400">{errors.title}</p>
		{/if}
	</div>

	<!-- Repository URL -->
	<div>
		<label for="repo_url" class="block text-sm font-medium text-slate-300 mb-2">
			Repository URL <span class="text-red-400">*</span>
		</label>
		<input
			id="repo_url"
			type="url"
			bind:value={repo_url}
			placeholder="https://github.com/username/repo"
			disabled={isLoading}
			class="w-full px-4 py-2 bg-slate-900 border border-slate-700 rounded-lg text-slate-100 placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent disabled:opacity-50 disabled:cursor-not-allowed"
		/>
		{#if errors.repo_url}
			<p class="mt-1 text-sm text-red-400">{errors.repo_url}</p>
		{/if}
	</div>

	<!-- Demo URL -->
	<div>
		<label for="demo_url" class="block text-sm font-medium text-slate-300 mb-2">
			Demo URL <span class="text-slate-500 text-xs">(Optional)</span>
		</label>
		<input
			id="demo_url"
			type="url"
			bind:value={demo_url}
			placeholder="https://demo.example.com"
			disabled={isLoading}
			class="w-full px-4 py-2 bg-slate-900 border border-slate-700 rounded-lg text-slate-100 placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent disabled:opacity-50 disabled:cursor-not-allowed"
		/>
		{#if errors.demo_url}
			<p class="mt-1 text-sm text-red-400">{errors.demo_url}</p>
		{/if}
	</div>

	<!-- Priority -->
	<div>
		<label for="priority" class="block text-sm font-medium text-slate-300 mb-2">
			Priority <span class="text-slate-500 text-xs">(0-255, Default: 0)</span>
		</label>
		<input
			id="priority"
			type="number"
			min="0"
			max="255"
			bind:value={priority}
			placeholder="0"
			disabled={isLoading}
			class="w-full px-4 py-2 bg-slate-900 border border-slate-700 rounded-lg text-slate-100 placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent disabled:opacity-50 disabled:cursor-not-allowed"
		/>
		{#if errors.priority}
			<p class="mt-1 text-sm text-red-400">{errors.priority}</p>
		{/if}
		<p class="mt-1 text-xs text-slate-500">Lower numbers appear first • 0 = undecided (alphabetical)</p>
	</div>

	<!-- Actions -->
	<div class="flex gap-3 pt-4">
		<button
			type="submit"
			disabled={isLoading}
			class="flex-1 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
		>
			{isLoading ? 'Saving...' : initialData ? 'Update Project' : 'Create Project'}
		</button>
		<button
			type="button"
			onclick={onCancel}
			disabled={isLoading}
			class="px-4 py-2 bg-slate-700 hover:bg-slate-600 text-slate-300 font-medium rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
		>
			Cancel
		</button>
	</div>
</form>
