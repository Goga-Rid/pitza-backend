CREATE SEQUENCE IF NOT EXISTS products_id_seq;

CREATE TABLE "public"."products" (
    "id" int4 NOT NULL DEFAULT nextval('products_id_seq'::regclass),
    "name" text NOT NULL,
    "description" text,
    "price" numeric(10,2) NOT NULL,
    "category" text NOT NULL,
    "image_url" text,
    "available" bool NOT NULL DEFAULT true,
    "created_at" timestamp NOT NULL DEFAULT now(),
    PRIMARY KEY ("id")
);

INSERT INTO "public"."products" ("id", "name", "description", "price", "category", "image_url", "available", "created_at") VALUES
(1, 'Сырная', 'Моцарелла, сыры чеддер и пармезан, фирменный соус альфредо', 369.00, 'pizza', 'https://media.dodostatic.net/image/r:292x292/11ee7d610d2925109ab2e1c92cc5383c.jpg', 't', '2025-05-19 13:52:41.654425'),
(2, 'Чилл Грилл', 'Цыпленок, маринованные огурчики, красный лук, соус гриль, моцарелла, чеснок, фирменный соус альфредо', 549.00, 'pizza', 'https://media.dodostatic.net/image/r:292x292/019591c69fac7921a27e4ecd8c99f9df.jpg', 't', '2025-05-19 13:52:41.698409'),
(3, 'Креветка и песто', 'Креветки, томаты, шампиньоны, соус песто, моцарелла, итальянские травы, фирменный томатный соус', 639.00, 'pizza', 'https://media.dodostatic.net/image/r:292x292/019591b642d87304a62d322945990861.jpg', 't', '2025-05-19 13:52:41.738281'),
(4, 'Пепперони фреш', 'Пикантная пепперони, увеличенная порция моцареллы, томаты, фирменный томатный соус', 369.00, 'pizza', 'https://media.dodostatic.net/image/r:292x292/11ee7d612fc7b7fca5be822752bee1e5.jpg', 't', '2025-05-19 13:52:41.782427'),
(5, 'Чоризо фреш ', 'Острые колбаски чоризо, сладкий перец, моцарелла, фирменный томатный соус', 369.00, 'pizza', 'https://media.dodostatic.net/image/r:292x292/11ee7d61706d472f9a5d71eb94149304.jpg', 't', '2025-05-19 13:52:41.822704'),
(6, 'Кока-кола 0.5л', 'Газированный напиток Coca-Cola 0.5 литра', 109.00, 'drink', 'https://media.dodostatic.net/image/r:292x292/0194b770052874e5866fb322a5ccd52e.jpg', 't', '2025-05-19 13:52:49.578401'),
(7, 'Чизкейк Нью-Йорк', 'Классический чизкейк с основой из печенья', 229.00, 'dessert', 'https://media.dodostatic.net/image/r:292x292/11eee20b6b6ec471ab74ab8f8885775b.jpg', 't', '2025-05-19 13:52:49.613998'),
(8, 'Комбо ужин', 'Пицца Маргарита + напиток + закуска', 799.00, 'combo', 'https://media.dodostatic.net/image/r:292x292/0195960b3e227387aa74b7eabc1117b2.jpg', 'f', '2025-05-19 13:52:49.650486');