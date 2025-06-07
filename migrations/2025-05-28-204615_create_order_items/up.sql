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