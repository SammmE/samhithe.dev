import React from 'react';
import Navbar from './Navbar';
import Footer from './Footer';

interface LayoutProps {
    children: React.ReactNode;
}

const Layout = ({ children }: LayoutProps) => {
    return (
        <div className="min-h-screen flex flex-col">
            <Navbar />
            <main className="flex-grow px-6 md:px-12 max-w-7xl mx-auto w-full py-8 md:py-16">
                {children}
            </main>
            <Footer />
        </div>
    );
};

export default Layout;
