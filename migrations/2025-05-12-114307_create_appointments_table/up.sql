CREATE TABLE IF NOT EXISTS "appointments" (
  "id" serial PRIMARY KEY,
  "patient_id" integer NOT NULL,
  "appointment_at" timestamp NOT NULL,
  "specialty" varchar(100) NOT NULL,
  "notes" text,
  "canceled" boolean NOT NULL DEFAULT FALSE,
  "canceled_at" timestamp,
  "cancellation_reason" text
);

ALTER TABLE IF EXISTS "appointments" ADD FOREIGN KEY ("patient_id") REFERENCES "patients" ("id");
