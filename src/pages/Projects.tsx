import ProjectCard from '../components/ProjectCard';

const Projects = () => {
    const projects = [
        {
            title: "NeuroKernel",
            problem: "Bare-metal AI without an Operating System.",
            stack: ["x86 Assembly", "C", "QEMU", "Makefile"],
            hurdle: "I wrote a custom OS kernel from scratch specifically optimized to run a Feed-Forward Neural Network, including writing low-level memory allocators and VGA drivers to visualize training on hardware.",
            tags: ["Systems", "AI", "Kernel"],
            link: "#"
        },
        {
            title: "Autonomous VTOL UAV",
            problem: "Custom Flight Controller & Vision Integration.",
            stack: ["Python", "C++", "ROS", "CV"],
            hurdle: "Designed and flight-tested a custom VTOL drone. Rather than using off-the-shelf software, I engineered the flight controller logic from scratch, tuning PID loops for stabilization and integrating a neural network to fuse telemetry with camera data.",
            tags: ["Aviation", "Robotics", "Embedded"],
            link: "#"
        },
        {
            title: "FerrOS",
            problem: "UNIX-like Kernel written in Rust.",
            stack: ["Rust", "Bare-Metal"],
            hurdle: "An exploration into memory safety at the hardware level. I implemented VGA drivers, Interrupt Descriptor Tables (IDT), and keyboard handling to prevent the undefined behavior common in C-based embedded systems.",
            tags: ["OS", "Rust", "Safety"],
            link: "#"
        },
        {
            title: "Distributed Raytracer",
            problem: "Rendering high-res scenes took too long on a single machine.",
            stack: ["Rust", "WebAssembly", "AWS"],
            hurdle: "Managing race conditions in the job distribution queue.",
            tags: ["Graphics", "Distributed Systems"],
            link: "#"
        }
    ];

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
                {projects.map((project, index) => (
                    <ProjectCard key={index} {...project} />
                ))}
            </div>
        </div>
    );
};

export default Projects;
