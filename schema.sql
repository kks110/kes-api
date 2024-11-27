DROP TABLE IF EXISTS Slabs;

CREATE TABLE IF NOT EXISTS Slabs (
    id INTEGER PRIMARY KEY,
    owner TEXT NOT NULL,
    for_sale BOOLEAN NOT NULL,
    card_name TEXT NOT NULL,
    card_number TEXT NOT NULL,
    set_name TEXT NOT NULL,
    tcg TEXT NOT NULL,
    language TEXT NOT NULL,
    cost REAL NOT NULL,
    grading_company TEXT NOT NULL,
    grade INTEGER NOT NULL,
    cert_number TEXT UNIQUE NOT NULL,
    price INTEGER,
    sold BOOLEAN NOT NULL,
    sold_value INTEGER,
    date_sold TEXT,
    notes TEXT NOT NULL,
    image_url TEXT NOT NULL
);

ALTER TABLE Slabs ADD COLUMN ace_label_url TEXT;