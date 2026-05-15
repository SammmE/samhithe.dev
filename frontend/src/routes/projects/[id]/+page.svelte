<script lang="ts">
	import { onMount } from 'svelte';
	import { marked } from 'marked';
	import type { Project } from '$lib/types';

	let { data } = $props();

	let project = $derived<Project>(data.project);
	let markdownContent = $derived<string>(data.markdownContent);

	// Extract headings for ToC
	interface TocItem {
		id: string;
		text: string;
		level: number;
	}

	function extractToc(html: string): TocItem[] {
		const items: TocItem[] = [];
		const regex = /<h([2-3])[^>]*id="([^"]*)"[^>]*>(.*?)<\/h[2-3]>/gi;
		let match;
		while ((match = regex.exec(html)) !== null) {
			items.push({
				level: parseInt(match[1], 10),
				id: match[2],
				text: match[3].replace(/<[^>]*>/g, '')
			});
		}
		return items;
	}

	const renderer = new marked.Renderer();
	renderer.heading = function ({ text, depth }: { text: string; depth: number }) {
		const slug = text.toLowerCase().replace(/[^\w]+/g, '-').replace(/^-+|-+$/g, '');
		return `<h${depth} id="${slug}">${text}</h${depth}>`;
	};
	marked.setOptions({ renderer });

	let renderedHtml = $derived(marked.parse(markdownContent, { async: false }) as string);
	let toc = $derived(extractToc(renderedHtml));

	let scrollProgress = $state(0);
	let activeHeading = $state('');

	onMount(() => {
		function updateProgress() {
			const scrollTop = window.scrollY;
			const docHeight = document.documentElement.scrollHeight - window.innerHeight;
			scrollProgress = docHeight > 0 ? Math.min(100, (scrollTop / docHeight) * 100) : 0;

			const headingElements = toc.map(t => document.getElementById(t.id)).filter(Boolean) as HTMLElement[];
			let current = '';
			for (const el of headingElements) {
				if (el.getBoundingClientRect().top <= 120) {
					current = el.id;
				}
			}
			activeHeading = current;
		}

		window.addEventListener('scroll', updateProgress, { passive: true });
		updateProgress();

		return () => {
			window.removeEventListener('scroll', updateProgress);
		};
	});
</script>

<svelte:head>
	<title>{project.name} — Samhith</title>
	<meta name="description" content={project.description} />
</svelte:head>

<div class="max-w-[1100px] mx-auto px-(--spacing-gutter) w-full flex-1 flex flex-col lg:flex-row gap-(--spacing-gutter) relative mt-16 mb-(--spacing-section-gap)">
	<!-- Sidebar: Outline -->
	<aside class="bg-surface-container-low h-[calc(100vh-64px)] w-64 top-[64px] sticky border-r border-white/10 hidden lg:flex flex-col py-8 gap-4 overflow-y-auto shrink-0">
		<div class="px-4 mb-4">
			<h3 class="text-label-caps text-on-surface mb-1">Project Outline</h3>
			<p class="text-code-sm text-on-surface-variant opacity-70">Table of Contents</p>
		</div>
		<nav class="flex flex-col gap-1 text-code-sm">
			{#each toc as item}
				<a
					href="#{item.id}"
					class="transition-all flex items-center gap-3 py-2 no-underline
						{item.level === 3 ? 'pl-8' : 'pl-4'}
						{activeHeading === item.id
							? 'text-primary border-l-2 border-primary bg-surface-variant/20'
							: 'text-on-surface-variant hover:text-on-surface border-l-2 border-transparent hover:bg-surface-variant/50'
						}"
				>
					{item.text}
				</a>
			{/each}
		</nav>
	</aside>

	<!-- Main Content -->
	<div class="flex-1 py-8 lg:py-16 max-w-3xl">
		<header class="mb-12">
			<div class="flex items-center gap-4 mb-6">
				{#if project.repo_link}
					<a href={project.repo_link} target="_blank" rel="noopener noreferrer" class="border border-white/15 px-4 py-2 text-code-sm text-on-surface-variant brand-flat hover:border-primary hover:text-primary transition-colors no-underline">Source Code</a>
				{/if}
				{#if project.demo_link}
					<a href={project.demo_link} target="_blank" rel="noopener noreferrer" class="border border-white/15 px-4 py-2 text-code-sm text-on-surface-variant brand-flat hover:border-primary hover:text-primary transition-colors no-underline">Live Demo</a>
				{/if}
			</div>
			<h1 class="text-headline-xl text-on-surface mb-6">{project.name}</h1>
			<p class="text-body-md text-on-surface-variant text-lg">{project.description}</p>
		</header>

		<article class="prose prose-invert prose-obsidian max-w-none space-y-8">
			{#if renderedHtml}
				{@html renderedHtml}
			{:else}
				<p class="text-on-surface-variant italic">No readme provided for this project.</p>
			{/if}
		</article>
	</div>
</div>

<div class="fixed bottom-0 left-0 w-full h-1.5 bg-surface-container-highest z-50">
	<div
		class="h-full bg-primary transition-all duration-150"
		style="width: {scrollProgress}%; box-shadow: 0 0 10px oklch(from var(--color-primary) l c h / 0.5);"
	></div>
</div>
