<script lang="ts">
  import type { Project, ArticleMeta } from "$lib/types";

  let { data } = $props();

  let featuredProjects = $derived<Project[]>(data.featuredProjects ?? []);
  let recentArticles = $derived<ArticleMeta[]>(data.recentArticles ?? []);



  function formatDate(dateStr: string): string {
    return dateStr.slice(0, 10);
  }

  // Grid column spans for bento layout
  function projectSpan(index: number): string {
    if (index === 0) return "md:col-span-8";
    if (index === 1) return "md:col-span-4";
    return "md:col-span-12";
  }
</script>

<svelte:head>
  <title>Sam Hite - personal site</title>
  <meta
    name="description"
    content="Software engineer focused on high-performance infrastructure, scalable backend services, and clean, functional interfaces."
  />
</svelte:head>

<!-- Hero Section -->
<section
  class="relative max-w-[1100px] mx-auto px-(--spacing-gutter) py-(--spacing-section-gap) flex flex-col justify-center min-h-[716px]"
>
  <div class="max-w-3xl space-y-8">
    <h1 class="text-5xl md:text-[72px] font-bold tracking-tight text-on-surface mb-2">
      Hello!t
    </h1>
    <p class="text-lg md:text-xl text-on-surface-variant max-w-2xl leading-relaxed">
      I'm Samhith, a developer in high school who's interested in Computer
      Architecture, embedded systems, and machine learning. Check out my
      projects!
    </p>
    <div class="flex flex-col sm:flex-row gap-4 pt-4">
      <a
        href="/projects"
        class="inline-flex items-center justify-center bg-primary-container text-white px-8 py-4 hover:bg-inverse-primary transition-colors duration-200 brand-flat text-label-caps tracking-widest no-underline"
      >
        View Projects
      </a>
      <a
        href="/blog"
        class="inline-flex items-center justify-center border border-white/20 text-on-surface hover:border-primary-container hover:text-primary transition-colors duration-200 px-8 py-4 brand-flat text-label-caps tracking-widest bg-transparent no-underline"
      >
        Read the Blog
      </a>
    </div>
  </div>

  <!-- Downward Arrow Indicator -->
  <div class="absolute bottom-12 left-1/2 -translate-x-1/2 animate-bounce opacity-60 hover:opacity-100 transition-opacity">
    <a href="#projects" class="text-on-surface-variant hover:text-primary no-underline flex justify-center" aria-label="Scroll to projects">
      <span class="material-symbols-outlined text-4xl">keyboard_arrow_down</span>
    </a>
  </div>
</section>

<!-- Featured Projects Section -->
<section
  class="max-w-[1100px] mx-auto px-(--spacing-gutter) pb-(--spacing-section-gap)"
  id="projects"
