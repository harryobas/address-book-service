CREATE TABLE IF NOT EXISTS address_books (
    id SERIAL PRIMARY KEY,
    address_book_name VARCHAR(255) NOT NULL
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS contacts (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    address VARCHAR(255) NOT NULL,
    phone_number VARCHAR(20), 
    email VARCHAR(255),
    address_book_id INTEGER REFERENCES address_books(id) ON DELETE CASCADE
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);


