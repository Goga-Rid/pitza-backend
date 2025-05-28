ALTER TABLE "public"."reviews" DROP CONSTRAINT IF EXISTS reviews_user_id_fkey;
ALTER TABLE "public"."reviews" DROP CONSTRAINT IF EXISTS reviews_product_id_fkey;
DROP TABLE IF EXISTS "public"."reviews";
DROP SEQUENCE IF EXISTS reviews_id_seq;