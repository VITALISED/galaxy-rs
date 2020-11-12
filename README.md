# Galaxy-rs

An implementation of Voxtl's API in Rust.

Requires the `diesel_cli` cargo binary to run the database migrations (obviously). This project uses postgres specifically so you should install it with only that feature.

Currently handles 
`GET /users`
`GET /users/{id}`
`POST /users`
`DELETE /users`

Should be able to handle ingest and other stuff later on.