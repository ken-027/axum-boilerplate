# Axum Boilerplate

Production-ready Rust web application boilerplate using Axum framework with SQLx ORM and PostgreSQL.

## Features

- **Web Framework**: Axum with async/await support
- **Database**: PostgreSQL with SQLx ORM
- **Authentication**: JWT-based authentication with bcrypt password hashing
- **Architecture**: Repository pattern for database operations
- **Error Handling**: Comprehensive error handling with custom error types
- **Validation**: Request validation using validator crate
- **Logging**: Structured logging with tracing
- **Configuration**: Environment-based configuration management
- **Containerization**: Docker and Docker Compose setup
- **Database Migrations**: SQLx migrations for schema management
- **Security**: CORS, password hashing, JWT tokens

## Quick Start

### Prerequisites

- Rust (latest stable version)
- PostgreSQL
- Docker & Docker Compose (optional)

### Setup

1. Clone the repository:
```bash
git clone <repository-url>
cd axum-boilerplate
```

2. Copy environment file:
```bash
cp .env.example .env
```

3. Update `.env` with your database credentials and JWT secret.

### Running with Docker Compose

```bash
docker-compose up --build
```

### Running Locally

1. **Install PostgreSQL** (if not already installed)

2. **Set up the database:**

   **Option A: Using the setup script (Linux/macOS/WSL):**
   ```bash
   chmod +x scripts/setup_db.sh
   ./scripts/setup_db.sh
   ```

   **Option B: Manual setup:**
   ```bash
   # Connect to PostgreSQL as superuser
   psql -U postgres

   # Run the setup commands
   \i scripts/setup_db.sql
   ```

   **Option C: Using Docker:**
   ```bash
   docker-compose up postgres -d
   ```

3. **Copy environment file:**
   ```bash
   cp .env.example .env
   # Edit .env with your database credentials if different
   ```

4. **Run migrations:**
   ```bash
   sqlx migrate run
   ```

5. **Start the application:**
   ```bash
   cargo run
   ```

### Database Setup Details

The application expects a PostgreSQL database with:
- **Database:** `axum_db`
- **User:** `axum_user`
- **Password:** `axum_password`
- **Host:** `localhost:5432`

If you need different credentials, update the `DATABASE_URL` in your `.env` file.

## API Endpoints

### Health Check
- `GET /health` - Health check endpoint

### Authentication
- `POST /auth/register` - User registration
- `POST /auth/login` - User login

### Users
- `GET /users` - List all users (with pagination)
- `GET /users/me` - Get current user (requires auth)
- `GET /users/:id` - Get user by ID
- `DELETE /users/:id` - Delete user (requires auth, own account only)

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `PORT` | Server port | 3000 |
| `DATABASE_URL` | PostgreSQL connection string | - |
| `DATABASE_MAX_CONNECTIONS` | Max database connections | 10 |
| `JWT_SECRET` | JWT signing secret | - |
| `JWT_EXPIRATION_HOURS` | JWT token expiration | 24 |
| `RUST_LOG` | Log level | debug |

## Project Structure

```
src/
├── config/          # Configuration management
├── handlers/        # HTTP request handlers
├── middleware/      # Custom middleware
├── models/          # Data models and DTOs
├── repositories/    # Database repository pattern
├── utils/           # Utility functions (errors, JWT, validation)
└── main.rs          # Application entry point
migrations/          # Database migrations
```

## Development

### Adding New Features

1. Define models in `src/models/`
2. Create repository in `src/repositories/`
3. Implement handlers in `src/handlers/`
4. Add routes in respective handler files
5. Update main.rs to include new routes

### Database Migrations

Create new migration:
```bash
sqlx migrate add <migration_name>
```

Run migrations:
```bash
sqlx migrate run
```

## Production Considerations

- [ ] Use a secure JWT secret
- [ ] Configure proper CORS policies
- [ ] Set up proper logging and monitoring
- [ ] Use connection pooling for database
- [ ] Implement rate limiting
- [ ] Add input sanitization
- [ ] Set up SSL/TLS
- [ ] Configure proper error pages
- [ ] Add health checks and metrics
- [ ] Implement graceful shutdown

## License

MIT License