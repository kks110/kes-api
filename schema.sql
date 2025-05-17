DROP TABLE IF EXISTS Slabs;

CREATE TABLE IF NOT EXISTS Slabs (
    id INTEGER PRIMARY KEY,
    card_name TEXT NOT NULL,
    card_number TEXT NOT NULL,
    set_name TEXT NOT NULL,
    tcg TEXT NOT NULL,
    language TEXT NOT NULL,
    grading_company TEXT NOT NULL,
    grade REAL NOT NULL,
    cert_number TEXT UNIQUE NOT NULL,
    slab_case TEXT,
    price INTEGER,
    sold BOOLEAN NOT NULL,
    sold_value INTEGER,
    postage_and_fees INTEGER,
    date_sold TEXT,
    notes TEXT NOT NULL,
    image_url TEXT NOT NULL,
    ace_label_url TEXT,
    listing_url TEXT,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);

create trigger update_timestamp
AFTER UPDATE ON Slabs
FOR EACH ROW
BEGIN
    UPDATE Slabs SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;
