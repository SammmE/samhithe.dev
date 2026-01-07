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
            </div>
        </nav>
    );
};

export default Navbar;
