import { defineCollection, defineConfig, s } from 'velite'

const projects = defineCollection({
  name: 'Project',
  pattern: 'projects/**/*.mdx',
  schema: s.object({
    slug: s.path(),
    title: s.string().max(99),
    problem: s.string(),
    stack: s.array(s.string()),
    tags: s.array(s.string()).optional(),
    hurdle: s.string(),
    link: s.string().optional(),
    github_link: s.string().optional(),
    featured: s.boolean().default(false),
    content: s.mdx(),
  })
})

const posts = defineCollection({
  name: 'Post',
  pattern: 'posts/**/*.mdx',
  schema: s.object({
    slug: s.path(),
    title: s.string(),
    date: s.string(),
    tags: s.array(s.string()),
    preview: s.string(),
    content: s.mdx(),
  })
})

export default defineConfig({
  root: 'content',
  collections: { projects, posts },
  mdx: {
    rehypePlugins: [], 
    remarkPlugins: [],
  }
})
