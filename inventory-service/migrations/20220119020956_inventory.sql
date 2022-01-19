-- Inventory Schema Model

CREATE SCHEMA IF NOT EXISTS inventory 
    CREATE TABLE products(
        id SERIAL NOT NULL PRIMARY KEY,
        title TEXT NOT NULL,
        code TEXT NOT NULL UNIQUE,
        note TEXT,
        pson JSON 
        
    )
    CREATE TABLE orders(
        id SERIAL NOT NULL,
        
    )
    