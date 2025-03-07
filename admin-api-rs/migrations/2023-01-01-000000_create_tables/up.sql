-- Create tenants table
CREATE TABLE tenants (
    id TEXT PRIMARY KEY NOT NULL,
    display_name TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

-- Create activities table
CREATE TABLE activities (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    activity_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    message TEXT,
    params TEXT, -- JSON
    result TEXT,
    filter_ukey1 TEXT UNIQUE,
    filter_key1 TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

-- Create index for activities
CREATE INDEX idx_activities_filter_key1 ON activities (filter_key1);

-- Create services table
CREATE TABLE services (
    id TEXT PRIMARY KEY NOT NULL,
    display_name TEXT NOT NULL,
    service_url TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

-- Create DAG variants table
CREATE TABLE dag_variants (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    dag TEXT NOT NULL, -- JSON
    service_id TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP,
    FOREIGN KEY (service_id) REFERENCES services (id) ON DELETE CASCADE,
    UNIQUE (name, service_id)
);