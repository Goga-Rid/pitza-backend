CREATE SEQUENCE IF NOT EXISTS complaints_id_seq;

CREATE TABLE "public"."complaints" (
    "id" int4 NOT NULL DEFAULT nextval('complaints_id_seq'::regclass),
    "order_id" int4 NOT NULL,
    "user_id" int4 NOT NULL,
    "reason" text NOT NULL,
    "comment" text,
    "created_at" timestamp NOT NULL DEFAULT now(),
    PRIMARY KEY ("id")
);

ALTER TABLE "public"."complaints" ADD FOREIGN KEY ("order_id") REFERENCES "public"."orders"("id") ON DELETE CASCADE;
ALTER TABLE "public"."complaints" ADD FOREIGN KEY ("user_id") REFERENCES "public"."users"("id") ON DELETE SET NULL;