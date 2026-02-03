<script lang="ts">
	import { onMount } from 'svelte';
	import { toast } from 'svelte-sonner';
	import {
		getProjects,
		createProject,
		updateProject,
		deleteProject,
		refreshProjectReadme
	} from '$lib/api';
	import type { Project, ProjectCreateInput, ProjectUpdateInput } from '$lib/types';
	import Modal from '$lib/components/Modal.svelte';
	import ProjectForm from '$lib/components/ProjectForm.svelte';
	import MarkdownPreview from '$lib/components/MarkdownPreview.svelte';
	import TableSkeleton from '$lib/components/TableSkeleton.svelte';
	import {
		Plus,
		Pencil,
		Trash2,
		RefreshCw,
		Eye,
		ExternalLink,
		Search,
		GitBranch,
		GripVertical
	} from 'lucide-svelte';

	let projects = $state<Project[]>([]);
	let isLoading = $state(true);
	let searchQuery = $state('');

	// Modals
	let showCreateModal = $state(false);
	let showEditModal = $state(false);
	let showReadmeModal = $state(false);
	let editingProject = $state<Project | null>(null);
	let viewingReadme = $state<Project | null>(null);
	let isSubmitting = $state(false);

	// Loading states for individual actions
	let refreshingIds = $state<Set<number>>(new Set());
	let deletingIds = $state<Set<number>>(new Set());

	// Drag and drop state
	let draggedIndex = $state<number | null>(null);
	let dragOverIndex = $state<number | null>(null);
	let isReordering = $state(false);

	// Filtered projects based on search
	let filteredProjects = $derived(
		projects.filter(
			(p) =>
				p.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
				p.repo_url.toLowerCase().includes(searchQuery.toLowerCase()) ||
				p.id.toString().includes(searchQuery)
		)
	);

	async function loadProjects() {
		try {
			isLoading = true;
			projects = await getProjects();
		} catch (error) {
			toast.error('Failed to load projects');
			console.error(error);
		} finally {
			isLoading = false;
		}
	}

	async function handleCreateProject(data: ProjectCreateInput) {
		try {
			isSubmitting = true;
			const newProject = await createProject(data);
			projects = [...projects, newProject];
			toast.success('Project created successfully');
			showCreateModal = false;
		} catch (error) {
			toast.error('Failed to create project');
			console.error(error);
		} finally {
			isSubmitting = false;
		}
	}

	async function handleUpdateProject(data: ProjectUpdateInput) {
		if (!editingProject) return;

		try {
			isSubmitting = true;
			const updated = await updateProject(editingProject.id, data);
			projects = projects.map((p) => (p.id === updated.id ? updated : p));
			toast.success('Project updated successfully');
			showEditModal = false;
			editingProject = null;
		} catch (error) {
			toast.error('Failed to update project');
			console.error(error);
		} finally {
			isSubmitting = false;
		}
	}

	async function handleDeleteProject(project: Project) {
		if (!confirm(`Are you sure you want to delete "${project.title}"?`)) {
			return;
		}

		try {
			deletingIds.add(project.id);
			deletingIds = deletingIds; // Trigger reactivity
			await deleteProject(project.id);
			projects = projects.filter((p) => p.id !== project.id);
			toast.success('Project deleted successfully');
		} catch (error) {
			toast.error('Failed to delete project');
			console.error(error);
		} finally {
			deletingIds.delete(project.id);
			deletingIds = deletingIds;
		}
	}

	async function handleRefreshReadme(project: Project) {
		try {
			refreshingIds.add(project.id);
			refreshingIds = refreshingIds; // Trigger reactivity
			await refreshProjectReadme(project.id);
			toast.success('README refreshed successfully');
			// Reload projects to get updated readme
			await loadProjects();
		} catch (error) {
			toast.error('Failed to refresh README');
			console.error(error);
		} finally {
			refreshingIds.delete(project.id);
			refreshingIds = refreshingIds;
		}
	}

	function openEditModal(project: Project) {
		editingProject = project;
		showEditModal = true;
	}

	function openReadmeModal(project: Project) {
		viewingReadme = project;
		showReadmeModal = true;
	}

	function closeCreateModal() {
		showCreateModal = false;
	}

	function closeEditModal() {
		showEditModal = false;
		editingProject = null;
	}

	function closeReadmeModal() {
		showReadmeModal = false;
		viewingReadme = null;
	}

	// Drag and drop handlers
	function handleDragStart(e: DragEvent, index: number) {
		if (searchQuery) {
			// Don't allow reordering while searching
			e.preventDefault();
			toast.error('Clear search to reorder projects');
			return;
		}
		draggedIndex = index;
		if (e.dataTransfer) {
			e.dataTransfer.effectAllowed = 'move';
		}
	}

	function handleDragOver(e: DragEvent, index: number) {
		e.preventDefault();
		if (draggedIndex === null) return;
		dragOverIndex = index;
	}

	function handleDragLeave() {
		dragOverIndex = null;
	}

	async function handleDrop(e: DragEvent, dropIndex: number) {
		e.preventDefault();
		
		if (draggedIndex === null || draggedIndex === dropIndex) {
			draggedIndex = null;
			dragOverIndex = null;
			return;
		}

		try {
			isReordering = true;

			// Reorder the array
			const reordered = [...projects];
			const [draggedItem] = reordered.splice(draggedIndex, 1);
			reordered.splice(dropIndex, 0, draggedItem);

			// Calculate new priorities based on position
			// Backend orders by lowest first, so we assign priorities in ascending order
			const updatedProjects = reordered.map((project, index) => ({
				...project,
				priority: index + 1
			}));

			// OPTIMIZATION: Only update projects whose priority actually changed
			// This dramatically reduces API calls for large lists
			const projectsToUpdate = updatedProjects.filter((project, index) => {
				const originalProject = projects.find((p) => p.id === project.id);
				return originalProject && originalProject.priority !== project.priority;
			});

			// Update local state immediately for smooth UX
			projects = updatedProjects;

			// Send API requests only for projects with changed priorities
			if (projectsToUpdate.length > 0) {
				const updatePromises = projectsToUpdate.map((project) =>
					updateProject(project.id, { priority: project.priority })
				);

				await Promise.all(updatePromises);
				toast.success(
					`Project order updated (${projectsToUpdate.length} ${projectsToUpdate.length === 1 ? 'project' : 'projects'} changed)`
				);
			} else {
				// Edge case: no actual changes (shouldn't happen but good to handle)
				toast.info('No changes needed');
			}
		} catch (error) {
			toast.error('Failed to update project order');
			console.error(error);
			// Reload to get correct state from server
			await loadProjects();
		} finally {
			draggedIndex = null;
			dragOverIndex = null;
			isReordering = false;
		}
	}

	function handleDragEnd() {
		draggedIndex = null;
		dragOverIndex = null;
	}

	onMount(() => {
		loadProjects();
	});
