<script lang="ts">
	import type { ArticleMeta } from '$lib/types';
	import { calculateReadingTime } from '$lib/utils';

	let { data } = $props();
	let articles = $derived<ArticleMeta[]>(data.articles ?? []);

	function formatDate(dateStr: string): string {
		const d = new Date(dateStr);
		return d.toLocaleDateString('en-US', { year: 'numeric', month: 'long', day: 'numeric' });
	}
</script>

<svelte:head>
	<title>Blog — Samhith</title>
	<meta name="description" content="Technical articles, deep dives into system architecture, and reflections on the craft of software engineering." />
</svelte:head>

<div class="max-w-[1100px] mx-auto px-(--spacing-gutter) w-full py-20 lg:py-32">
	<!-- Header -->
	<header class="mb-20">
		<h1 class="text-headline-xl mb-6 text-on-surface">Thoughts & Observations</h1>
		<p class="text-body-md text-on-surface-variant max-w-2xl">
			A collection of technical articles, deep dives into system architecture, and reflections on the craft of software engineering.
		</p>
	</header>

	<!-- Blog List -->
	{#if articles.length > 0}
		<div class="flex flex-col gap-12">
			{#each articles as article}
				<a href="/blog/{article.id}" class="group border-t border-white/10 pt-8 flex flex-col md:flex-row gap-6 md:gap-12 items-start transition-colors hover:border-primary/50 no-underline cursor-pointer">
					<!-- Date / Reading Time -->
					<div class="flex-shrink-0 w-full md:w-48 pt-1">
						<div class="text-code-sm text-on-surface-variant flex items-center gap-4 md:flex-col md:items-start md:gap-2">
							<time datetime={article.date.slice(0, 10)}>{formatDate(article.date)}</time>
							<span class="md:hidden text-surface-variant">•</span>
							<span>{calculateReadingTime(article)}</span>
						</div>
					</div>

					<!-- Content -->
					<div class="flex-grow">
						<h2 class="text-headline-lg-mobile md:text-headline-lg mb-4 text-on-surface group-hover:text-primary transition-colors">
							{article.title}
						</h2>
						<p class="text-body-md text-on-surface-variant mb-6">{article.description}</p>
					</div>
				</a>
			{/each}
		</div>
	{:else}
		<p class="text-on-surface-variant text-body-md">No articles yet.</p>
	{/if}
</div>
