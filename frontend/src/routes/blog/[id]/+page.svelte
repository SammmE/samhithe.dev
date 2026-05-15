<script lang="ts">
	import { onMount } from 'svelte';
	import { marked } from 'marked';
	import { recordHit } from '$lib/api';
	import type { Article } from '$lib/types';
	import { calculateReadingTime } from '$lib/utils';

	let { data } = $props();

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
				level: parseInt(match[1]),
				id: match[2],
				text: match[3].replace(/<[^>]*>/g, '')
			});
		}
		return items;
	}

	// Configure marked to add IDs to headings
	const renderer = new marked.Renderer();
	renderer.heading = function ({ text, depth }: { text: string; depth: number }) {
		const slug = text.toLowerCase().replace(/[^\w]+/g, '-').replace(/^-+|-+$/g, '');
		return `<h${depth} id="${slug}">${text}</h${depth}>`;
	};
	marked.setOptions({ renderer });

	// All derived values
	let article = $derived<Article>(data.article);
	let readTime = $derived(calculateReadingTime(article));
	let formattedDate = $derived(new Date(article.date).toLocaleDateString('en-US', {
		year: 'numeric',
		month: 'short',
		day: 'numeric'
	}));
	let renderedHtml = $derived(marked.parse(article.content, { async: false }) as string);
	let toc = $derived(extractToc(renderedHtml));

	// Scroll progress
	let scrollProgress = $state(0);
	let activeHeading = $state('');

	onMount(() => {
		// Record hit
		recordHit(article.id).catch(() => {});

		function updateProgress() {
			const scrollTop = window.scrollY;
			const docHeight = document.documentElement.scrollHeight - window.innerHeight;
			scrollProgress = docHeight > 0 ? Math.min(100, (scrollTop / docHeight) * 100) : 0;

			// Update active heading
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
	<title>{article.title} — Samhith</title>
	<meta name="description" content={article.description} />
</svelte:head>

<div class="max-w-[1100px] mx-auto px-(--spacing-gutter) w-full flex-1 flex flex-col lg:flex-row gap-(--spacing-gutter) relative mt-16 mb-(--spacing-section-gap)">
	<!-- Sidebar: Article Outline -->
	<aside class="bg-surface-container-low h-[calc(100vh-64px)] w-64 top-[64px] sticky border-r border-white/10 hidden lg:flex flex-col py-8 gap-4 overflow-y-auto shrink-0">
		<div class="px-4 mb-4">
			<h3 class="text-label-caps text-on-surface mb-1">Article Outline</h3>
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

	<!-- Main Article Content -->
	<div class="flex-1 py-8 lg:py-16 max-w-3xl">
		<header class="mb-12">
			<div class="flex items-center gap-3 mb-6 text-code-sm text-on-surface-variant">
				<span>{formattedDate}</span>
				<span>•</span>
				<span>{readTime}</span>
				<span>•</span>
				<span>{article.views} views</span>
			</div>
			<h1 class="text-headline-xl text-on-surface mb-6">{article.title}</h1>
			<p class="text-body-md text-on-surface-variant text-lg">{article.description}</p>
		</header>

		<article class="prose prose-invert prose-obsidian max-w-none space-y-8">
			{@html renderedHtml}
		</article>
	</div>
</div>

<!-- Sticky Bottom Progress Bar -->
<div class="fixed bottom-0 left-0 w-full h-1.5 bg-surface-container-highest z-50">
	<div
		class="h-full bg-primary transition-all duration-150"
		style="width: {scrollProgress}%; box-shadow: 0 0 10px oklch(from var(--color-primary) l c h / 0.5);"
	></div>
</div>
