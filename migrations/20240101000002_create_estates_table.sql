-- Create estates table
CREATE TABLE IF NOT EXISTS estates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    address TEXT NOT NULL,
    image_url TEXT NOT NULL,
    price_in_cents BIGINT NOT NULL CHECK (price_in_cents >= 0),
    space_in_meters INTEGER NOT NULL CHECK (space_in_meters > 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create index on name for search
CREATE INDEX IF NOT EXISTS idx_estates_name ON estates(name);

-- Create index on price for filtering
CREATE INDEX IF NOT EXISTS idx_estates_price ON estates(price_in_cents);

-- Create index on space for filtering
CREATE INDEX IF NOT EXISTS idx_estates_space ON estates(space_in_meters);

-- Insert sample estates
INSERT INTO estates (id, name, address, image_url, price_in_cents, space_in_meters) VALUES
    ('10000000-0000-0000-0000-000000000001', 'فيلا فاخرة في التجمع الخامس', 'التجمع الخامس، القاهرة الجديدة', 'https://images.unsplash.com/photo-1613490493576-7fde63acd811?w=800', 500000000, 350),
    ('10000000-0000-0000-0000-000000000002', 'شقة مودرن في المعادي', 'المعادي، القاهرة', 'https://images.unsplash.com/photo-1545324418-cc1a3fa10c00?w=800', 150000000, 180),
    ('10000000-0000-0000-0000-000000000003', 'بنتهاوس في الزمالك', 'الزمالك، القاهرة', 'https://images.unsplash.com/photo-1512917774080-9991f1c4c750?w=800', 800000000, 400)
ON CONFLICT (id) DO NOTHING;
