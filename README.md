## url-shortener
Based on Alex Xu's System Design implementation.

## Why use base 62 ?
1. Can represent large numbers up to 2 ** 62 = 4611686018427387904
2. Base 62 uses a-z, A-Z 0-9, this creates 62 possible symbols for each encoded number and a friendly url

## In-depth analysis
Assume 100 million urls are generated daily.
The service must run for 10 years, this means 100 million * 365 * 10 = 365 billion.

Base 62 is capable of generating >> 365billion entries.

## Database
1. `diesel setup --database-url=postgresql://localhost:5432/url_shortener`
2. `diesel migration generate <NAME_OF_MIGRATION>`
3. `diesel migration run`

## Build
1. Run `cargo build --release --bin url-shortener` first.