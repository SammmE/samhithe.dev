import type { PageServerLoad } from './$types';
import { fetchArticle } from '$lib/api';
import { error } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ params }) => {
	try {
		const article = await fetchArticle(params.id);
		return { article };
	} catch (e) {
		throw error(404, 'Article not found');
	}
};
