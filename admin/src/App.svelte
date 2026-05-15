<script lang="ts">
  import {
    Activity,
    ArrowLeft,
    BarChart3,
    BookOpen,
    CheckCircle2,
    ExternalLink,
    FileText,
    FolderKanban,
    KeyRound,
    LayoutDashboard,
    LogOut,
    Plus,
    RefreshCw,
    Save,
    Search,
    Settings,
    ShieldCheck,
    Wrench,
    X
  } from "lucide-svelte";
  import { onMount } from "svelte";
  import { Carta, Markdown, MarkdownEditor } from "carta-md";
  import "carta-md/default.css";
  import "@cartamd/plugin-emoji/default.css";
  import "katex/dist/katex.min.css";
  import { attachment } from "@cartamd/plugin-attachment";
  import { code } from "@cartamd/plugin-code";
  import { emoji } from "@cartamd/plugin-emoji";
  import { math } from "@cartamd/plugin-math";
  import { slash } from "@cartamd/plugin-slash";
  import { tikz } from "@cartamd/plugin-tikz";
  import DOMPurify from "isomorphic-dompurify";

  import { api } from "./api";
  import { hasFirebaseConfig, signIn, signOutAdmin, watchToken } from "./firebase";
  import type { Article, ArticleMeta, ChartPoint, Project, ProjectInput, Stats } from "./types";

  type Page =
    | "overview"
    | "articles"
    | "article-view"
    | "projects"
    | "project-edit"
    | "settings";

  const carta = new Carta({
    sanitizer: DOMPurify.sanitize,
    theme: "catppuccin-mocha",
    extensions: [
      code({ theme: "catppuccin-mocha" }),
      math(),
      emoji(),
      slash(),
      tikz(),
      attachment({ upload: fileToDataUrl })
    ]
  });

  const navItems: Array<{ page: Page; label: string; icon: typeof LayoutDashboard }> = [
    { page: "overview", label: "Overview", icon: LayoutDashboard },
    { page: "articles", label: "Articles", icon: FileText },
    { page: "projects", label: "Projects", icon: FolderKanban },
    { page: "settings", label: "Settings", icon: Settings }
  ];

  let page: Page = "overview";
  let token = api.token;
  let apiBaseUrl = api.baseUrl;
  let email = localStorage.getItem("adminEmail") ?? "";
  let password = "";
  let signedInAs = "";
  let stats: Stats | null = null;
  let chart: ChartPoint[] = [];
  let articles: ArticleMeta[] = [];
  let projects: Project[] = [];
  let selectedArticle: Article | null = null;
  let selectedProject: Project | null = null;
  let search = "";
  let message = "";
  let error = "";
  let loading = false;
  let articleComposerOpen = false;
  let projectComposerOpen = false;

  let articleForm = emptyArticle();
  let projectForm: ProjectInput = emptyProject();

  $: chartMax = Math.max(1, ...chart.map((point) => point.views));
  $: filteredArticles = articles.filter((article) =>
    `${article.title} ${article.description} ${article.id}`.toLowerCase().includes(search.toLowerCase())
  );
  $: filteredProjects = projects.filter((project) =>
    `${project.name} ${project.description} ${project.id}`.toLowerCase().includes(search.toLowerCase())
  );
  $: pageTitle = navItems.find((item) => item.page === page)?.label ?? "Admin";

  onMount(() => {
    const unwatch = watchToken((user, nextToken) => {
      signedInAs = user?.email ?? "";
      token = nextToken;
      api.saveToken(nextToken);
      if (nextToken) void refresh();
    });

    page = routeFromHash();
    window.addEventListener("hashchange", syncRoute);
    void refresh();

    return () => {
      window.removeEventListener("hashchange", syncRoute);
      unwatch();
    };
  });

  function syncRoute() {
    page = routeFromHash();
    error = "";
    message = "";
  }

  function routeFromHash(): Page {
    const route = window.location.hash.replace("#/", "");
    if (route === "article-new") return "articles";
    if (route === "project-new") return "projects";
    return navItems.some((item) => item.page === route) || route === "article-view" || route === "project-edit"
      ? (route as Page)
      : "overview";
  }

  function navigate(next: Page) {
    window.location.hash = `/${next}`;
  }

  async function refresh() {
    loading = true;
    error = "";
    try {
      const publicData = Promise.all([api.articles(), api.projects()]);
      const adminData = token ? Promise.all([api.stats(), api.charts()]) : Promise.resolve([stats, chart] as const);
      const [[nextArticles, nextProjects], [nextStats, nextChart]] = await Promise.all([
        publicData,
        adminData
      ]);

      articles = nextArticles;
      projects = nextProjects;
      stats = nextStats;
      chart = nextChart;
    } catch (err) {
      error = err instanceof Error ? err.message : "Unable to refresh data";
    } finally {
      loading = false;
    }
  }

  async function login() {
    error = "";
    message = "";
    try {
      const nextToken = await signIn(email, password);
      api.saveToken(nextToken);
      localStorage.setItem("adminEmail", email);
      token = nextToken;
      password = "";
      message = "Signed in";
      await refresh();
    } catch (err) {
      error = err instanceof Error ? err.message : "Unable to sign in";
    }
  }

  async function logout() {
    await signOutAdmin();
    api.saveToken("");
    token = "";
    signedInAs = "";
    stats = null;
    chart = [];
    message = "Signed out";
  }

  async function saveSettings() {
    api.baseUrl = apiBaseUrl.trim();
    api.saveToken(token);
    message = "Settings saved";
    await refresh();
  }

  async function createArticle() {
    error = "";
    message = "";
    try {
      const created = await api.createArticle(articleForm);
      message = `Article saved: ${created.title}`;
      articleForm = emptyArticle();
      await refresh();
      articleComposerOpen = false;
      navigate("articles");
    } catch (err) {
      error = err instanceof Error ? err.message : "Unable to save article";
    }
  }

  async function openArticle(id: string) {
    error = "";
    selectedArticle = null;
    try {
      selectedArticle = await api.article(id);
      navigate("article-view");
    } catch (err) {
      error = err instanceof Error ? err.message : "Unable to load article";
    }
  }

  async function createProject() {
    error = "";
    message = "";
    try {
      const created = await api.createProject(normalizeProject(projectForm));
      message = `Project created: ${created.name}`;
      projectForm = emptyProject();
      await refresh();
      projectComposerOpen = false;
      navigate("projects");
    } catch (err) {
      error = err instanceof Error ? err.message : "Unable to create project";
    }
  }

  function editProject(project: Project) {
    selectedProject = { ...project };
    navigate("project-edit");
  }

  async function saveProject() {
    if (!selectedProject) return;
    error = "";
    message = "";
    try {
      const updated = await api.updateProject(selectedProject.id, {
        name: selectedProject.name,
        description: selectedProject.description,
        demo_link: selectedProject.demo_link,
        repo_link: selectedProject.repo_link,
        readme_type: selectedProject.readme_type,
        readme_content: selectedProject.readme_content
      });
      projects = projects.map((project) => (project.id === updated.id ? updated : project));
      selectedProject = updated;
      message = `Project saved: ${updated.name}`;
    } catch (err) {
      error = err instanceof Error ? err.message : "Unable to save project";
    }
  }

  async function fileToDataUrl(file: File) {
    return new Promise<string | null>((resolve) => {
      const reader = new FileReader();
      reader.addEventListener("load", () => resolve(typeof reader.result === "string" ? reader.result : null));
      reader.addEventListener("error", () => resolve(null));
      reader.readAsDataURL(file);
    });
  }

  function emptyArticle() {
    return {
      id: "",
      title: "",
      description: "",
      abstract_markdown: "",
      content: "",
      is_published: true
    };
  }

  function emptyProject(): ProjectInput {
    return {
      id: "",
      name: "",
      description: "",
      demo_link: null,
      repo_link: null,
      readme_type: "raw",
      readme_content: ""
    };
  }

  function normalizeProject(project: ProjectInput): ProjectInput {
    return {
      ...project,
      demo_link: project.demo_link || null,
      repo_link: project.repo_link || null
    };
  }
