安装 diesel_cli，只包含 mysql 的支持：
```
cargo install diesel_cli --no-default-features --features mysql
```
这个命令将只安装 diesel_cli 的 mysql 特性，不包括 postgres 和 sqlite 特性，所以不需要 -lpq 库。