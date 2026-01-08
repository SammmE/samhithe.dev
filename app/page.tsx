import Link from 'next/link';
import Typewriter from '@/components/Typewriter';
import ProjectCard from '@/components/ProjectCard';
import { projects } from '#site/content';

export default function Home() {
    const featuredProjects = projects.filter((p) => p.featured);

    return (
        <div className="space-y-20">
            {/* Hero Section */}
            <section className="min-h-[60vh] flex flex-col justify-center max-w-4xl">
                <h1 className="text-4xl md:text-6xl font-bold mb-6 leading-tight">
                    Hi, I&apos;m Sam.<br />
                    <span className="text-gray-400 text-xl md:text-3xl font-normal block mt-4 leading-relaxed">
                        <Typewriter text="Building high-performance systems at the intersection of low-level computing and aviation." delay={500} speed={30} />
                    </span>
                </h1>

                <div className="flex items-center gap-3 mt-8 text-sm font-mono text-terminal-green bg-terminal-green/5 w-fit px-4 py-2 rounded-full border border-terminal-green/20">
                    <span className="relative flex h-2 w-2">
                        <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-terminal-green opacity-75"></span>
                        <span className="relative inline-flex rounded-full h-2 w-2 bg-terminal-green"></span>
                    </span>
                    Current Mission: Architecting a neural network kernel in x86 Assembly.
                </div>
            </section>

            {/* Featured Projects */}
            <section>
                <div className="flex justify-between items-end mb-8">
                    <h2 className="text-2xl font-mono text-white">Featured Projects</h2>
                    <Link href="/projects" className="text-sm font-mono text-gray-500 hover:text-tech-blue transition-colors">See all --&gt;</Link>
                </div>
                <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                    {featuredProjects.map((project) => (
                        <ProjectCard
                            key={project.slug}
                            slug={project.slug}
                            title={project.title}
                            problem={project.problem}
                            stack={project.stack}
                            hurdle={project.hurdle}
                            tags={project.tags}
                            link={project.link}
                            github_link={project.github_link}
                        />
                    ))}
                </div>
            </section>
        </div>
    );
}
