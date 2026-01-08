import { defineCollection, defineConfig, s } from 'velite'

// Define the "Project" Schema
const projects = defineCollection({
  name: 'Project',
  pattern: 'projects/**/*.mdx', // Where your files are
  schema: s.object({
    slug: s.path(), // Auto-generate slug from filename
    title: s.string().max(99),
    problem: s.string(),
    stack: s.array(s.string()),
    tags: s.array(s.string()).optional(), // Make optional just in case
    hurdle: s.string(),
    link: s.string().optional(),
    github_link: s.string().optional(),
    featured: s.boolean().default(false),
    content: s.mdx(), // Compiles MDX to executable code
  })
})

// Define the "Post" Schema
const posts = defineCollection({
  name: 'Post',
  pattern: 'posts/**/*.mdx',
  schema: s.object({
    slug: s.path(),
    title: s.string(),
    date: s.string(), // You can even coerce this to a Date object automatically
    tags: s.array(s.string()),
    preview: s.string(),
    content: s.mdx(),
  })
})

export default defineConfig({
  root: 'src/content', // Your content folder
  collections: { projects, posts },
  mdx: {
    rehypePlugins: [], 
    remarkPlugins: [], // Velite handles the GFM versions internally usually
  }
})
