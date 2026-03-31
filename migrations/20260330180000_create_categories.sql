-- Create categories table first
CREATE TABLE categories (
    id VARCHAR(100) PRIMARY KEY,
    label VARCHAR(255) NOT NULL,
    display_order INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Insert default categories
INSERT INTO categories (id, label, display_order) VALUES
    ('editor', 'Editor', 1),
    ('terminal', 'Terminal', 2);
