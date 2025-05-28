CREATE SEQUENCE IF NOT EXISTS order_items_id_seq;

CREATE TABLE "public"."order_items" (
    "id" int4 NOT NULL DEFAULT nextval('order_items_id_seq'::regclass),
    "order_id" int4 NOT NULL,
    "product_id" int4 NOT NULL,
    "quantity" int4 NOT NULL,
    "price" numeric(10,2) NOT NULL,
    PRIMARY KEY ("id")
);

ALTER TABLE "public"."order_items" ADD FOREIGN KEY ("order_id") REFERENCES "public"."orders"("id") ON DELETE CASCADE;
ALTER TABLE "public"."order_items" ADD FOREIGN KEY ("product_id") REFERENCES "public"."products"("id");

INSERT INTO "public"."order_items" ("id", "order_id", "product_id", "quantity", "price") VALUES
(1, 1, 1, 2, 369.00),
(2, 1, 6, 1, 109.00),
(3, 2, 4, 1, 369.00),
(4, 3, 2, 3, 549.00);