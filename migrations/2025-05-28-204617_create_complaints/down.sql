ALTER TABLE "public"."complaints" DROP CONSTRAINT IF EXISTS complaints_order_id_fkey;
ALTER TABLE "public"."complaints" DROP CONSTRAINT IF EXISTS complaints_user_id_fkey;
DROP TABLE IF EXISTS "public"."complaints";
DROP SEQUENCE IF EXISTS complaints_id_seq;