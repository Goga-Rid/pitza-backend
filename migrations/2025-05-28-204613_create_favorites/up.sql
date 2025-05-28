CREATE SEQUENCE IF NOT EXISTS favorites_id_seq;

CREATE TABLE "public"."favorites" (
    "id" int4 NOT NULL DEFAULT nextval('favorites_id_seq'::regclass),
    "user_id" int4 NOT NULL,
    "product_id" int4 NOT NULL, 
    PRIMARY KEY ("id")
);

CREATE UNIQUE INDEX favorites_user_id_product_id_key ON public.favorites USING btree (user_id, product_id);

ALTER TABLE "public"."favorites" ADD FOREIGN KEY ("product_id") REFERENCES "public"."products"("id") ON DELETE CASCADE;
ALTER TABLE "public"."favorites" ADD FOREIGN KEY ("user_id") REFERENCES "public"."users"("id") ON DELETE CASCADE;

INSERT INTO "public"."favorites" ("id", "user_id", "product_id") VALUES
(6, 1, 8),
(9, 1, 1),
(10, 1, 4);