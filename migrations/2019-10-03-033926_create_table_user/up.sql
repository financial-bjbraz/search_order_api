CREATE TABLE users(
   id serial PRIMARY KEY,
   name VARCHAR (50) UNIQUE NOT NULL,
   identity VARCHAR (50) NOT NULL,
   hometown VARCHAR (355) UNIQUE NOT NULL,
   age int4,
   created_on TIMESTAMP NOT NULL,
   last_login TIMESTAMP
);