CREATE TABLE IF NOT EXISTS workspaces(
	id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    owner_user_id INT NOT NULL,
    status VARCHAR(20) CHECK (status IN ('Inactive', 'Active', 'Expired', 'Provisioning'))
);