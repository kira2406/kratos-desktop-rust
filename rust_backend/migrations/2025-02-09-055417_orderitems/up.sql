-- Your SQL goes here
CREATE TABLE orderitems (
    order_item_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID NOT NULL,
    game_id UUID NOT NULL,
    quantity INT4 NOT NULL,
    unit_price NUMERIC NOT NULL,
    discount NUMERIC,
    FOREIGN KEY (order_id) REFERENCES orders(order_id) ON DELETE CASCADE,
    FOREIGN KEY (game_id) REFERENCES games(game_id) ON DELETE CASCADE
);
