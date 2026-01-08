import { projects } from '#site/content'
import { MDXContent } from '@/components/mdx-content'
import { notFound } from 'next/navigation'
import Link from 'next/link'

export async function generateStaticParams() {
  return projects.map((project) => ({
    slug: project.slug.split('/').pop(),
  }))
}

export default function ProjectPage({ params }: { params: { slug: string } }) {
  const project = projects.find((p) => p.slug.split('/').pop() === params.slug)

  if (!project) notFound()

  return (
    <article className="prose prose-invert prose-headings:text-text-white max-w-none">
        <div className="mb-8 border-b border-gray-800 pb-8">
            <h1 className="text-4xl font-bold text-text-white mb-4">{project.title}</h1>
            <div className="flex gap-4 items-center">
                {project.github_link && (
                    <Link
                        href={project.github_link}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="inline-flex items-center gap-2 text-text-offwhite hover:text-tech-blue transition-colors px-4 py-2 border border-gray-700 rounded-md hover:border-tech-blue"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                            <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path>
                        </svg>
                        View Source
                    </Link>
                )}
                 <div className="flex gap-2 mt-4">
                    {project.stack.map(tech => <span key={tech} className="text-sm text-gray-400 border border-gray-700 rounded px-2 py-1">{tech}</span>)}
                </div>
            </div>
        </div>
        
        <MDXContent code={project.content} />
    </article>
  )
}