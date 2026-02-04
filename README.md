# Cryptos Real Estate Platform

A modern real estate management system built with Rust, Leptos, and PostgreSQL.

## 🚀 Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Docker Desktop](https://www.docker.com/products/docker-desktop)
- [cargo-leptos](https://github.com/leptos-rs/cargo-leptos) - Install with: `cargo install cargo-leptos`

### Setup

1. **Clone and navigate to project:**
   ```bash
   cd cryptos-site
   ```

2. **Start database with Docker:**
   ```bash
   ./setup_docker_db.sh
   ```
   This will:
   - Start PostgreSQL 15 container
   - Create database and tables
   - Insert sample data (admin user + 3 estates)
   - Start pgAdmin (optional web interface)

3. **Run the application:**
   ```bash
   cargo leptos watch
   ```

4. **Open in browser:**
   - Application: http://localhost:3000
   - Login page: http://localhost:3000/login
   - pgAdmin (optional): http://localhost:5050

### Default Credentials

**Admin Login:**
- Username: `admin`
- Password: `admin123`

**pgAdmin (Database UI):**
- Email: `admin@cryptos.com`
- Password: `admin123`

## ✨ Features

- ✅ **PostgreSQL Database** - Full data persistence across restarts
- ✅ **User Authentication** - Secure login with session management
- ✅ **Role-Based Access Control** - Admin and User permission levels
- ✅ **Real Estate Management** - CRUD operations for properties
- ✅ **User Management** - Admin can create, update, and delete users
- ✅ **Modern UI** - Responsive design with Tailwind CSS
- ✅ **Arabic Support** - Full RTL (right-to-left) language support
- ✅ **Session Persistence** - Sessions stored in PostgreSQL
- ✅ **Secure Passwords** - Bcrypt hashing via password-auth
- ✅ **Docker Development** - Containerized database environment

## 🏗️ Tech Stack

### Frontend
- **Leptos 0.8** - Rust web framework with islands architecture
- **Tailwind CSS** - Utility-first styling
- **WebAssembly** - Client-side reactivity

### Backend
- **Axum 0.8** - High-performance web server
- **SQLx** - Compile-time checked SQL queries
- **tower-sessions** - PostgreSQL-backed session management
- **password-auth** - Secure password hashing (Argon2/bcrypt)

### Database
- **PostgreSQL 15** - Relational database
- **Docker Compose** - Container orchestration

## 📁 Project Structure

```
cryptos-site/
├── src/
│   ├── app/                    # Application components
│   │   ├── dashboard/          # Dashboard pages
│   │   │   ├── manage_user/    # User management
│   │   │   └── manage_estates/ # Estate management
│   │   ├── login.rs            # Login page
│   │   └── navbar.rs           # Navigation
│   ├── auth/                   # Authentication & authorization
│   │   └── mod.rs              # Auth functions (require_auth, require_admin)
│   ├── db/                     # Database layer
│   │   ├── users.rs            # User CRUD operations
│   │   ├── estates.rs          # Estate CRUD operations
│   │   └── mod.rs              # Database connection pool
│   ├── models.rs               # Data models (User, Estate, Level)
│   ├── lib.rs                  # Library root
│   └── main.rs                 # Application entry point
├── migrations/                 # SQL migrations
│   └── 20240101_initial.sql    # Database schema
├── sql/                        # SQL scripts
│   └── seed_data.sql           # Sample data
├── public/                     # Static assets
├── docker-compose.yml          # Docker services configuration
├── setup_docker_db.sh          # Database setup automation
├── Cargo.toml                  # Rust dependencies
├── AUTH_INTEGRATION.md         # Authentication guide
├── AUTH_STATUS.md              # Auth implementation status
├── DOCKER_SETUP.md            # Docker documentation
└── SECURITY.md                # Security implementation
```

## 🔐 Authentication System

The application uses a **fully integrated PostgreSQL-backed authentication system**:

### Session Management
- Sessions stored in PostgreSQL (not in-memory)
- 1-hour inactivity timeout
- Persist across server restarts
- Automatic cleanup on logout

### Authorization Levels
- **Admin**: Full access to all features
- **User**: Limited access (view-only)

### Protected Routes
All dashboard routes require authentication:
- `/dashboard/:id` - Main dashboard
- `/dashboard/manageUser/:id` - User management (Admin)
- `/dashboard/addUser/:id` - Add user (Admin)
- `/dashboard/updateUser/:targetId/:userId` - Update user (Admin)
- `/dashboard/manageEstates/:id` - Estate management
- `/dashboard/addEstate/:id` - Add estate
- `/dashboard/updateEstate/:targetId/:userId` - Update estate
- `/dashboard/estateDetails/:targetId/:userId` - Estate details

### Security Features
- ✅ Password hashing with bcrypt
- ✅ Secure session storage
- ✅ SQL injection prevention (parameterized queries)
- ✅ CSRF protection (built into Leptos)
- ✅ Admin-only operations protected
- ✅ Automatic login redirect

**For detailed authentication documentation, see [AUTH_STATUS.md](AUTH_STATUS.md)**

## 📊 Database Schema

### Users Table
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE,
    password TEXT NOT NULL,              -- bcrypt hash
    level VARCHAR(50) NOT NULL,          -- 'Admin' or 'User'
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

### Estates Table
```sql
CREATE TABLE estates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    address TEXT NOT NULL,
    image_url TEXT NOT NULL,
    price_in_cents BIGINT NOT NULL,
    space_in_meters INTEGER NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);
```

### Sessions Table
Automatically created by `tower-sessions-sqlx-store`:
- Stores session ID, data, and expiry
- Used for user authentication persistence

## 🐳 Docker Commands

### Basic Operations
```bash
# Start all services
docker compose up -d

# Stop services
docker compose down

# View logs
docker compose logs -f postgres

# Restart services
docker compose restart
```

### Database Access
```bash
# Access PostgreSQL shell
docker compose exec postgres psql -U cryptos_user -d cryptos_db

# Run SQL file
docker compose exec -T postgres psql -U cryptos_user -d cryptos_db < sql/seed_data.sql

# Export database
docker compose exec postgres pg_dump -U cryptos_user cryptos_db > backup.sql
```

### Maintenance
```bash
# Reset database (delete all data)
docker compose down -v
./setup_docker_db.sh

# View container status
docker compose ps

# View resource usage
docker compose stats
```

## 🔧 Development

### Environment Setup

Create a `.env` file in the project root:
```env
DATABASE_URL=postgres://cryptos_user:cryptos_password@localhost:5432/cryptos_db
```

### Running Migrations
```bash
# Run all pending migrations
sqlx migrate run

# Create new migration
sqlx migrate add <migration_name>
```

### Development Server
```bash
# Start with hot reload
cargo leptos watch

# Build for production
cargo leptos build --release

# Run tests
cargo test

# Check for errors without building
cargo check
```

### Database Operations
```bash
# Connect to database
psql -h localhost -p 5432 -U cryptos_user -d cryptos_db

# View all users
SELECT id, name, level FROM users;

# View all estates
SELECT id, name, address, price_in_cents FROM estates;

# Check active sessions
SELECT * FROM tower_sessions;
```

## 🛠️ Troubleshooting

### Database Won't Start

**Issue**: Port 5432 already in use

**Solution 1** - Stop local PostgreSQL:
```bash
# macOS
brew services stop postgresql

# Linux
sudo systemctl stop postgresql

# Windows
net stop postgresql-x64-15
```

**Solution 2** - Change Docker port in `docker-compose.yml`:
```yaml
services:
  postgres:
    ports:
      - "5433:5432"  # Use different external port
```
Then update `.env`:
```env
DATABASE_URL=postgres://cryptos_user:cryptos_password@localhost:5433/cryptos_db
```

### Build Errors

**Issue**: `cargo leptos build` fails

**Solutions**:
```bash
# Clean and rebuild
cargo clean
cargo leptos build

# Update dependencies
cargo update

# Ensure cargo-leptos is installed
cargo install cargo-leptos
```

### Login Not Working

**Symptom**: Login always fails even with correct credentials

**Check**:
```sql
-- Verify admin user exists
SELECT name, level FROM users WHERE name = 'admin';

-- If missing, insert admin user
INSERT INTO users (name, password, level) 
VALUES ('admin', '$argon2id$v=19$m=19456,t=2,p=1$...', 'Admin');
```

**Solution**: Run `./setup_docker_db.sh` to reset database with default admin user.

### Session Not Persisting

**Issue**: User logged out after server restart

**Check**:
1. Verify sessions are stored in PostgreSQL:
   ```sql
   SELECT COUNT(*) FROM tower_sessions;
   ```
2. Ensure database is running:
   ```bash
   docker compose ps postgres
   ```
3. Check `.env` file has correct `DATABASE_URL`

### Migration Panic: "relation already exists"

**Issue**: Application panics with `Failed to run migrations: ExecuteMigration(Database(PgDatabaseError... "relation already exists"`

**Cause**: Migration files were edited after indexes were already created in the database

**Solution**:
```bash
# Delete migration tracking to force re-run
psql postgres://cryptos_user:cryptos_password@localhost:5432/cryptos_db -c "DELETE FROM _sqlx_migrations;"

# Or drop the specific indexes causing conflicts
psql postgres://cryptos_user:cryptos_password@localhost:5432/cryptos_db << 'EOF'
DROP INDEX IF EXISTS idx_users_name;
DROP INDEX IF EXISTS idx_users_level;
DROP INDEX IF EXISTS idx_estates_name;
DROP INDEX IF EXISTS idx_estates_price;
DROP INDEX IF EXISTS idx_estates_space;
DROP INDEX IF EXISTS idx_sessions_expiry;
EOF

# Then restart the application
cargo leptos watch
```

### Migration Version Mismatch

**Issue**: `Failed to run migrations: VersionMismatch`

**Cause**: Migration file content changed after it was already run

**Solution**:
```bash
# Clear migration history and let it re-run
psql postgres://cryptos_user:cryptos_password@localhost:5432/cryptos_db -c "DELETE FROM _sqlx_migrations;"

# Restart application
cargo leptos watch
```

### Fresh Start

If you encounter persistent issues:
```bash
# Complete reset
docker compose down -v
rm -rf target/
cargo clean
./setup_docker_db.sh
cargo leptos watch
```

## 🚀 Production Deployment

### Recommended Services
- **Database**: AWS RDS, DigitalOcean Managed Database, Supabase
- **Hosting**: Fly.io, Railway, Render, AWS EC2
- **CDN**: Cloudflare (for static assets)

### Production Checklist

1. **Database**
   - [ ] Use managed PostgreSQL service
   - [ ] Enable automated backups
   - [ ] Set up connection pooling
   - [ ] Configure SSL connections

2. **Security**
   - [ ] Enable HTTPS (set `with_secure(true)` in session config)
   - [ ] Change default admin password
   - [ ] Use environment variables for secrets
   - [ ] Enable rate limiting on login endpoint
   - [ ] Set up firewall rules

3. **Performance**
   - [ ] Build with `--release` flag
   - [ ] Enable gzip compression
   - [ ] Set up CDN for static assets
   - [ ] Configure database indexes
   - [ ] Monitor query performance

4. **Environment Variables**
   ```env
   DATABASE_URL=postgresql://user:pass@host:5432/db?sslmode=require
   RUST_LOG=info
   LEPTOS_SITE_ADDR=0.0.0.0:8080
   ```

### Build for Production
```bash
# Build optimized release
cargo leptos build --release

# Output is in target/site/
# Deploy contents to your hosting service
```

## 📚 Documentation

- **[AUTH_STATUS.md](AUTH_STATUS.md)** - Complete authentication status and usage guide
- **[AUTH_INTEGRATION.md](AUTH_INTEGRATION.md)** - Detailed authentication integration guide
- **[DOCKER_SETUP.md](DOCKER_SETUP.md)** - Comprehensive Docker setup guide
- **[SECURITY.md](SECURITY.md)** - Security implementation details

## 🧪 Testing

### Manual Testing Checklist

1. **Authentication**
   - [ ] Login with admin/admin123
   - [ ] Verify redirect to dashboard
   - [ ] Test logout
   - [ ] Verify redirect to login after logout

2. **User Management**
   - [ ] Create new user
   - [ ] Update user name
   - [ ] Update user password
   - [ ] Change user level
   - [ ] Delete user

3. **Estate Management**
   - [ ] View all estates
   - [ ] Create new estate
   - [ ] Update estate details
   - [ ] View estate details
   - [ ] Delete estate

4. **Session Persistence**
   - [ ] Login and restart server
   - [ ] Verify still logged in
   - [ ] Check session timeout (1 hour)

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## 📝 License

This project is licensed under [Your License Here].

## 📧 Contact

For questions or support, contact [Your Contact Information].

---

**Built with ❤️ using Rust, Leptos, and PostgreSQL**

---

## Quick Reference

### Default Ports
- Application: `3000`
- PostgreSQL: `5432`
- pgAdmin: `5050`

### Default Credentials
- **App Admin**: admin / admin123
- **Database**: cryptos_user / cryptos_password
- **pgAdmin**: admin@cryptos.com / admin123

### Useful Commands
```bash
# Start everything
./setup_docker_db.sh && cargo leptos watch

# Reset database
docker compose down -v && ./setup_docker_db.sh

# View logs
docker compose logs -f

# Database shell
docker compose exec postgres psql -U cryptos_user -d cryptos_db
```
