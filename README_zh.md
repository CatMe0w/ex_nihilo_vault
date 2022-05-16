# ex_nihilo_vault

Project Ex Nihilo：后端 “档案馆”

## 快速开始

### 使用容器

```
docker run -d --name ex_nihilo_vault -p 127.0.0.1:8000:8000 --restart always ghcr.io/catme0w/ex_nihilo_vault
```

### 不使用容器

需要 Rust。

```
git clone https://github.com/CatMe0w/ex_nihilo_vault
cd ex_nihilo_vault
xz -d vault.db.xz
cargo run --release
```

## 获取 `vault.db`

运行 ex_nihilo_vault 需要 `vault.db` 文件，它是 Project Ex Nihilo 的数据集。

注：预编译容器镜像已经打包了 `vault.db` 文件。若使用预编译镜像，可忽略这部分。

### 直接下载

在本仓库内下载 `vault.db.xz` 文件，然后将其解压到项目根目录下。

具体操作请参考上方的“快速开始”。

### 自行准备

1. 下载并解压 proma.db.xz：  
https://github.com/CatMe0w/proma/releases/download/trigger-1/proma.db.xz
2. 下载并解压 uncover.db.xz：  
https://github.com/CatMe0w/backstage_uncover/releases/download/trigger-1/uncover.db.xz
3. 为 proma.db 内的所有表添加 `pr_` 前缀
4. 为 uncover.db 内的所有表添加 `un_` 前缀
5. 合并所有表，保存为 `vault.db`
6. 视情况添加索引
7. `git clone https://github.com/CatMe0w/ex_nihilo_vault`
8. `cd ex_nihilo_vault`
9. 把 `vault.db` 放在这里
10. `cargo run --release`

## 注意

- 不要编译到 `x86_64-unknown-linux-musl` target，否则可执行程序会报错 Segmentation fault 并立即退出。
- ex_nihilo_vault 不提供 TLS 支持，请使用 nginx/apache/caddy 等反向代理来提供 HTTPS 访问。
- 若不使用容器且不想配置防火墙，请将 `Rocket.toml` 中的 `address` 改为 `127.0.0.1`。

## 开源许可

[MIT License](https://opensource.org/licenses/MIT)
