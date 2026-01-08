import Link from 'next/link';

interface ProjectCardProps {
    title: string;
    problem: string;
    stack: string[];
    hurdle: string;
    tags?: string[];
    link?: string;
    github_link?: string;
    slug: string;
}

const ProjectCard = ({ title, problem, stack, hurdle, tags, link, github_link, slug }: ProjectCardProps) => {
    // Determine the main link destination
    // If link is present and not '#', use it. Otherwise, use the internal project page.
    const hasExternalLink = link && link !== '#';
    const mainLink = hasExternalLink ? link : `/${slug}`;
    const projectPageLink = `/${slug}`;

    return (
        <div className="group relative bg-card-bg border border-white/10 p-6 rounded-lg transition-transform duration-300 hover:-translate-y-1 hover:border-tech-blue/50">
            {/* Main Clickable Area (Stretched Link) */}
            {hasExternalLink ? (
                <a href={mainLink} target="_blank" rel="noopener noreferrer" className="absolute inset-0 z-0">
                    <span className="sr-only">View Project</span>
                </a>
            ) : (
                <Link href={mainLink} className="absolute inset-0 z-0">
                    <span className="sr-only">View Project</span>
                </Link>
            )}

            <div className="flex justify-between items-start mb-4 relative z-10 pointer-events-none">
                <h3 className="text-xl text-text-offwhite font-bold group-hover:text-tech-blue transition-colors pointer-events-auto">
                    {title}
                </h3>
                
                <div className="flex gap-3 pointer-events-auto">
                    {/* GitHub Icon */}
                    {github_link && (
                        <a 
                            href={github_link} 
                            target="_blank" 
                            rel="noopener noreferrer" 
                            className="text-gray-500 hover:text-white transition-colors p-1"
                            title="View Source on GitHub"
                        >
                            <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                                <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path>
                            </svg>
                        </a>
                    )}
                    
                    {/* Evaluation/Description Icon */}
                    <Link 
                        href={projectPageLink}
                        className="text-gray-500 hover:text-tech-blue transition-colors p-1" 
                        title="Read Project Log"
                    >
                         <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                            <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
                            <polyline points="14 2 14 8 20 8"></polyline>
                            <line x1="16" y1="13" x2="8" y2="13"></line>
                            <line x1="16" y1="17" x2="8" y2="17"></line>
                            <polyline points="10 9 9 9 8 9"></polyline>
                        </svg>
                    </Link>
                </div>
            </div>

            <div className="mb-4 relative z-10 pointer-events-none">
                <h4 className="text-xs font-mono uppercase text-gray-500 mb-1">The Problem</h4>
                <p className="text-sm text-gray-300 leading-relaxed">{problem}</p>
            </div>

            <div className="mb-4 relative z-10 pointer-events-none">
                <h4 className="text-xs font-mono uppercase text-gray-500 mb-1">The Stack</h4>
                <div className="flex flex-wrap gap-2 text-xs font-mono text-tech-blue">
                    {stack.map((item) => (
                        <span key={item} className="bg-tech-blue/10 px-2 py-1 rounded">{item}</span>
                    ))}
                </div>
            </div>

            <div className="relative z-10 pointer-events-none">
                <h4 className="text-xs font-mono uppercase text-gray-500 mb-1">The Hardest Part</h4>
                <p className="text-sm text-gray-300 italic">&quot;{hurdle}&quot;</p>
            </div>

            {tags && (
                <div className="mt-4 pt-4 border-t border-white/5 flex gap-2 relative z-10 pointer-events-none">
                    {tags.map(tag => (
                        <span key={tag} className="text-[10px] uppercase tracking-wider text-gray-600 border border-gray-700 px-1.5 py-0.5 rounded">{tag}</span>
                    ))}
                </div>
            )}
        </div>
    );
};

export default ProjectCard;
