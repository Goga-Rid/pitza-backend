CREATE SEQUENCE IF NOT EXISTS orders_id_seq;

CREATE TABLE "public"."orders" (
    "id" int4 NOT NULL DEFAULT nextval('orders_id_seq'::regclass),
    "user_id" int4 NOT NULL,
    "total" numeric(10,2) NOT NULL,
    "status" text NOT NULL DEFAULT 'оформлен'::text,
    "address" text NOT NULL,
    "created_at" timestamp NOT NULL DEFAULT now(),
    PRIMARY KEY ("id")
);

ALTER TABLE "public"."orders" ADD FOREIGN KEY ("user_id") REFERENCES "public"."users"("id") ON DELETE SET NULL;