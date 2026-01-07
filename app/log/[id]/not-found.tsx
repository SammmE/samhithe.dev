import Link from 'next/link';

export default function NotFound() {
    return (
        <div className="text-center py-20">
            <h2 className="text-2xl font-bold text-white mb-4">404: Post Not Found</h2>
            <Link href="/log" className="text-tech-blue hover:underline">Return to Logbook</Link>
        </div>
    );
}