>
  <div class="mb-12 border-b border-white/10 pb-4">
    <h2 class="text-label-caps text-primary tracking-widest">
      FEATURED PROJECTS
    </h2>
  </div>

  {#if featuredProjects.length > 0}
    <div class="grid grid-cols-1 md:grid-cols-12 gap-(--spacing-base)">
      {#each featuredProjects as project, i}
        <article
          class="{projectSpan(
            i,
          )} bg-surface border border-white/10 p-8 hover:border-primary-container transition-colors duration-300 brand-flat flex flex-col {i ===
          2
            ? 'md:flex-row gap-8'
            : ''} justify-between min-h-[320px] group"
        >
          {#if i < 2}
            <!-- Standard vertical card -->
            <div>
              <div class="flex justify-between items-start mb-6">
                <h3
                  class="text-headline-lg-mobile md:text-headline-lg text-on-surface hover:text-primary transition-colors"
                >
                  <a href="/projects/{project.id}" class="no-underline text-inherit">{project.name}</a>
                </h3>
                <span
                  class="material-symbols-outlined text-on-surface-variant group-hover:text-primary transition-colors"
                  >arrow_outward</span
                >
              </div>
              <p
                class="text-body-md text-on-surface-variant mb-8 {i === 0
                  ? 'max-w-lg'
                  : ''}"
              >
                {project.description}
              </p>
            </div>
            <div class="flex flex-wrap gap-2">
              {#if project.repo_link}
                <a
                  href={project.repo_link}
                  target="_blank"
                  rel="noopener noreferrer"
                  class="border border-white/15 px-3 py-1 text-code-sm text-on-surface-variant brand-flat hover:border-primary hover:text-primary transition-colors no-underline"
                  >Source</a
                >
              {/if}
              {#if project.demo_link}
                <a
                  href={project.demo_link}
                  target="_blank"
                  rel="noopener noreferrer"
                  class="border border-white/15 px-3 py-1 text-code-sm text-on-surface-variant brand-flat hover:border-primary hover:text-primary transition-colors no-underline"
                  >Demo</a
                >
              {/if}
            </div>
          {:else}
            <!-- Full-width horizontal card -->
            <div class="max-w-2xl">
              <h3
                class="text-headline-lg-mobile md:text-headline-lg text-on-surface mb-6 hover:text-primary transition-colors"
              >
                <a href="/projects/{project.id}" class="no-underline text-inherit">{project.name}</a>
              </h3>
              <p class="text-body-md text-on-surface-variant">
                {project.description}
              </p>
            </div>
            <div
              class="flex flex-col items-start md:items-end gap-4 w-full md:w-auto"
            >
              <span
                class="hidden md:block material-symbols-outlined text-on-surface-variant group-hover:text-primary transition-colors"
                >arrow_outward</span
              >
              <div class="flex flex-wrap gap-2">
                {#if project.repo_link}
                  <a
                    href={project.repo_link}
                    target="_blank"
                    rel="noopener noreferrer"
                    class="border border-white/15 px-3 py-1 text-code-sm text-on-surface-variant brand-flat hover:border-primary hover:text-primary transition-colors no-underline"
                    >Source</a
                  >
                {/if}
                {#if project.demo_link}
                  <a
                    href={project.demo_link}
                    target="_blank"
                    rel="noopener noreferrer"
                    class="border border-white/15 px-3 py-1 text-code-sm text-on-surface-variant brand-flat hover:border-primary hover:text-primary transition-colors no-underline"
                    >Demo</a
                  >
                {/if}
              </div>
            </div>
          {/if}
        </article>
      {/each}
    </div>
  {:else}
    <p class="text-on-surface-variant text-body-md">Projects loading…</p>
  {/if}
</section>

<!-- Recent Writing Section -->
<section
  class="max-w-[1100px] mx-auto px-(--spacing-gutter) pb-(--spacing-section-gap)"
  id="blog"
>
  <div class="mb-12 border-b border-white/10 pb-4">
    <h2 class="text-label-caps text-primary tracking-widest">RECENT WRITING</h2>
  </div>

  {#if recentArticles.length > 0}
    <div class="flex flex-col border border-white/10 brand-flat">
      {#each recentArticles as article, i}
        <a
          href="/blog/{article.id}"
          class="group flex flex-col md:flex-row md:items-center gap-4 md:gap-8 p-6 md:p-8 {i <
          recentArticles.length - 1
            ? 'border-b border-white/10'
            : ''} hover:bg-surface-variant/20 transition-colors no-underline"
        >
          <time class="text-code-sm text-on-surface-variant w-32 shrink-0"
            >{formatDate(article.date)}</time
          >
          <h3
            class="text-headline-lg-mobile text-on-surface group-hover:text-primary transition-colors flex-grow"
          >
            {article.title}
          </h3>
          <span
            class="material-symbols-outlined text-on-surface-variant opacity-0 md:group-hover:opacity-100 transition-opacity transform group-hover:translate-x-1"
            >arrow_forward</span
          >
        </a>
      {/each}
    </div>
  {:else}
    <p class="text-on-surface-variant text-body-md">Articles loading…</p>
  {/if}
</section>
