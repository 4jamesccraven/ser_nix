An opinionated Nix serialiser, using the serde frame-work.

> [!NOTE]
> This is not a deserialiser, for that look into
> [tvix_serde](https://docs.tvix.dev/rust/tvix_serde/index.html) (which only
> implements the deserialiser half). I make no guarantees of compatibility
> between the two.

Currently ser_nix only exposes a to_string function, and an error type. Ser_nix
is new software, and currently only exists to fulfill a need I had for my [main
project](https://github.com/4jamesccraven/mkdev). The decision to release it
publicly was in hope that others might wish to contribute and improve, as the
ecosystem seemed bare in this regard.

Please open an issue or contact me via email for any concerns related to this
crate.
