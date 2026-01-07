import Link from 'next/link';
import { notFound } from 'next/navigation';
import { posts } from '@/data/posts';

interface PageProps {
    params: Promise<{ id: string }>;
}

export default async function BlogPost({ params }: PageProps) {
    const { id } = await params;
    const post = posts.find(p => p.id === id);

    if (!post) {
        notFound();
    }

    return (
        <article className="max-w-3xl mx-auto">
            <Link href="/log" className="text-xs font-mono text-gray-500 hover:text-white mb-8 block">&lt;-- Back to Logbook</Link>

            <header className="mb-10 border-b border-white/10 pb-6">
                <h1 className="text-3xl md:text-4xl font-bold text-white mb-4">{post.title}</h1>
                <div className="flex gap-4 items-center font-mono text-xs text-gray-500">
                    <span>{post.date}</span>
                    <div className="flex gap-2">
                        {post.tags.map(tag => (
                            <span key={tag} className="bg-white/5 px-2 py-0.5 rounded text-gray-300">#{tag}</span>
                        ))}
                    </div>
                </div>
            </header>

            <div
                className="prose prose-invert prose-code:text-terminal-green prose-pre:bg-black/50 prose-headings:font-bold prose-headings:text-white text-gray-300 max-w-none"
                dangerouslySetInnerHTML={{ __html: post.content }}
            />
        </article>
    );
}

export function generateStaticParams() {
    return posts.map((post) => ({
        id: post.id,
    }));
}
