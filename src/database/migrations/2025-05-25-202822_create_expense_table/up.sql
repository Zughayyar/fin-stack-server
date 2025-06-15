CREATE TABLE expenses (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    item_name VARCHAR NOT NULL,
    amount NUMERIC NOT NULL,
    date DATE NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
