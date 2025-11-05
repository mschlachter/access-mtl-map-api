# Access MTL Map API

Small API to fetch the list of Access Montreal partners with geo-coordinate data added. Can be used to build a map of places to use an Access Montreal card, e.g. https://www.schlachter.xyz/projects/access-montreal-map

Contains data scraped from https://montreal.ca/en/articles/acces-montreal-card-exclusive-offers-and-discounts-5990 and https://montreal.ca/articles/carte-acces-montreal-offres-et-rabais-exclusifs-5990

Endpoints
- GET /data/raw - returns the raw JSON records
- GET /data/<lang> - returns localized data for language `en` or `fr` (e.g. `/data/en`)

## Run locally

Make sure the Rust toolchain is installed and then run:

```bash
cargo run
```

To run tests use:

```bash
cargo test
```
