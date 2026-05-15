import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { fetchProjects } from '$lib/api';

export const load: PageServerLoad = async ({ params, fetch }) => {
	const projects = await fetchProjects().catch(() => []);
	const project = projects.find(p => p.id === params.id);

	if (!project) {
		throw error(404, 'Project not found');
	}

	let markdownContent = '';

	if (project.readme_type === 'url' && project.readme_content) {
		try {
			const res = await fetch(project.readme_content);
			if (res.ok) {
				markdownContent = await res.text();
			} else {
				markdownContent = '> Failed to load project readme from URL.';
			}
		} catch (e) {
			markdownContent = '> Failed to load project readme.';
		}
	} else {
		markdownContent = project.readme_content || '';
	}

	return {
		project,
		markdownContent
	};
};
