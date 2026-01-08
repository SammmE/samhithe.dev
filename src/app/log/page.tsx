import Link from 'next/link';
import { posts } from '#site/content';

export default function Logbook() {

    return (
        <div className="space-y-12">
            <div className="space-y-2">
                <h1 className="text-4xl font-bold text-white">Logbook</h1>
                <p className="text-gray-400 font-mono text-sm">
                    A dump of things I've learned. Expect code snippets and rants about segfaults.
                </p>
            </div>

            <div className="space-y-8">
                {posts.map((post) => {
                    const slug = post.slug.split('/').pop();
                    return (
                    <article key={slug} className="group border-b border-white/5 pb-8 last:border-0">
                        <div className="flex flex-col md:flex-row md:items-baseline justify-between mb-2">
                            <Link href={`/log/${slug}`} className="text-xl font-bold text-text-offwhite group-hover:text-amber-400 transition-colors cursor-pointer">
                                {post.title}
                            </Link>
                            <span className="font-mono text-xs text-gray-600 shrink-0">{post.date}</span>
                        </div>
                        <p className="text-gray-400 mb-3 leading-relaxed text-sm max-w-2xl">
                            {post.preview}
                        </p>
                        <div className="flex gap-2">
                            {post.tags.map((tag) => (
                                <span key={tag} className="text-[10px] font-mono text-terminal-green bg-terminal-green/5 px-2 py-0.5 rounded">
                                    #{tag}
                                </span>
                            ))}
                            <Link href={`/log/${slug}`} className="text-[10px] font-mono text-gray-600 hover:text-white cursor-pointer ml-auto">
                                Read more --&gt;
                            </Link>
                        </div>
                    </article>
                )})}
            </div>
        </div>
    );
}
