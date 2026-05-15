<script lang="ts">
	import type { Project } from '$lib/types';

	let { data } = $props();
	let projects = $derived<Project[]>(data.projects ?? []);

	function healthDot(status: string): string {
		switch (status) {
			case 'healthy': return 'bg-green-500';
			case 'broken': return 'bg-red-500';
			default: return 'bg-on-surface-variant';
		}
	}

	function actionLabel(project: Project): { label: string; href: string } {
		return { label: 'VIEW DETAILS', href: `/projects/${project.id}` };
	}
</script>

<svelte:head>
	<title>Projects — Samhith</title>
	<meta name="description" content="A collection of technical projects, systems architecture, and experimental code by Samhith." />
</svelte:head>

<div class="max-w-[1100px] mx-auto px-(--spacing-gutter) w-full py-(--spacing-section-gap)">
	<!-- Page Header -->
	<header class="mb-16 md:mb-24 flex flex-col gap-4">
		<h1 class="text-headline-xl text-on-surface">Selected Works</h1>
		<p class="text-body-md text-on-surface-variant max-w-2xl">
			A collection of technical projects, systems architecture, and experimental code. Built with a focus on performance, scalability, and precision engineering.
		</p>
	</header>

	<!-- Projects Grid -->
	{#if projects.length > 0}
		<div class="grid grid-cols-1 md:grid-cols-2 gap-8 lg:gap-12">
			{#each projects as project}
				{@const action = actionLabel(project)}
				<article class="bg-surface border border-white/15 hover:border-primary transition-colors duration-300 flex flex-col group brand-flat">
					<!-- Placeholder visual header -->
					<div class="h-64 w-full bg-surface-container-low overflow-hidden relative border-b border-white/15 flex items-center justify-center">
						<div class="absolute inset-0 bg-[radial-gradient(ellipse_at_center,oklch(from_var(--color-primary)_l_c_h_/_0.1),var(--color-background))]"></div>
						<span class="material-symbols-outlined text-[64px] text-primary opacity-50 group-hover:opacity-100 transition-opacity z-10">
							{project.health_status === 'healthy' ? 'check_circle' : project.health_status === 'broken' ? 'error' : 'memory'}
						</span>
					</div>

					<div class="p-6 flex flex-col flex-grow gap-4">
						<div class="flex items-center gap-2">
							<h2 class="text-headline-lg-mobile md:text-headline-lg text-on-surface flex-grow">{project.name}</h2>
							<!-- Health dot -->
							<span class="w-2 h-2 rounded-full {healthDot(project.health_status)}" title="{project.health_status}"></span>
						</div>

						<p class="text-body-md text-on-surface-variant flex-grow">{project.description}</p>

						<!-- Action link -->
						{#if action.href !== '#'}
							<a
								href={action.href}
								class="mt-6 flex items-center gap-2 text-label-caps text-on-surface group-hover:text-primary transition-colors w-fit border-b border-transparent group-hover:border-primary pb-1 no-underline"
							>
								{action.label}
								<span class="material-symbols-outlined text-[16px] transition-transform group-hover:translate-x-1">arrow_forward</span>
							</a>
						{/if}
					</div>
				</article>
			{/each}
		</div>
	{:else}
		<p class="text-on-surface-variant text-body-md">No projects yet.</p>
	{/if}
</div>
