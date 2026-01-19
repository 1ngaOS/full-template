# Full Template Monorepo

A comprehensive monorepo template following project rules with multiple applications, shared services, and proper structure.

## 📁 Project Structure

```
.
├── apps/
│   └── sample/
│       ├── frontend/    # SvelteKit frontend application
│       ├── backend/     # Rust backend API server
│       └── docs/        # Docusaurus documentation
├── shared/
│   └── docs/           # Unified documentation for shared services
├── pnpm-workspace.yaml  # pnpm workspace configuration
├── package.json         # Root package configuration
├── turbo.json           # Turborepo configuration
└── tsconfig.json        # Root TypeScript configuration
```

## 🚀 Getting Started

### Prerequisites

- Node.js >= 18.0.0
- pnpm >= 8.0.0
- Rust (for backend development)

### Installation

```bash
# Install all dependencies
pnpm install
```

### Development

```bash
# Run frontend
pnpm sample:fr dev

# Run backend
pnpm sample:be dev

# Run app documentation
pnpm sample:docs start

# Run shared documentation
pnpm --filter shared-docs start
```

### Building

```bash
# Build all components
pnpm build

# Build specific component
pnpm sample:fr build
pnpm sample:be build
pnpm sample:docs build
```

## 📦 Applications

### Sample App

- **Frontend** (`apps/sample/frontend`): SvelteKit with SSR, Tailwind CSS v4
- **Backend** (`apps/sample/backend`): Rust API server with Axum
- **Documentation** (`apps/sample/docs`): Docusaurus documentation site

## 🛠️ Available Scripts

### Root Level

- `pnpm sample:fr <command>` - Run frontend commands
- `pnpm sample:be <command>` - Run backend commands
- `pnpm sample:docs <command>` - Run app documentation commands
- `pnpm build` - Build all packages and apps (via Turborepo)
- `pnpm dev` - Start all apps in development mode (via Turborepo)
- `pnpm lint` - Lint all packages and apps
- `pnpm test` - Run tests across all packages
- `pnpm format` - Format code with Prettier

## 🏗️ Architecture

This monorepo follows strict project rules:

- **pnpm workspaces** - Package manager and workspace configuration
- **Turborepo** - Build orchestration and caching
- **SvelteKit** - Frontend framework (SSR)
- **Rust** - Backend language
- **Docusaurus** - Documentation framework
- **Tailwind CSS v4** - Styling framework

## 📝 Development Guidelines

1. **App Structure**: All apps MUST be in `apps/{name}/{frontend,backend,mobile,docs}`
2. **Shared Services**: Global services go in `shared/` folder
3. **Documentation**: Every app MUST have a `docs` directory with Docusaurus
4. **Package Manager**: MUST use pnpm, not npm or yarn
5. **Frontend**: MUST use SvelteKit with SSR adapter
6. **Backend**: MUST use Rust or Python
7. **Testing**: MUST include Playwright for frontend, comprehensive tests for backend

## 🔧 Configuration

- **pnpm-workspace.yaml**: Workspace package discovery
- **Turborepo**: Build pipeline and caching
- **TypeScript**: Type safety across TypeScript projects
- **Prettier**: Code formatting
- **ESLint**: Linting configuration

## 📚 Learn More

- [Turborepo Documentation](https://turbo.build/repo/docs)
- [SvelteKit Documentation](https://kit.svelte.dev/docs)
- [pnpm Workspaces](https://pnpm.io/workspaces)
- [Docusaurus Documentation](https://docusaurus.io/docs)

## 🤝 Contributing

1. Follow the project structure rules
2. Use pnpm for all package management
3. Include tests for all new features
4. Update documentation as needed
5. Follow commit message standards

## 📄 License

This project is a template and can be used freely.

