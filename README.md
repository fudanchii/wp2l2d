wp2l2d
---

wordpress rss feed converter to Line Today's custom XML format

powered by [Actix-web](https://actix.rs) [and](https://github.com/rust-syndication/rss) [friends](https://github.com/tafia/quick-xml).


how to build:
---

To build it yourself, ensure you have rust and cargo installed (we recommend using rustup),
then run:
```
cargo install
# or
cargo build --release
```

how to run:
---

Specify these configs as environment variables:

- `HOST` host ip for wp2l2d to bind to.
- `PORT` host port for wp2l2d to bind to.
- `WP_FEED_URL` your wordpress feed full url
- `LINE_NATIVE_COUNTRY` specify origin country where the feed come from
- `LINE_PUB_TO_COUNTRY` (optional) specify comma delimited list of country id where this feed allowed to publish to
- `LINE_EXCL_FROM_COUNTRY` (optional) specify comma delimited list of country id where this feed should excluded from
- `LINE_LANG` (optional) specify language in ISO 3166-1 alpha-2 if not specified, will be inferred from wordpress site

and then run
```bash
./target/release/wp2l2d
```

demo
---
- https://secret-ocean-18432.herokuapp.com/ping
- https://secret-ocean-18432.herokuapp.com/health
- https://secret-ocean-18432.herokuapp.com/line.xml
