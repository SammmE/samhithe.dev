import type { PageServerLoad } from './$types';
import { fetchArticles } from '$lib/api';

export const load: PageServerLoad = async () => {
	const articles = await fetchArticles().catch(() => []);
	return { articles };
};
