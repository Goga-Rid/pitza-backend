CREATE SEQUENCE IF NOT EXISTS products_id_seq;

CREATE TABLE "public"."products" (
    "id" int4 NOT NULL DEFAULT nextval('products_id_seq'::regclass),
    "name" text NOT NULL,
    "description" text,
    "price" numeric(10,2) NOT NULL,
    "category" text NOT NULL,
    "image_url" text,
    "available" bool NOT NULL DEFAULT true,
    "weight" int4, -- Добавленная колонка для веса в граммах
    "created_at" timestamp NOT NULL DEFAULT now(),
    PRIMARY KEY ("id")
);

INSERT INTO "public"."products" ("name", "description", "price", "category", "image_url", "available", "weight", "created_at") VALUES
('Сырная', 'Моцарелла, сыры чеддер и пармезан, фирменный соус альфредо', 369.00, 'pizza', 'https://i.postimg.cc/k5szbwWG/59f5dc91c88719bb1c38a932dd2ca6a7e2a9f99d.png', 't', 400, '2025-05-19 13:52:41.654425'),
('Чилл Грилл', 'Цыпленок, маринованные огурчики, красный лук, соус гриль, моцарелла, чеснок, фирменный соус альфредо', 549.00, 'pizza', 'https://i.postimg.cc/XJ7MHtn9/751a7e58c2e611769fc2a6440cb0b82cc14c2492.png', 't', 420, '2025-05-19 13:52:41.698409'),
('Креветка и песто', 'Креветки, томаты, шампиньоны, соус песто, моцарелла, итальянские травы, фирменный томатный соус', 639.00, 'pizza', 'https://i.postimg.cc/d0tsXnRc/019591b642d87304a62d322945990861.png', 't', 430, '2025-05-19 13:52:41.738281'),
('Пепперони фреш', 'Пикантная пепперони, увеличенная порция моцареллы, томаты, фирменный томатный соус', 369.00, 'pizza', 'https://i.postimg.cc/cJ0PCPgP/611abef00d40539f733bdee740eef937c3f48948.png', 't', 410, '2025-05-19 13:52:41.782427'),
('Чоризо фреш ', 'Острые колбаски чоризо, сладкий перец, моцарелла, фирменный томатный соус', 369.00, 'pizza', 'https://i.postimg.cc/vm6qQwtr/11ee7d61706d472f9a5d71eb94149304.png', 't', 410, '2025-05-19 13:52:41.822704'),
('Кока-кола 0.5л', 'Газированный напиток Coca-Cola 0.5 литра', 109.00, 'drink', 'https://i.postimg.cc/650FSKMT/0194b770052874e5866fb322a5ccd52e.png', 't', 500, '2025-05-19 13:52:49.578401'),
('Чизкейк Нью-Йорк', 'Классический чизкейк с основой из печенья', 229.00, 'dessert', 'https://media.dodostatic.net/image/r:292x292/11eee20b6b6ec471ab74ab8f8885775b.jpg', 't', 150, '2025-05-19 13:52:49.613998'),
('Комбо ужин', 'Пицца Маргарита + напиток + закуска', 799.00, 'combo', 'https://i.postimg.cc/tCCBsxzC/0195960b3e227387aa74b7eabc1117b2-no-bg-preview-carve-photos.png', 'f', NULL, '2025-05-19 13:52:49.650486');