CREATE TABLE "admins" (
  "id" serial PRIMARY KEY,
  "name" varchar(100) NOT NULL,
  "email" varchar(150) UNIQUE NOT NULL,
  "password_hash" varchar(255) NOT NULL
);
