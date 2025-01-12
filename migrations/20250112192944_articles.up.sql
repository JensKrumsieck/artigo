-- Add up migration script here
CREATE TABLE
    articles (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        slug VARCHAR(255) UNIQUE NOT NULL,
        title VARCHAR(255) NOT NULL,
        body TEXT NOT NULL,
        hero_image VARCHAR(255),
        tags JSON DEFAULT '[]',
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
    );

-- create dummy
INSERT INTO
    articles (slug, title, body, hero_image, tags)
VALUES
    (
        'hello-world',
        'Hello World',
        'This is my first article',
        'https://example.com/hello-world.jpg',
        '["hello", "world"]'
    );