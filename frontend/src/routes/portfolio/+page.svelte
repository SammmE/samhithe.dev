<script lang="ts">
	import { marked } from 'marked';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	let searchQuery = $state('');

	let filteredProjects = $derived.by(() => {
		let filtered = [...data.projects];
		// Sort by importance descending
		filtered.sort((a, b) => (b.importance || 0) - (a.importance || 0));

		const query = searchQuery.trim().toLowerCase();
		if (query) {
			if (query.startsWith('#')) {
				const searchTag = query.slice(1);
				filtered = filtered.filter(p => p.tags && p.tags.some(t => t.toLowerCase().includes(searchTag)));
			} else {
				filtered = filtered.filter(p => 
					p.name.toLowerCase().includes(query) || 
					(p.description && p.description.toLowerCase().includes(query)) ||
					(p.portfolio_entry && p.portfolio_entry.toLowerCase().includes(query))
				);
			}
		}
		return filtered;
	});

	function handleTagClick(tag: string) {
		searchQuery = `#${tag}`;
	}
</script>

<svelte:head>
	<title>Portfolio | Samhith</title>
</svelte:head>

<div class="max-w-[1100px] mx-auto px-(--spacing-gutter) py-12">
	<div class="mb-12 border-b border-white/10 pb-8">
		<h1 class="text-display-md text-on-surface mb-4">Portfolio</h1>
		<p class="text-on-surface-variant text-body-lg mb-8 max-w-2xl">
			A curated collection of my most important work. Search by keywords or filter by tags to explore specific skills and technologies.
		</p>

		<div class="relative max-w-xl">
			<div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
				<svg class="w-5 h-5 text-on-surface-variant" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path></svg>
			</div>
			<input 
				type="text" 
				bind:value={searchQuery} 
				placeholder="Search projects or use #tag to filter by skills..." 
				class="w-full pl-10 pr-4 py-3 bg-surface border border-white/20 text-on-surface placeholder:text-on-surface-variant focus:outline-none focus:border-primary focus:ring-1 focus:ring-primary transition-colors"
			/>
		</div>
	</div>

	<div class="grid grid-cols-1 md:grid-cols-2 gap-8">
		{#each filteredProjects as project (project.id)}
			<div class="flex flex-col bg-surface-container border border-white/10 hover:border-white/30 transition-colors shadow-lg">
				<div class="p-6 flex-grow flex flex-col">
					<div class="flex justify-between items-start mb-4 gap-4">
						<h2 class="text-title-lg font-bold text-on-surface">{project.name}</h2>
						{#if (project.importance || 0) > 5}
							<span class="shrink-0 px-2 py-1 bg-primary/10 text-primary text-label-sm border border-primary/20 uppercase tracking-wider">
								Featured
							</span>
						{/if}
					</div>
					
					<div class="prose prose-invert prose-sm max-w-none mb-6 text-on-surface-variant flex-grow">
						{@html marked.parse(project.portfolio_entry || project.description || '')}
					</div>

					{#if project.tags && project.tags.length > 0}
						<div class="flex flex-wrap gap-2 mt-auto pt-4">
							{#each project.tags as tag}
								<button 
									onclick={() => handleTagClick(tag)}
									class="px-2 py-1 bg-surface text-on-surface-variant text-label-sm border border-white/10 hover:text-primary hover:border-primary/50 transition-colors cursor-pointer"
								>
									#{tag}
								</button>
							{/each}
						</div>
					{/if}
				</div>
				<div class="px-6 py-4 bg-surface border-t border-white/10 flex flex-wrap gap-6">
					{#if project.demo_link}
						<a href={project.demo_link} target="_blank" rel="noopener noreferrer" class="text-label-caps text-primary hover:opacity-80 transition-opacity no-underline flex items-center gap-1">
							Live Demo
							<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"></path></svg>
						</a>
					{/if}
					{#if project.repo_link}
						<a href={project.repo_link} target="_blank" rel="noopener noreferrer" class="text-label-caps text-on-surface-variant hover:text-on-surface transition-colors no-underline flex items-center gap-1">
							Source Code
							<svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"></path></svg>
						</a>
					{/if}
				</div>
			</div>
		{/each}
		
		{#if filteredProjects.length === 0}
			<div class="col-span-full py-16 text-center text-on-surface-variant border border-dashed border-white/20 bg-surface/50">
				<p class="text-body-lg">No projects found matching "{searchQuery}".</p>
				<button onclick={() => searchQuery = ''} class="mt-4 text-primary hover:underline font-medium">Clear search</button>
			</div>
		{/if}
	</div>
</div>
