import type { Project, ArticleMeta, Article } from './types';

const API_BASE = import.meta.env.PUBLIC_API_BASE ?? 'http://localhost:3000';


async function apiFetch<T>(path: string, init?: RequestInit): Promise<T> {
	const res = await fetch(API_BASE + path, init);
	if (!res.ok) {
		throw new Error(`API error: ${res.status} ${res.statusText}`);
	}
	return res.json() as Promise<T>;
}

export function fetchProjects(): Promise<Project[]> {
	return apiFetch<Project[]>('/projects');
}

export function fetchArticles(): Promise<ArticleMeta[]> {
	return apiFetch<ArticleMeta[]>('/articles');
}

export function fetchArticle(id: string): Promise<Article> {
	return apiFetch<Article>(`/articles/${encodeURIComponent(id)}`);
}

export function recordHit(id: string): Promise<{ counted: boolean }> {
	return apiFetch<{ counted: boolean }>(`/hit/${encodeURIComponent(id)}`, {
		method: 'POST'
	});
}
