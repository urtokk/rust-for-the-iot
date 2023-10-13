-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXIST "comments" (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
);