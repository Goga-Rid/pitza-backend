ALTER TABLE "public"."order_items" DROP CONSTRAINT IF EXISTS order_items_order_id_fkey;
ALTER TABLE "public"."order_items" DROP CONSTRAINT IF EXISTS order_items_product_id_fkey;
DROP TABLE IF EXISTS "public"."order_items";
DROP SEQUENCE IF EXISTS order_items_id_seq;