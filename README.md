# samhithe.dev

Personal portfolio website built with Next.js, showcasing systems engineering projects and technical logs.

## Tech Stack

- **Framework**: Next.js 14 (App Router)
- **Styling**: Tailwind CSS
- **Content**: Velite (MDX content management)
- **Language**: TypeScript

## Getting Started

### Prerequisites

- Node.js 20+
- npm or pnpm

### Installation

```bash
npm install
```

### Development

```bash
npm run dev
```

Open [http://localhost:3000](http://localhost:3000) to view the site.

### Build

```bash
npm run build
```

### Production

```bash
npm run start
```

## Project Structure

```
├── app/                  # Next.js app router pages
│   ├── about/           # About page
│   ├── log/             # Blog/logbook pages
│   ├── projects/        # Projects pages
│   └── layout.tsx       # Root layout
├── components/          # React components
├── content/            # MDX content files
│   ├── posts/          # Blog posts
│   └── projects/       # Project documentation
├── public/             # Static assets
└── velite.config.ts    # Content configuration
```

## Content Management

Content is managed through MDX files in the `content/` directory. Velite compiles these at build time into type-safe data.

### Adding a Project

Create a new `.mdx` file in `content/projects/`:

```mdx
---
title: Project Name
problem: What problem does it solve?
stack: [Tech1, Tech2, Tech3]
hurdle: Biggest challenge
tags: [tag1, tag2]
link: https://example.com
github_link: https://github.com/username/repo
featured: true
---

# Project details...
```

### Adding a Blog Post

Create a new `.mdx` file in `content/posts/`:

```mdx
---
title: Post Title
date: YYYY-MM-DD
tags: [tag1, tag2]
preview: Brief description
---

# Post content...
```

## License

Private repository - All rights reserved
