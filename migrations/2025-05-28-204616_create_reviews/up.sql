CREATE SEQUENCE IF NOT EXISTS reviews_id_seq;

CREATE TABLE "public"."reviews" (
    "id" int4 NOT NULL DEFAULT nextval('reviews_id_seq'::regclass),
    "user_id" int4 NOT NULL,
    "product_id" int4 NOT NULL,
    "rating" int4 NOT NULL CHECK ((rating >= 1) AND (rating <= 5)),
    "comment" text,
    "created_at" timestamp NOT NULL DEFAULT now(),
    PRIMARY KEY ("id")
);

ALTER TABLE "public"."reviews" ADD FOREIGN KEY ("user_id") REFERENCES "public"."users"("id") ON DELETE SET NULL;
ALTER TABLE "public"."reviews" ADD FOREIGN KEY ("product_id") REFERENCES "public"."products"("id") ON DELETE CASCADE;

INSERT INTO "public"."reviews" ("id", "user_id", "product_id", "rating", "comment", "created_at") VALUES
(1, 1, 1, 5, 'Отлично', '2025-05-19 14:21:09.96767'),
(2, 1, 6, 5, 'Отлично', '2025-05-19 14:21:16.86685'),
(3, 1, 1, 5, 'Отлично', '2025-05-19 14:21:34.827217');