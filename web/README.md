# Feedtown Web

A modern React-based web application for the Feedtown platform.

## Tech Stack

- **Runtime**: Deno 🦕
- **Framework**: React 18 with TypeScript
- **Bundler**: Vite
- **Routing**: React Router DOM
- **Styling**: Tailwind CSS + DaisyUI
- **Build Tool**: Vite

## Development

Start the development server:
```bash
just web
# or
cd web && deno task dev
```

The application will be available at `http://localhost:5173`

## Building

Build for production:
```bash
just web-build
# or
cd web && deno task build
```

Preview the production build:
```bash
just web-preview
# or
cd web && deno task preview
```

## Features

- 🎨 Modern UI with Tailwind CSS and DaisyUI components
- 🌙 Dark/Light theme toggle
- 📱 Responsive design
- 🧭 Client-side routing with React Router
- 🔧 Type-safe development with TypeScript
- ⚡ Fast development and builds with Vite

## API Integration

The web application is designed to connect to the Feedtown API running on port 3000. Make sure to start the API server before using features that require backend connectivity.

```bash
just api
```