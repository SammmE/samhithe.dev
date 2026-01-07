import ProjectCard from '../components/ProjectCard';
import { projects } from '../data/projects';

const Projects = () => {
    return (
        <div className="space-y-8">
            <div className="flex flex-col gap-2">
                <h1 className="text-4xl font-bold text-white">Projects</h1>
                <p className="text-gray-400 font-mono text-sm max-w-2xl">
                    A collection of systems, graphics, and aerospace engineering projects.
                    Internships are won on documentation.
                </p>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                {projects.map((project) => (
                    <ProjectCard key={project.id} {...project} />
                ))}
            </div>
        </div>
    );
};

export default Projects;
