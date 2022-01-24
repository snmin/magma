-- Add migration script here
CREATE SCHEMA inventory (
    CREATE TABLE products (
        id SERIAL NOT NULL
        stripe_id TEXT NOT NULL
        product_name TEXT NOT NULL
        product_code TEXT NOT NULL
        incoming INTEGER NOT NULL
        outgoing INTEGER NOT NULL
        instock INTEGER NOT NULL
        product_description TEXT
        price INTEGER NOT NULL
        magma_info JSON
        PRIMARY KEY (id)
        
    );

    CREATE TABLE orders (
        id SERIAL NOT NULL
        order_number INTEGER NOT NULL
        customer_id INTEGER NOT NULL
        payment_id INTEGER NOT NULL
        product_list JSON NOT NULL
        order_date TIMESTAMP NOT NULL
        magma_info JSON
        PRIMARY KEY (id)
        FOREIGN KEY (customer_id) REFERENCES humans.customers(id)
        FOREIGN KEY (payment_id) REFERENCES finance.payments(id)
    );

);

CREATE SCHEMA finance (
    CREATE TABLE payments (
        id SERIAL NOT NULL
        stripe_id TEXT
        PRIMARY KEY (id)
    );

);

CREATE SCHEMA humans (
    CREATE TABLE customers (
        id SERIAL NOT NULL
        stripe_id TEXT NOT NULL
        customer_name TEXT NOT NULL
        PRIMARY KEY (id)
    );
    CREATE TABLE employees (
        id SERIAL NOT NULL
        PRIMARY KEY (id)
    );
);
