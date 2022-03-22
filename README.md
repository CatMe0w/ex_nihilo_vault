# ex_nihilo_vault

中文版请见[这里](https://github.com/CatMe0w/ex_nihilo_vault/blob/master/README_zh.md)。

Project Ex Nihilo: backend "Vault"

## Quick start

Rust is required.

```
git clone https://github.com/CatMe0w/ex_nihilo_vault
cd ex_nihilo_vault
xz -d vault.db.xz
cargo run --release
```

## Deploy

### Download directly

Download `vault.db.xz` from this repository, then decompress it to the root directory of the project.

### Prepare by yourself

1. Download and decompress proma.db.xz: https://github.com/CatMe0w/proma/releases/download/trigger-1/proma.db.xz
2. Download and decompress uncover.db.xz: https://github.com/CatMe0w/backstage_uncover/releases/download/trigger-1/uncover.db.xz
3. Add prefix `pr_` to all tables in proma.db
4. Add prefix `un_` to all tables in uncover.db
5. Merge all tables, save as `vault.db`
6. Add indexes if needed
7. `git clone https://github.com/CatMe0w/ex_nihilo_vault`
8. `cd ex_nihilo_vault`
9. Put `vault.db` here
10. `cargo run --release`

## License

[MIT License](https://opensource.org/licenses/MIT)
