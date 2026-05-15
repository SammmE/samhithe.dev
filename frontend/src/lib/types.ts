export interface Project {
	id: string;
	name: string;
	description: string;
	demo_link: string | null;
	repo_link: string | null;
	readme_type: 'url' | 'raw';
	readme_content: string;
	health_status: 'unknown' | 'healthy' | 'broken';
	last_health_check: string | null;
}

export interface ArticleMeta {
	id: string;
	title: string;
	description: string;
	date: string;
	word_count: number;
	image_count: number;
	heading_count: number;
	views: number;
	is_published: boolean;
}

export interface Article extends ArticleMeta {
	abstract_markdown: string;
	content: string;
}
