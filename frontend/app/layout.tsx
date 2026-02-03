import type { Metadata } from 'next';
import './globals.css';
import Navbar from '@/components/Navbar';
import Footer from '@/components/Footer';

export const metadata: Metadata = {
    title: 'Samhith Enganti',
    description: 'Portfolio of a high-performance systems engineer.',
    icons: {
        icon: '/favicon.ico',
    },
};

export default function RootLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    return (
        <html lang="en">
            <body className="flex flex-col min-h-screen">
                <Navbar />
                <main className="flex-grow flex flex-col items-center justify-center p-6 md:p-12 w-full max-w-7xl mx-auto">
                    <div className="w-full">
                        {children}
                    </div>
                </main>
                <Footer />
            </body>
        </html>
    );
}
