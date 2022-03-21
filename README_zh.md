# ex_nihilo_vault

Project Ex Nihilo：后端 “档案馆”

## 快速开始

需要 Rust。

```
git clone https://github.com/CatMe0w/ex_nihilo_vault
cd ex_nihilo_vault
xz -d vault.db.xz
cargo run --release
```

## 部署

### 直接下载

在本仓库内下载 `vault.db.xz` 文件，然后将其解压到项目根目录下。

### 自行准备

1. 下载并解压 proma.db.xz: https://github.com/CatMe0w/proma/releases/download/trigger-1/proma.db.xz
2. 下载并解压 uncover.db.xz: https://github.com/CatMe0w/backstage_uncover/releases/download/trigger-1/uncover.db.xz
3. 为 proma.db 内的所有表添加 `pr_` 前缀
4. 为 uncover.db 内的所有表添加 `un_` 前缀
5. 合并所有表，保存为 `vault.db`
6. 视情况添加索引

## 开源许可

[MIT License](https://opensource.org/licenses/MIT)
