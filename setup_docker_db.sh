#!/bin/bash

echo "🐳 Setting up PostgreSQL with Docker for Cryptos Real Estate"
echo "=============================================================="
echo ""

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed!"
    echo "Please install Docker Desktop from: https://www.docker.com/products/docker-desktop"
    exit 1
fi

# Check if Docker Compose is available
if ! docker compose version &> /dev/null; then
    echo "❌ Docker Compose is not available!"
    echo "Please install Docker Compose or update Docker Desktop"
    exit 1
fi

echo "✅ Docker found"
echo ""

# Stop and remove existing containers if any
echo "🧹 Cleaning up old containers..."
docker compose down -v 2>/dev/null || true

echo ""
echo "📝 Creating .env file..."
cat > .env << 'ENVEOF'
# Database Configuration
DATABASE_URL=postgres://cryptos_user:cryptos_password@localhost:5432/cryptos_db

# Leptos Configuration
LEPTOS_OUTPUT_NAME="cryptos-site"
LEPTOS_SITE_ROOT="target/site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
ENVEOF

echo "✅ .env file created"
echo ""

# Start PostgreSQL with Docker Compose
echo "🚀 Starting PostgreSQL container..."
docker compose up -d postgres

echo ""
echo "⏳ Waiting for PostgreSQL to be ready..."
sleep 5

# Wait for PostgreSQL to be healthy
echo "🔍 Checking PostgreSQL health..."
for i in {1..30}; do
    if docker compose exec -T postgres pg_isready -U cryptos_user -d cryptos_db &> /dev/null; then
        echo "✅ PostgreSQL is ready!"
        break
    fi
    if [ $i -eq 30 ]; then
        echo "❌ PostgreSQL failed to start"
        docker compose logs postgres
        exit 1
    fi
    echo -n "."
    sleep 1
done

echo ""
echo "📦 Installing sqlx-cli (if not already installed)..."
if ! command -v sqlx &> /dev/null; then
    cargo install sqlx-cli --no-default-features --features postgres
else
    echo "✅ sqlx-cli already installed"
fi

echo ""
echo "🔄 Running database migrations..."
sqlx migrate run

echo ""
echo "✅ Database setup complete!"
echo ""

# Verify setup
echo "📊 Verifying database..."
docker compose exec -T postgres psql -U cryptos_user -d cryptos_db -c "\dt" -c "SELECT COUNT(*) as user_count FROM users;" -c "SELECT COUNT(*) as estate_count FROM estates;"

echo ""
echo "=========================================================="
echo "✨ Docker Setup Complete!"
echo "=========================================================="
echo ""
echo "📋 Services Running:"
echo "   🐘 PostgreSQL: localhost:5432"
echo "   🔧 pgAdmin:    http://localhost:5050 (optional)"
echo ""
echo "🔐 Database Credentials:"
echo "   Database: cryptos_db"
echo "   User:     cryptos_user"
echo "   Password: cryptos_password"
echo ""
echo "🔐 Default Login (for app):"
echo "   Username: admin"
echo "   Password: admin123"
echo ""
echo "🔧 pgAdmin Login (if started):"
echo "   Email:    admin@cryptos.com"
echo "   Password: admin123"
echo ""
echo "📝 Useful Commands:"
echo "   Start database:     docker compose up -d"
echo "   Stop database:      docker compose down"
echo "   View logs:          docker compose logs postgres"
echo "   Reset database:     docker compose down -v && ./setup_docker_db.sh"
echo "   Access psql:        docker compose exec postgres psql -U cryptos_user -d cryptos_db"
echo ""
echo "🚀 Start your app:     cargo leptos watch"
echo ""
