// API Response Types

export interface Stats {
	uptime_seconds: number;
	memory_usage_mb: number;
	engine: string;
	buffered_views_size: number;
}

export interface Project {
	id: number;
	title: string;
	repo_url: string;
	demo_url?: string;
	readme_content?: string;
	view_count: number;
	priority: number;
	created_at: string;
	updated_at?: string;
}

export interface Log {
	id: number;
	content: string;
	view_count: number;
	created_at: string;
	updated_at?: string;
}

export interface ProjectCreateInput {
	title: string;
	repo_url: string;
	demo_url?: string;
	priority?: number;
}

export interface ProjectUpdateInput {
	title?: string;
	repo_url?: string;
	demo_url?: string;
	priority?: number;
}

export interface LogCreateInput {
	content: string;
}

export interface LogUpdateInput {
	content: string;
}

export interface ApiError {
	message: string;
	status?: number;
}
