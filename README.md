# Gruvbox Ports

A clean, minimal web application showcasing gruvbox theme ports across various applications.

## Prerequisites

- PostgreSQL database
- Environment variables in `.env` (copy from `.env.example`)

## Running

### Native
```bash
cargo run
```

### Docker
```bash
docker build -t gruvbox_ports .
docker run -p 3000:3000 --env-file .env gruvbox_ports
```

## Configuration

Required environment variables:
- `APP_HOST` - Bind address (e.g., `0.0.0.0`)
- `APP_PORT` - Server port (e.g., `3000`)
- `APP_ORIGIN` - Public URL for CORS/cookies (e.g., `https://gruvbox.example.com`)
- `DATABASE_URL` - PostgreSQL connection string
- `DATABASE_RUN_MIGRATIONS` - Auto-migrate on startup (optional, default: `false`)
