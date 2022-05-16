# ex_nihilo_vault

中文版请见[这里](https://github.com/CatMe0w/ex_nihilo_vault/blob/master/README_zh.md)。

Project Ex Nihilo: backend "Vault"

## Quick start

### Use containers

```
docker run -d --name ex_nihilo_vault -p 127.0.0.1:8000:8000 --restart always ghcr.io/catme0w/ex_nihilo_vault
```

### Don't use containers

Rust is required.

```
git clone https://github.com/CatMe0w/ex_nihilo_vault
cd ex_nihilo_vault
xz -d vault.db.xz
cargo run --release
```

## Get `vault.db`

Running ex_nihilo_vault requires the `vault.db` file, which is the dataset for Project Ex Nihilo.

Note: The prebuilt container images already packaged `vault.db`. If you use the prebuilt images, you can ignore this section.

### Download directly

Download `vault.db.xz` from this repository, then decompress it to the root directory of the project.

See "Quick start" above for the detailed steps.

### Prepare by yourself

1. Download and decompress proma.db.xz:  
https://github.com/CatMe0w/proma/releases/download/trigger-1/proma.db.xz
2. Download and decompress uncover.db.xz:  
https://github.com/CatMe0w/backstage_uncover/releases/download/trigger-1/uncover.db.xz
3. Add prefix `pr_` to all tables in proma.db
4. Add prefix `un_` to all tables in uncover.db
5. Merge all tables, save as `vault.db`
6. Add indexes if needed
7. `git clone https://github.com/CatMe0w/ex_nihilo_vault`
8. `cd ex_nihilo_vault`
9. Put `vault.db` here
10. `cargo run --release`

## Caveats

- Do not build for `x86_64-unknown-linux-musl` target or the executable will exit immediately with a segmentation fault.
- ex_nihilo_vault does not provide a TLS support. You need to use a nginx/apache/caddy/etc. reverse proxy to provide HTTPS access.
- Change the `address` field to `127.0.0.1` in `Rocket.toml` if you do not want to use containers or configure firewalls.

## License

[MIT License](https://opensource.org/licenses/MIT)
