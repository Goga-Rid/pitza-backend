CREATE SEQUENCE IF NOT EXISTS users_id_seq;

CREATE TABLE "public"."users" (
    "id" int4 NOT NULL DEFAULT nextval('users_id_seq'::regclass),
    "email" text NOT NULL,
    "password" text NOT NULL,
    "name" text,
    "address" text,
    "created_at" timestamp NOT NULL DEFAULT now(),
    "role" text NOT NULL DEFAULT 'user'::text,
    PRIMARY KEY ("id")
);

CREATE UNIQUE INDEX users_email_key ON public.users USING btree (email);

INSERT INTO "public"."users" ("id", "email", "password", "name", "address", "created_at", "role") VALUES
(1, 'kritos.biz@gmail.com', '$argon2id$v=19$m=19456,t=2,p=1$hzMu5CQXnCelJqAK4rVr8g$SCufd48rbuy3cqUOWeTfUcVPE2VVuOC8CB+WQlHuQPo', 'Igors', 'fgfg', '2025-05-19 12:14:42.079551', 'user'),
(2, 'kritos.biz1@gmail.com', '$argon2id$v=19$m=19456,t=2,p=1$EPjHf6iNPNOsb+0N9mgsMQ$1nAG5/HV5/J/R83BP8v804eQ4Tj6IrrkGNIr3/t1zb0', 'Alex', NULL, '2025-05-19 15:00:42.410006', 'user'),
(3, 'kritos.biz11@gmail.com', '$argon2id$v=19$m=19456,t=2,p=1$Jo8U/Y9Q5jB8Qaf6TTQWcQ$xfEfA9yqT/1QdCe6ZgOdyuGp/8y7SZPktft9/dTzAYc', 'Dzhon', NULL, '2025-05-19 15:01:45.411816', 'user'),
(4, 'AbdualaAhmed228@gmail.com', '$argon2id$v=19$m=19456,t=2,p=1$o0yeb6GqAcE5mytLNffvMw$o8/mbVlEHQ6k48dpH48mIz474XMCDHx3RQ9XKdNdM/I', 'Abdula', 'Vietnam', '2025-05-19 15:54:27.791548', 'user');