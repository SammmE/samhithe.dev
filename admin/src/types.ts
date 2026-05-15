export type ArticleMeta = {
  id: string;
  title: string;
  description: string;
  date: string;
  word_count: number;
  image_count: number;
  heading_count: number;
  views: number;
  is_published: boolean;
};

export type Article = ArticleMeta & {
  abstract_markdown: string;
  content: string;
};

export type Project = {
  id: string;
  name: string;
  description: string;
  demo_link: string | null;
  repo_link: string | null;
  readme_type: "url" | "raw";
  readme_content: string;
  health_status: "unknown" | "healthy" | "broken";
  last_health_check: string | null;
};

export type ProjectInput = {
  id: string;
  name: string;
  description: string;
  demo_link: string | null;
  repo_link: string | null;
  readme_type: "url" | "raw";
  readme_content: string;
};

export type Stats = {
  global_views: number;
  best_performer: ArticleMeta | null;
  trending: number;
  broken_projects: number;
};

export type ChartPoint = {
  date: string;
  views: number;
};
