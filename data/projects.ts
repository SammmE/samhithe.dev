export interface Project {
    id: string;
    title: string;
    problem: string;
    stack: string[];
    hurdle: string;
    tags: string[];
    link: string;
    featured: boolean;
}

export const projects: Project[] = [
    {
        id: "neurokernel",
        title: "NeuroKernel",
        problem: "Bare-metal AI without an Operating System.",
        stack: ["x86 Assembly", "C", "QEMU", "Makefile"],
        hurdle: "Figuring out how to load weights and images without making a disk.",
        tags: ["Systems", "AI", "Kernel"],
        link: "#",
        featured: true
    },
    {
        id: "autonomous-vtol-uav",
        title: "Autonomous VTOL UAV",
        problem: "Custom Flight Controller & Vision Integration.",
        stack: ["Python", "C++", "ROS", "CV"],
        hurdle: "Testing the flight controller logic safely without risking hardware damage.",
        tags: ["Aviation", "Robotics", "Embedded"],
        link: "#",
        featured: true
    },
    {
        id: "ferros",
        title: "FerrOS",
        problem: "UNIX-like Kernel written in Rust.",
        stack: ["Rust", "Bare-Metal"],
        hurdle: "Writing a robust task scheduler to manage process execution.",
        tags: ["OS", "Rust", "Safety"],
        link: "#",
        featured: false
    }
];
