import { Link, useLocation } from 'react-router-dom';

const Navbar = () => {
    const location = useLocation();

    const isActive = (path: string) => location.pathname === path;

    const linkClass = (path: string) =>
        `transition-colors duration-200 hover:text-tech-blue ${isActive(path) ? 'text-tech-blue' : 'text-text-offwhite'
        }`;

    return (
        <nav className="flex items-center justify-between py-6 px-6 md:px-12 max-w-7xl mx-auto w-full">
            <Link to="/" className="text-xl font-mono font-bold tracking-tighter hover:text-tech-blue transition-colors">
                samhithe.dev
            </Link>
            <div className="flex gap-6 md:gap-8 font-mono text-sm">
                <Link to="/projects" className={linkClass('/projects')}>
                    /projects
                </Link>
                <Link to="/log" className={linkClass('/log')}>
                    /log
                </Link>
                <Link to="/about" className={linkClass('/about')}>
                    /about
                </Link>
            </div>
        </nav>
    );
};

export default Navbar;
