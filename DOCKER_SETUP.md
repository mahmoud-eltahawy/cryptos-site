# Docker Setup for Cryptos Real Estate

## Why Docker?

✅ **No PostgreSQL installation needed** - Everything runs in containers
✅ **Consistent environment** - Same setup on any machine
✅ **Easy cleanup** - Remove everything with one command
✅ **Isolated** - Doesn't affect your system
✅ **Fast setup** - One script to get everything running

## Prerequisites

1. **Docker Desktop** (includes Docker Compose)
   - **macOS**: https://docs.docker.com/desktop/install/mac-install/
   - **Windows**: https://docs.docker.com/desktop/install/windows-install/
   - **Linux**: https://docs.docker.com/desktop/install/linux-install/

2. **Rust & Cargo** (you already have this)

## Quick Start

### 1. Setup Database

```bash
./setup_docker_db.sh
```

This will:
- Start PostgreSQL in a Docker container
- Create the database and user
- Run all migrations
- Add sample data (1 admin user + 3 estates)
- Verify everything works

### 2. Start Your App

```bash
cargo leptos watch
```

Your app will be available at http://localhost:3000

## Services

### PostgreSQL Database
- **Port**: 5432
- **Database**: `cryptos_db`
- **User**: `cryptos_user`
- **Password**: `cryptos_password`
- **Connection**: `postgres://cryptos_user:cryptos_password@localhost:5432/cryptos_db`

### pgAdmin (Optional Database UI)
- **URL**: http://localhost:5050
- **Email**: admin@cryptos.com
- **Password**: admin123

To start pgAdmin:
```bash
docker compose up -d pgadmin
```

## Common Commands

### Start Database
```bash
docker compose up -d postgres
```

### Stop Database
```bash
docker compose stop
```

### Stop and Remove Database (keeps data)
```bash
docker compose down
```

### Stop and Delete Everything (including data)
```bash
docker compose down -v
```

### View Logs
```bash
docker compose logs postgres
docker compose logs -f postgres  # Follow logs
```

### Access PostgreSQL CLI
```bash
docker compose exec postgres psql -U cryptos_user -d cryptos_db
```

### Check Database Status
```bash
docker compose ps
```

### Restart Database
```bash
docker compose restart postgres
```

### Reset Database (Fresh Start)
```bash
docker compose down -v
./setup_docker_db.sh
```

## Useful SQL Commands

Once in the PostgreSQL CLI (`docker compose exec postgres psql -U cryptos_user -d cryptos_db`):

```sql
-- List all tables
\dt

-- Show users
SELECT * FROM users;

-- Show estates
SELECT * FROM estates;

-- Count records
SELECT COUNT(*) FROM users;
SELECT COUNT(*) FROM estates;

-- Exit
\q
```

## Troubleshooting

### Port 5432 Already in Use

If you have PostgreSQL installed locally:

**Option 1**: Stop local PostgreSQL
```bash
# macOS
brew services stop postgresql

# Linux
sudo systemctl stop postgresql

# Windows
# Stop PostgreSQL service from Services app
```

**Option 2**: Change Docker port in `docker-compose.yml`
```yaml
ports:
  - "5433:5432"  # Use 5433 instead
```

Then update `DATABASE_URL` in `.env`:
```
DATABASE_URL=postgres://cryptos_user:cryptos_password@localhost:5433/cryptos_db
```

### Container Won't Start

Check Docker is running:
```bash
docker ps
```

View logs:
```bash
docker compose logs postgres
```

### Database Connection Errors

1. Check container is running:
```bash
docker compose ps
```

2. Wait for database to be ready:
```bash
docker compose exec postgres pg_isready -U cryptos_user -d cryptos_db
```

3. Verify `.env` file has correct `DATABASE_URL`

### Migrations Failed

Re-run migrations:
```bash
sqlx migrate run
```

Reset and try again:
```bash
docker compose down -v
./setup_docker_db.sh
```

## File Structure

```
cryptos-site/
├── docker-compose.yml      # Docker services configuration
├── setup_docker_db.sh      # Setup script
├── .dockerignore           # Files to ignore in Docker
├── migrations/             # SQL migration files
│   ├── 20240101000001_create_users_table.sql
│   ├── 20240101000002_create_estates_table.sql
│   └── 20240101000003_create_sessions_table.sql
└── .env                    # Environment variables (auto-created)
```

## Data Persistence

Data is stored in Docker volumes and persists between restarts:
- `postgres_data` - Database files
- `pgadmin_data` - pgAdmin configuration

To completely remove data:
```bash
docker compose down -v
```

## Production Deployment

For production, use managed PostgreSQL services:
- **Heroku Postgres**
- **AWS RDS**
- **DigitalOcean Managed Databases**
- **Supabase**
- **Neon**

Just update `DATABASE_URL` in production environment.

## Benefits Over Local Install

| Feature | Docker | Local Install |
|---------|--------|---------------|
| Setup Time | 1 minute | 10-30 minutes |
| System Impact | None | Installs services |
| Cleanup | One command | Manual uninstall |
| Version Control | Easy | Difficult |
| Team Setup | Identical | Varies |
| Multiple Projects | Isolated | Conflicts |

## Tips

1. **Keep containers running** while developing - they use minimal resources
2. **Use pgAdmin** for visual database management
3. **Backup important data** before running `docker compose down -v`
4. **Check logs** if something goes wrong
5. **Update images** periodically: `docker compose pull`

## Need Help?

- Docker logs: `docker compose logs postgres`
- Database health: `docker compose exec postgres pg_isready`
- System status: `docker compose ps`
- Full reset: `docker compose down -v && ./setup_docker_db.sh`

---

**Happy Coding!** 🚀
