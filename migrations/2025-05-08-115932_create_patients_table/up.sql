CREATE TABLE IF NOT EXISTS "patients" (
  "id" serial PRIMARY KEY,
  "name" varchar(255) NOT NULL,
  "cpf" string UNIQUE NOT NULL
);
