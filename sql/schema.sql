\connect "postgres";
DROP TABLE IF EXISTS "estates";
DROP SEQUENCE IF EXISTS estates_id_seq;
CREATE SEQUENCE estates_id_seq INCREMENT 1 MINVALUE 1 MAXVALUE 9223372036854775807 CACHE 1;

CREATE TABLE "public"."estates" (
    "id" BIGINT DEFAULT nextval('estates_id_seq') NOT NULL,
    "address" text NOT NULL,
    "price" integer NOT NULL,
    "kind" text NOT NULL,
    "prepay" integer,
    "description" text NOT NULL,
    CONSTRAINT "estates_pkey" PRIMARY KEY ("id")
)
WITH (oids = false);
