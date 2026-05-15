import type { PageServerLoad } from './$types';
import { fetchProjects } from '$lib/api';

export const load: PageServerLoad = async () => {
	const projects = await fetchProjects().catch(() => []);
	return { projects };
};
