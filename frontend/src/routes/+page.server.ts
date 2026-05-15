import type { PageServerLoad } from './$types';
import { fetchProjects, fetchArticles } from '$lib/api';

export const load: PageServerLoad = async ({ fetch }) => {
	const [projects, articles] = await Promise.all([
		fetchProjects().catch(() => []),
		fetchArticles().catch(() => [])
	]);

	return {
		featuredProjects: projects.slice(0, 3),
		recentArticles: articles.slice(0, 2)
	};
};
