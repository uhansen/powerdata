CREATE TABLE IF NOT EXISTS Customer 
( 
id uniqueidentifier PRIMARY KEY,
first_name TEXT NOT NULL,
last_name TEXT NOT NULL,
email TEXT NOT NULL,
eloverblik_key TEXT NOT NULL,
street TEXT NOT NULL,
city TEXT NOT NULL,
zip_code TEXT NOT NULL,
country TEXT NOT NULL,
longitude REAL NOT NULL,
latitude REAL NOT NULL,
meter_number TEXT NOT NULL
);
