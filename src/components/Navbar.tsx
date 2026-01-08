'use client';

import Link from 'next/link';
import { usePathname } from 'next/navigation';

const Navbar = () => {
    const pathname = usePathname();

    const isActive = (path: string) => pathname === path;

    const linkClass = (path: string) =>
        `transition-colors duration-200 hover:text-tech-blue ${isActive(path) ? 'text-tech-blue' : 'text-text-offwhite'
        }`;

    return (
        <nav className="flex items-center justify-between py-6 px-6 md:px-12 max-w-7xl mx-auto w-full">
            <Link href="/" className="text-xl font-mono font-bold tracking-tighter hover:text-tech-blue transition-colors">
                samhithe.dev
            </Link>
            <div className="flex gap-6 md:gap-8 font-mono text-sm">
                <Link href="/projects" className={linkClass('/projects')}>
                    /projects
                </Link>
                <Link href="/log" className={linkClass('/log')}>
                    /log
                </Link>
                <Link href="/about" className={linkClass('/about')}>
                    /about
                </Link>
                <Link
                    href="https://github.com/SammmE"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="text-text-offwhite hover:text-tech-blue transition-colors duration-200"
                    aria-label="GitHub Profile"
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width="20"
                        height="20"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        strokeWidth="2"
                        strokeLinecap="round"
                        strokeLinejoin="round"
                    >
                        <path d="M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"></path>
                    </svg>
                </Link>
            </div>
        </nav>
    );
};

export default Navbar;
