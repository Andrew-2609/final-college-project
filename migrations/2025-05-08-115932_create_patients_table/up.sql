CREATE TABLE IF NOT EXISTS "patients" (
  "id" serial PRIMARY KEY,
  "name" varchar(255) NOT NULL,
  "cpf" varchar(11) UNIQUE NOT NULL
);
