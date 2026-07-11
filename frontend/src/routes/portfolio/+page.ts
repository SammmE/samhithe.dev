import type { PageLoad } from './$types';
import { fetchProjects } from '$lib/api';

export const load: PageLoad = async () => {
	const projects = await fetchProjects();
	return { projects };
};