</script>

<div class="max-w-7xl">
	<!-- Header -->
	<div class="flex items-center justify-between mb-8">
		<div>
			<h1 class="text-3xl font-bold text-slate-100 mb-2">Projects</h1>
			<p class="text-slate-400">
				Manage your portfolio projects • Drag rows to reorder (lowest priority first)
			</p>
		</div>
		<button
			onclick={() => (showCreateModal = true)}
			class="flex items-center gap-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg transition-colors"
		>
			<Plus size={18} />
			Add Project
		</button>
	</div>

	<!-- Search Bar -->
	<div class="mb-6">
		<div class="relative">
			<Search class="absolute left-3 top-1/2 -translate-y-1/2 text-slate-400" size={20} />
			<input
				type="text"
				bind:value={searchQuery}
				placeholder="Search projects by title, URL, or ID..."
				class="w-full pl-10 pr-4 py-3 bg-slate-800 border border-slate-700 rounded-lg text-slate-100 placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
			/>
		</div>
	</div>

	<!-- Projects Table -->
	{#if isLoading}
		<TableSkeleton rows={5} columns={5} />
	{:else if filteredProjects.length === 0}
		<div class="bg-slate-800 rounded-lg p-12 text-center border border-slate-700">
			<p class="text-slate-400 mb-4">
				{searchQuery ? 'No projects found matching your search' : 'No projects yet'}
			</p>
			{#if !searchQuery}
				<button
					onclick={() => (showCreateModal = true)}
					class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg transition-colors"
				>
					Create Your First Project
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
								Title
							</th>
							<th class="px-6 py-4 text-left text-xs font-medium text-slate-400 uppercase tracking-wider">
								Repository
							</th>
							<th class="px-6 py-4 text-left text-xs font-medium text-slate-400 uppercase tracking-wider">
								Priority
							</th>
							<th class="px-6 py-4 text-left text-xs font-medium text-slate-400 uppercase tracking-wider">
								Views
							</th>
							<th class="px-6 py-4 text-right text-xs font-medium text-slate-400 uppercase tracking-wider">
								Actions
							</th>
						</tr>
					</thead>
					<tbody class="divide-y divide-slate-700">
						{#each filteredProjects as project, index}
							<tr
								draggable={!searchQuery}
								ondragstart={(e) => handleDragStart(e, index)}
								ondragover={(e) => handleDragOver(e, index)}
								ondragleave={handleDragLeave}
								ondrop={(e) => handleDrop(e, index)}
								ondragend={handleDragEnd}
								class="hover:bg-slate-700/50 transition-colors {draggedIndex === index
									? 'opacity-50'
									: ''} {dragOverIndex === index
									? 'border-t-2 border-t-blue-500'
									: ''} {!searchQuery ? 'cursor-move' : ''}"
							>
								<td class="px-6 py-4 whitespace-nowrap text-sm text-slate-300">
									<div class="flex items-center gap-2">
										{#if !searchQuery}
											<GripVertical size={16} class="text-slate-500 cursor-grab active:cursor-grabbing" />
										{/if}
										<span>#{project.id}</span>
									</div>
								</td>
								<td class="px-6 py-4 whitespace-nowrap">
									<div class="flex items-center gap-2">
										<span class="text-sm font-medium text-slate-100">{project.title}</span>
										{#if project.demo_url}
											<a
												href={project.demo_url}
												target="_blank"
												rel="noopener noreferrer"
												class="text-blue-400 hover:text-blue-300"
												title="View Demo"
											>
												<ExternalLink size={14} />
											</a>
										{/if}
									</div>
							</td>
							<td class="px-6 py-4 whitespace-nowrap">
								<a
									href={project.repo_url}
									target="_blank"
									rel="noopener noreferrer"
									class="flex items-center gap-2 text-sm text-slate-400 hover:text-slate-200 transition-colors"
								>
									<GitBranch size={14} />
									<span class="max-w-xs truncate">{project.repo_url}</span>
								</a>
							</td>
							<td class="px-6 py-4 whitespace-nowrap">
								<span
									class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {project.priority >= 200
										? 'bg-red-500/10 text-red-400'
										: project.priority >= 100
											? 'bg-amber-500/10 text-amber-400'
											: project.priority > 0
												? 'bg-blue-500/10 text-blue-400'
												: 'bg-slate-700 text-slate-400'}"
								>
									{project.priority}
								</span>
							</td>
							<td class="px-6 py-4 whitespace-nowrap text-sm text-slate-300">
									<div class="flex items-center gap-1">
										<Eye size={14} class="text-slate-500" />
										{project.view_count}
									</div>
								</td>
								<td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
									<div class="flex items-center justify-end gap-2">
										<!-- View README -->
										{#if project.readme_content}
											<button
												onclick={() => openReadmeModal(project)}
												class="p-2 text-slate-400 hover:text-blue-400 hover:bg-slate-700 rounded transition-colors"
												title="View README"
											>
												<Eye size={16} />
											</button>
										{/if}

										<!-- Edit -->
										<button
											onclick={() => openEditModal(project)}
											class="p-2 text-slate-400 hover:text-emerald-400 hover:bg-slate-700 rounded transition-colors"
											title="Edit"
										>
											<Pencil size={16} />
										</button>

										<!-- Refresh README -->
										<button
											onclick={() => handleRefreshReadme(project)}
											disabled={refreshingIds.has(project.id)}
											class="p-2 text-slate-400 hover:text-amber-400 hover:bg-slate-700 rounded transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
											title="Refresh README"
										>
											<RefreshCw size={16} class={refreshingIds.has(project.id) ? 'animate-spin' : ''} />
										</button>

										<!-- Delete -->
										<button
											onclick={() => handleDeleteProject(project)}
											disabled={deletingIds.has(project.id)}
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
		<p class="mt-4 text-sm text-slate-400">
			Showing {filteredProjects.length} of {projects.length} project{projects.length !== 1
				? 's'
				: ''}
		</p>
	{/if}
</div>

<!-- Create Modal -->
<Modal open={showCreateModal} title="Create Project" onClose={closeCreateModal}>
	<ProjectForm onSubmit={handleCreateProject} onCancel={closeCreateModal} isLoading={isSubmitting} />
</Modal>

<!-- Edit Modal -->
<Modal open={showEditModal} title="Edit Project" onClose={closeEditModal}>
	<ProjectForm
		initialData={editingProject}
		onSubmit={handleUpdateProject}
		onCancel={closeEditModal}
		isLoading={isSubmitting}
	/>
</Modal>

<!-- README Preview Modal -->
<Modal open={showReadmeModal} title="README Preview" onClose={closeReadmeModal} size="xl">
	{#if viewingReadme}
		<div class="mb-4 pb-4 border-b border-slate-700">
			<h3 class="text-lg font-semibold text-slate-100">{viewingReadme.title}</h3>
			<a
				href={viewingReadme.repo_url}
				target="_blank"
				rel="noopener noreferrer"
				class="text-sm text-blue-400 hover:text-blue-300 flex items-center gap-1 mt-1"
			>
				<GitBranch size={14} />
				{viewingReadme.repo_url}
			</a>
		</div>
		<MarkdownPreview content={viewingReadme.readme_content || ''} />
	{/if}
</Modal>
