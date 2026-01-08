/** @type {import('tailwindcss').Config} */
export default {
    content: [
        "./app/**/*.{js,ts,jsx,tsx,mdx}",
        "./components/**/*.{js,ts,jsx,tsx,mdx}",
    ],
    theme: {
        extend: {
            colors: {
                'bg-dark': '#121212',
                'text-offwhite': '#E0E0E0',
                'tech-blue': '#00ADB5',
                'alert-amber': '#FFC857',
                'terminal-green': '#39FF14',
                'card-bg': '#1E1E1E',
            },
            fontFamily: {
                mono: ['JetBrains Mono', 'Roboto Mono', 'monospace'],
                sans: ['Inter', 'Open Sans', 'sans-serif'],
            },
            backgroundImage: {
                'subtle-grid': "url('data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iNDAiIGhlaWdodD0iNDAiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+CjxwYXRoIGQ9Ik00MCAwTDTAIDQwIiBzdHJva2U9IndoaXRlIiBzdHJva2Utd2lkdGg9IjEiIGZpbGw9Im5vbmUiIG9wYWNpdHk9IjAuMDEiLz4KPC9zdmc+')",
            },
        },
    },
    plugins: [
        require('@tailwindcss/typography'),
    ],
}