</script>

<main class="app-shell">
  <aside class="sidebar">
    <div class="brand">
      <div class="brand-mark"><Wrench size={22} /></div>
      <div>
        <strong>samhithe.dev</strong>
        <span>Admin Console</span>
      </div>
    </div>

    <nav class="nav" aria-label="Admin navigation">
      {#each navItems as item}
        <button
          class:active={page === item.page}
          type="button"
          on:click={() => navigate(item.page)}
        >
          <svelte:component this={item.icon} size={18} />
          {item.label}
        </button>
      {/each}
    </nav>

    <div class="session-card">
      <ShieldCheck size={18} />
      <div>
        <span>{token ? "Authenticated" : "Locked"}</span>
        <strong>{signedInAs || "No admin session"}</strong>
      </div>
    </div>
  </aside>

  <section class="main">
    <header class="topbar">
      <div>
        <span class="eyebrow">Firebase Firestore</span>
        <h1>{pageTitle}</h1>
      </div>
      <div class="top-actions">
        <button type="button" on:click={refresh} disabled={loading}>
          <RefreshCw size={18} />
          Refresh
        </button>
        <a href={apiBaseUrl} target="_blank" rel="noreferrer">
          <ExternalLink size={18} />
          API
        </a>
      </div>
    </header>

    {#if message}
      <p class="notice success">{message}</p>
    {/if}
    {#if error}
      <p class="notice error">{error}</p>
    {/if}

    {#if page === "overview"}
      <section class="metrics">
        <article>
          <Activity size={20} />
          <span>Global views</span>
          <strong>{stats?.global_views ?? 0}</strong>
        </article>
        <article>
          <BarChart3 size={20} />
          <span>Trending 48h</span>
          <strong>{stats?.trending ?? 0}</strong>
        </article>
        <article>
          <FileText size={20} />
          <span>Best performer</span>
          <strong>{stats?.best_performer?.title ?? "None"}</strong>
        </article>
        <article>
          <CheckCircle2 size={20} />
          <span>Broken links</span>
          <strong>{stats?.broken_projects ?? 0}</strong>
        </article>
      </section>

      <section class="panel">
        <div class="section-title">
          <BarChart3 size={20} />
          <h2>Views by day</h2>
        </div>
        <div class="chart" aria-label="Views chart">
          {#each chart as point}
            <div class="bar" style={`height: ${Math.max(5, (point.views / chartMax) * 100)}%`}>
              <span>{point.views}</span>
            </div>
          {/each}
        </div>
      </section>
    {:else if page === "articles"}
      <section class="page-grid">
        {#if articleComposerOpen}
          <form class="compose-screen" on:submit|preventDefault={createArticle}>
            <div class="compose-header">
              <div class="form-header">
                <BookOpen size={22} />
                <div>
                  <h2>New article</h2>
                  <p>Write in Markdown with live preview, attachments, math, emoji, code, and TikZ support.</p>
                </div>
              </div>
              <button type="button" on:click={() => (articleComposerOpen = false)}>
                <X size={18} />
                Close
              </button>
            </div>
            <div class="grid-two">
              <label>Slug<input bind:value={articleForm.id} required /></label>
              <label>Title<input bind:value={articleForm.title} required /></label>
            </div>
            <label>Description<input bind:value={articleForm.description} required /></label>
            <div class="editor-grid">
              <div class="editor-panel compact">
                <span class="field-label">Abstract</span>
                <MarkdownEditor {carta} bind:value={articleForm.abstract_markdown} mode="tabs" theme="admin" placeholder="Write a concise abstract" />
              </div>
              <div class="editor-panel feature">
                <span class="field-label">Content</span>
                <MarkdownEditor {carta} bind:value={articleForm.content} mode="tabs" theme="admin" placeholder="Write the article in Markdown" />
              </div>
            </div>
            <div class="form-actions">
              <label class="check"><input type="checkbox" bind:checked={articleForm.is_published} /> Published</label>
              <button class="primary" type="submit"><Save size={18} /> Publish Article</button>
            </div>
          </form>
        {:else}
          <div class="toolbar">
            <label class="search">
              <Search size={18} />
              <input bind:value={search} placeholder="Search articles" />
            </label>
            <button class="primary" type="button" on:click={() => (articleComposerOpen = true)}>
              <Plus size={18} />
              New Article
            </button>
          </div>
          <div class="data-list">
            {#each filteredArticles as article}
              <article class="row-card">
                <div>
                  <span>{article.id}</span>
                  <h2>{article.title}</h2>
                  <p>{article.description}</p>
                </div>
                <div class="row-meta">
                  <strong>{article.views}</strong>
                  <span>views</span>
                  <button type="button" on:click={() => openArticle(article.id)}>View</button>
                </div>
              </article>
            {/each}
          </div>
        {/if}
      </section>
    {:else if page === "article-view"}
      <section class="detail-screen">
        <button type="button" on:click={() => navigate("articles")}><ArrowLeft size={18} /> Articles</button>
        {#if selectedArticle}
          <article class="reader">
            <span>{selectedArticle.id}</span>
            <h2>{selectedArticle.title}</h2>
            <p>{selectedArticle.description}</p>
            <div class="mini-metrics">
              <span>{selectedArticle.word_count} words</span>
              <span>{selectedArticle.image_count} images</span>
              <span>{selectedArticle.heading_count} headings</span>
              <span>{selectedArticle.views} views</span>
            </div>
            <h3>Abstract</h3>
            <div class="markdown-render">
              {#key selectedArticle.abstract_markdown}
                <Markdown {carta} value={selectedArticle.abstract_markdown} theme="admin" />
              {/key}
            </div>
            <h3>Content</h3>
            <div class="markdown-render">
              {#key selectedArticle.content}
                <Markdown {carta} value={selectedArticle.content} theme="admin" />
              {/key}
            </div>
          </article>
        {/if}
      </section>
    {:else if page === "projects"}
      <section class="page-grid">
        {#if projectComposerOpen}
          <form class="compose-screen" on:submit|preventDefault={createProject}>
            <div class="compose-header">
              <div class="form-header">
                <FolderKanban size={22} />
                <div>
                  <h2>New project</h2>
                  <p>Create a project document with README source and link health tracking.</p>
                </div>
              </div>
              <button type="button" on:click={() => (projectComposerOpen = false)}>
                <X size={18} />
                Close
              </button>
            </div>
            <div class="grid-two">
              <label>Slug<input bind:value={projectForm.id} required /></label>
              <label>Name<input bind:value={projectForm.name} required /></label>
            </div>
            <label>Description<textarea bind:value={projectForm.description} rows="4" required></textarea></label>
            <div class="grid-two">
              <label>Demo link<input bind:value={projectForm.demo_link} /></label>
              <label>Repo link<input bind:value={projectForm.repo_link} /></label>
            </div>
            <div class="grid-two">
              <label>
                README mode
                <select bind:value={projectForm.readme_type}>
                  <option value="url">URL</option>
                  <option value="raw">Raw Markdown</option>
                </select>
              </label>
              <div class="inline-hint">
                <strong>{projectForm.readme_type === "url" ? "URL mode" : "Raw Markdown mode"}</strong>
                <span>{projectForm.readme_type === "url" ? "Paste the GitHub README URL below." : "Use the live editor for a custom README."}</span>
              </div>
            </div>
            <div class="editor-panel feature">
              <span class="field-label">README</span>
              <MarkdownEditor {carta} bind:value={projectForm.readme_content} mode="tabs" theme="admin" placeholder="Paste a README URL or write raw Markdown" />
            </div>
            <div class="form-actions">
              <button class="primary" type="submit"><Save size={18} /> Create Project</button>
            </div>
          </form>
        {:else}
          <div class="toolbar">
            <label class="search">
              <Search size={18} />
              <input bind:value={search} placeholder="Search projects" />
            </label>
            <button class="primary" type="button" on:click={() => (projectComposerOpen = true)}>
              <Plus size={18} />
              New Project
            </button>
          </div>
          <div class="data-list">
            {#each filteredProjects as project}
              <article class="row-card">
                <div>
                  <span>{project.id}</span>
                  <h2>{project.name}</h2>
                  <p>{project.description}</p>
                </div>
                <div class="row-meta">
                  <p class={`health ${project.health_status}`}>{project.health_status}</p>
                  <button type="button" on:click={() => editProject(project)}>Open</button>
                </div>
              </article>
            {/each}
          </div>
        {/if}
      </section>
    {:else if page === "project-edit"}
      <section class="detail-screen">
        <button type="button" on:click={() => navigate("projects")}><ArrowLeft size={18} /> Projects</button>
        {#if selectedProject}
          <form class="form-screen" on:submit|preventDefault={saveProject}>
            <div class="form-header">
              <FolderKanban size={22} />
              <div>
                <h2>{selectedProject.name}</h2>
                <p>{selectedProject.id}</p>
              </div>
            </div>
            <label>Name<input bind:value={selectedProject.name} /></label>
            <label>Description<textarea bind:value={selectedProject.description} rows="4"></textarea></label>
            <div class="grid-two">
              <label>Demo link<input bind:value={selectedProject.demo_link} /></label>
              <label>Repo link<input bind:value={selectedProject.repo_link} /></label>
            </div>
            <label>
              README mode
              <select bind:value={selectedProject.readme_type}>
                <option value="url">URL</option>
                <option value="raw">Raw Markdown</option>
              </select>
            </label>
            <div class="editor-panel feature">
              <span class="field-label">README</span>
              <MarkdownEditor {carta} bind:value={selectedProject.readme_content} mode="tabs" theme="admin" placeholder="Paste a README URL or write raw Markdown" />
            </div>
            <p class={`health ${selectedProject.health_status}`}>{selectedProject.health_status}</p>
            <button class="primary" type="submit"><Save size={18} /> Save Project</button>
          </form>
        {/if}
      </section>
    {:else if page === "settings"}
      <section class="settings-grid">
        <form class="form-screen" on:submit|preventDefault={login}>
          <div class="form-header">
            <KeyRound size={22} />
            <div>
              <h2>Firebase sign in</h2>
              <p>Sign in once; the console keeps the Firebase ID token fresh.</p>
            </div>
          </div>
          <label>Email<input bind:value={email} type="email" autocomplete="email" disabled={!hasFirebaseConfig} /></label>
          <label>Password<input bind:value={password} type="password" autocomplete="current-password" disabled={!hasFirebaseConfig} /></label>
          <button class="primary" type="submit" disabled={!hasFirebaseConfig}><KeyRound size={18} /> Sign In</button>
          {#if signedInAs}
            <button type="button" on:click={logout}><LogOut size={18} /> Sign Out {signedInAs}</button>
          {/if}
        </form>
        <form class="form-screen" on:submit|preventDefault={saveSettings}>
          <div class="form-header">
            <Settings size={22} />
            <div>
              <h2>API settings</h2>
              <p>Point this console at a local or production backend.</p>
            </div>
          </div>
          <label>API base URL<input bind:value={apiBaseUrl} spellcheck="false" /></label>
          <p class={`token-state ${token ? "ready" : ""}`}>{token ? "Firebase token ready" : "Sign in to unlock admin routes"}</p>
          {#if !hasFirebaseConfig}
            <p class="notice error">Firebase web config is missing in admin .env.</p>
          {/if}
          <button class="primary" type="submit"><Save size={18} /> Save Settings</button>
        </form>
      </section>
    {/if}
  </section>
</main>
