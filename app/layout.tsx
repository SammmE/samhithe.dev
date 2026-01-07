import type { Metadata } from 'next';
import './globals.css';
import Navbar from '@/components/Navbar';
import Footer from '@/components/Footer';

export const metadata: Metadata = {
    title: 'samhithe.dev',
    description: 'Building high-performance systems at the intersection of low-level computing and aviation.',
};

export default function RootLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    return (
        <html lang="en">
            <body>
                <div className="min-h-screen flex flex-col">
                    <Navbar />
                    <main className="flex-grow px-6 md:px-12 max-w-7xl mx-auto w-full py-8 md:py-16">
                        {children}
                    </main>
                    <Footer />
                </div>
            </body>
        </html>
    );
}
