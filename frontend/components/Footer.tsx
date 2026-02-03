const Footer = () => {
    return (
        <footer className="w-full border-t border-white/10 mt-auto bg-bg-dark">
            <div className="max-w-7xl mx-auto px-6 md:px-12 py-4 flex flex-col md:flex-row justify-between items-center text-xs font-mono text-gray-500">
                <div>
                    &copy; 2026 Sam
                </div>
                <div className="flex items-center gap-2">
                    <span className="w-2 h-2 rounded-full bg-terminal-green animate-pulse"></span>
                    <span>git: 9070ed0</span>
                </div>
            </div>
        </footer>
    );
};

export default Footer;
