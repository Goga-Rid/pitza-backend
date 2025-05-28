ALTER TABLE "public"."orders" DROP CONSTRAINT IF EXISTS orders_user_id_fkey;
DROP TABLE IF EXISTS "public"."orders";
DROP SEQUENCE IF EXISTS orders_id_seq;