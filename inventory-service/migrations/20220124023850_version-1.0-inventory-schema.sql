-- Add migration script here
CREATE SCHEMA inventory (
    CREATE TABLE products (
        id SERIAL NOT NULL PRIMARY KEY
        product_name TEXT NOT NULL
        product_code TEXT NOT NULL
        incoming INTEGER NOT NULL
        outgoing INTEGER NOT NULL
        instock INTEGER NOT NULL
        product_description TEXT
        unit_price MONEY
        magma_info JSON
    )

    CREATE TABLE orders (
        id SERIAL NOT NULL PRIMARY KEY
        order_number INTEGER NOT NULL
        customer_id INTEGER NOT NULL
        payment_id INTEGER NOT NULL
        product_list JSON NOT NULL
        order_date TIMESTAMP NOT NULL
        magma_info JSON
    )

)