const Logbook = () => {
    const posts = [
        {
            title: "Why my radar data was noisy and how a Kalman filter fixed it",
            date: "2025-12-14",
            tags: ["Algorithms", "Signal Processing"],
            preview: "I spent three weeks debugging a sensor that seemed to output random noise, only to realize I was ignoring the measurement covariance."
        },
        {
            title: "Memory alignment issues in my custom allocator",
            date: "2025-11-02",
            tags: ["C++", "Systems"],
            preview: "I kept getting segfaults when accessing SIMD registers. Turns out `malloc` doesn't always 16-byte align."
        },
        {
            title: "Understanding ownership in Rust through linked lists",
            date: "2025-10-20",
            tags: ["Rust", "Learning"],
            preview: "Writing a doubly linked list in Rust is the hardest thing I've ever done. Here is why `Rc<RefCell<T>>` is both a blessing and a curse."
        }
    ];

    return (
        <div className="space-y-12">
            <div className="space-y-2">
                <h1 className="text-4xl font-bold text-white">Logbook</h1>
                <p className="text-gray-400 font-mono text-sm">
                    A dump of things I've learned. Expect code snippets and rants about segfaults.
                </p>
            </div>

            <div className="space-y-8">
                {posts.map((post, index) => (
                    <article key={index} className="group border-b border-white/5 pb-8 last:border-0">
                        <div className="flex flex-col md:flex-row md:items-baseline justify-between mb-2">
                            <h2 className="text-xl font-bold text-text-offwhite group-hover:text-amber-400 transition-colors cursor-pointer">
                                {post.title}
                            </h2>
                            <span className="font-mono text-xs text-gray-600 shrink-0">{post.date}</span>
                        </div>
                        <p className="text-gray-400 mb-3 leading-relaxed text-sm max-w-2xl">
                            {post.preview}
                        </p>
                        <div className="flex gap-2">
                            {post.tags.map(tag => (
                                <span key={tag} className="text-[10px] font-mono text-terminal-green bg-terminal-green/5 px-2 py-0.5 rounded">
                                    #{tag}
                                </span>
                            ))}
                            <span className="text-[10px] font-mono text-gray-600 hover:text-white cursor-pointer ml-auto">
                                Read more --&gt;
                            </span>
                        </div>
                    </article>
                ))}
            </div>
        </div>
    );
};

export default Logbook;
