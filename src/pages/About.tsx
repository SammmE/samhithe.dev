const About = () => {
    return (
        <div className="max-w-3xl space-y-12">
            <div className="space-y-6">
                <h1 className="text-4xl font-bold text-white">About Me</h1>
                <p className="text-gray-300 leading-relaxed text-lg">
                    I am a high school senior at Novi High School with a 1510 SAT, intending to major in Computer Engineering.
                </p>
                <p className="text-gray-300 leading-relaxed text-lg">
                    While many developers stay in the application layer, I prefer the 'engine room.' My passion lies in Systems Engineering—understanding how software interacts with hardware at the atomic level. Whether it's writing bootloaders in Assembly or designing flight controllers for VTOL drones, I build systems where performance and reliability are non-negotiable.
                </p>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-12 pt-8">
                <section>
                    <h2 className="text-xl font-mono text-amber-400 mb-6 flex items-center gap-2">
                        <span className="w-2 h-2 bg-amber-400 hidden md:block"></span>
                        Tech Stack
                    </h2>
                    <ul className="space-y-2 font-mono text-sm text-gray-400">
                        <li><span className="text-white">Languages:</span> C++, Rust, Python, Assembly</li>
                        <li><span className="text-white">Tools:</span> CMake, Git, Docker, GDB</li>
                        <li><span className="text-white">Platforms:</span> Linux, STM32, ESP32, AWS</li>
                        <li><span className="text-white">Graphics:</span> OpenGL, Vulkan, WGPU</li>
                    </ul>
                </section>

                <section>
                    <h2 className="text-xl font-mono text-tech-blue mb-6 flex items-center gap-2">
                        <span className="w-2 h-2 bg-tech-blue hidden md:block"></span>
                        Interests
                    </h2>
                    <ul className="space-y-2 font-mono text-sm text-gray-400">
                        <li>✈️ General Aviation (PPL Student)</li>
                        <li>📡 Software Defined Radio</li>
                        <li>🎹 Modular Synthesis</li>
                        <li>⛰️ Hiking & Alpinism</li>
                    </ul>
                </section>
            </div>

            <div className="pt-8 border-t border-white/10">
                <div className="flex gap-4 items-center">
                    <button className="bg-white text-bg-dark font-bold font-mono px-6 py-3 rounded hover:bg-gray-200 transition-colors flex items-center gap-2">
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="7 10 12 15 17 10"></polyline><line x1="12" y1="15" x2="12" y2="3"></line></svg>
                        Download Resume
                    </button>

                    <a href="mailto:sam@example.com" className="font-mono text-gray-500 hover:text-white transition-colors">
                        sam@example.com
                    </a>
                </div>
            </div>
        </div>
    );
};

export default About;
