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