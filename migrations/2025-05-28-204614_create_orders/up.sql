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

INSERT INTO "public"."orders" ("id", "user_id", "total", "status", "address", "created_at") VALUES
(1, 1, 847.00, 'Доставлен', 'dfgdf', '2025-05-19 14:08:00.839598'),
(2, 4, 369.00, 'оформлен', 'Vietnam', '2025-05-19 15:55:17.27605'),
(3, 4, 1647.00, 'оформлен', 'Vietnam', '2025-05-19 16:03:14.506919');