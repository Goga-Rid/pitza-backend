ALTER TABLE "public"."favorites" DROP CONSTRAINT IF EXISTS favorites_product_id_fkey;
ALTER TABLE "public"."favorites" DROP CONSTRAINT IF EXISTS favorites_user_id_fkey;
DROP INDEX IF EXISTS favorites_user_id_product_id_key;
DROP TABLE IF EXISTS "public"."favorites";
DROP SEQUENCE IF EXISTS favorites_id_seq;