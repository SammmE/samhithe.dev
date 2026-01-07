interface ProjectCardProps {
    title: string;
    problem: string;
    stack: string[];
    hurdle: string;
    tags?: string[];
    link?: string;
}

const ProjectCard = ({ title, problem, stack, hurdle, tags, link }: ProjectCardProps) => {
    return (
        <div className="group bg-card-bg border border-white/10 p-6 rounded-lg transition-transform duration-300 hover:-translate-y-1 hover:border-tech-blue/50">
            <div className="flex justify-between items-start mb-4">
                <h3 className="text-xl text-text-offwhite font-bold group-hover:text-tech-blue transition-colors">
                    {title}
                </h3>
                {link && (
                    <a href={link} target="_blank" rel="noopener noreferrer" className="text-gray-500 hover:text-white transition-colors">
                        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path><polyline points="15 3 21 3 21 9"></polyline><line x1="10" y1="14" x2="21" y2="3"></line></svg>
                    </a>
                )}
            </div>

            <div className="mb-4">
                <h4 className="text-xs font-mono uppercase text-gray-500 mb-1">The Problem</h4>
                <p className="text-sm text-gray-300 leading-relaxed">{problem}</p>
            </div>

            <div className="mb-4">
                <h4 className="text-xs font-mono uppercase text-gray-500 mb-1">The Stack</h4>
                <div className="flex flex-wrap gap-2 text-xs font-mono text-tech-blue">
                    {stack.map((item) => (
                        <span key={item} className="bg-tech-blue/10 px-2 py-1 rounded">{item}</span>
                    ))}
                </div>
            </div>

            <div>
                <h4 className="text-xs font-mono uppercase text-gray-500 mb-1">The Hardest Part</h4>
                <p className="text-sm text-gray-300 italic">"{hurdle}"</p>
            </div>

            {tags && (
                <div className="mt-4 pt-4 border-t border-white/5 flex gap-2">
                    {tags.map(tag => (
                        <span key={tag} className="text-[10px] uppercase tracking-wider text-gray-600 border border-gray-700 px-1.5 py-0.5 rounded">{tag}</span>
                    ))}
                </div>
            )}
        </div>
    );
};

export default ProjectCard;
