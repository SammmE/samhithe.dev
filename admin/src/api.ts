import type { Article, ArticleMeta, ChartPoint, Project, ProjectInput, Stats } from "./types";

const API_BASE = import.meta.env.VITE_API_BASE_URL ?? "https://api.samhithe.dev/";

export class ApiClient {
  baseUrl = API_BASE;
  token = localStorage.getItem("firebaseToken") ?? "";

  saveToken(token: string) {
    this.token = token.trim();
    localStorage.setItem("firebaseToken", this.token);
  }

  async articles() {
    return this.request<ArticleMeta[]>("articles");
  }

  async article(id: string) {
    return this.request<Article>(`articles/${encodeURIComponent(id)}`);
  }

  async projects() {
    return this.request<Project[]>("projects");
  }

  async stats() {
    return this.request<Stats>("admin/stats", { admin: true });
  }

  async charts() {
    return this.request<ChartPoint[]>("admin/charts", { admin: true });
  }

  async createArticle(payload: Record<string, unknown>) {
    return this.request<ArticleMeta>("admin/articles", {
      admin: true,
      method: "POST",
      body: payload
    });
  }

  async createProject(payload: ProjectInput) {
    return this.request<Project>("admin/projects", {
      admin: true,
      method: "POST",
      body: payload
    });
  }

  async updateProject(id: string, payload: Record<string, unknown>) {
    return this.request<Project>(`admin/projects/${encodeURIComponent(id)}`, {
      admin: true,
      method: "PATCH",
      body: payload
    });
  }

  private async request<T>(
    path: string,
    options: { admin?: boolean; method?: string; body?: unknown } = {}
  ): Promise<T> {
    const headers: Record<string, string> = {
      "Content-Type": "application/json"
    };

    if (options.admin) {
      headers.Authorization = `Bearer ${this.token}`;
    }

    const response = await fetch(`${this.baseUrl}${path}`, {
      method: options.method ?? "GET",
      headers,
      body: options.body ? JSON.stringify(options.body) : undefined
    });

    if (!response.ok) {
      const message = await response.text();
      throw new Error(message || `Request failed with ${response.status}`);
    }

    return response.json() as Promise<T>;
  }
}

export const api = new ApiClient();
