-- Add up migration script here
CREATE TABLE
    articles (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        slug VARCHAR(255) UNIQUE NOT NULL,
        title VARCHAR(255) NOT NULL,
        body TEXT NOT NULL,
        hero_image VARCHAR(255),
        tags TEXT[],
        created_at TIMESTAMPTZ DEFAULT now() NOT NULL,
        updated_at TIMESTAMPTZ DEFAULT now() NOT NULL
    );

CREATE INDEX ON articles(slug);
CREATE INDEX ON article(created_at DESC);

-- create dummy
INSERT INTO
    articles (slug, title, body, hero_image, tags)
VALUES
    (
        'hello-world',
        'Hello World',
        'This is my first article',
        'https://example.com/hello-world.jpg',
        ARRAY['hello', 'world']
    );