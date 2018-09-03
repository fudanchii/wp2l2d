[WIP] wp2l2d
---

wordpress rss feed converter to Line Today's custom XML format

powered by [Actix-web](https://actix.rs) [and](https://github.com/rust-syndication/rss) [friends](https://github.com/tafia/quick-xml).


how to build:
---

To build it yourself, ensure you have rust and cargo installed (we recommend using rustup),
then run:
```
cargo install
cargo build --release
```

how to run:
---

Specify these configs as environment variables:

- `WP2L2D_HOST` host ip for wp2l2d to bind to.
- `WP2L2D_PORT` host port for wp2l2d to bind to.
- `WP2L2D_CERT_FILE` (optional) cert file for ssl connection.
- `WP2L2D_KEY_FILE` (optional) private key file for ssl connection.
- `WP_FEED_URL` your wordpress feed full url
- `LINE_PUB_TO` (optional) specify comma delimited list of country id where this feed allowed to publish to
- `LINE_EXCL_FROM` (optional) specify comma delimited list of country id where this feed should excluded from
- `LINE_LANG` (optional) specify language in ISO 3166-1 alpha-2 if not specified, will be inferred from wordpress site

and then run
```bash
./wp2l2d
```
