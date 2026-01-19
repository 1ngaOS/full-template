# Architecture

## Overview

The Sample App follows a monorepo architecture with clear separation of concerns.

## Structure

```
apps/sample/
├── frontend/    # SvelteKit frontend application
├── backend/     # Rust backend API server
└── docs/        # Docusaurus documentation
```

## Frontend Architecture

- **Framework**: SvelteKit
- **Styling**: Tailwind CSS v4
- **Build**: Server-side rendering (SSR)
- **Testing**: Playwright

## Backend Architecture

- **Language**: Rust
- **Framework**: Axum
- **Features**: RESTful API, CORS support, health checks

