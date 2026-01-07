/** @type {import('tailwindcss').Config} */
export default {
    content: [
        "./index.html",
        "./src/**/*.{js,ts,jsx,tsx}",
    ],
    theme: {
        extend: {
            colors: {
                'bg-dark': '#121212',
                'text-offwhite': '#E0E0E0',
                'tech-blue': '#00d0ff', // Vivid, Electric Blue
                'alert-amber': '#ffab00', // Amber/Orange
                'terminal-green': '#00ff41', // Arcade/Terminal Green
                'card-bg': '#1E1E1E', // Dark Grey for cards
            },
            fontFamily: {
                mono: ['"JetBrains Mono"', '"Roboto Mono"', 'monospace'],
                sans: ['Inter', '"Open Sans"', 'sans-serif'],
            },
            backgroundImage: {
                'subtle-grid': "url(\"data:image/svg+xml,%3Csvg width='40' height='40' viewBox='0 0 40 40' xmlns='http://www.w3.org/2000/svg'%3E%3Cg fill='%23ffffff' fill-opacity='0.01' fill-rule='evenodd'%3E%3Cpath d='M0 40L40 0H20L0 20M40 40V20L20 40'/%3E%3C/g%3E%3C/svg%3E\")",
            },
            animation: {
                'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
            }
        },
    },
    plugins: [],
}
