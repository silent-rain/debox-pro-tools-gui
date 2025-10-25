# 后台管理-后端

这个是一个后端接口服务。

## 技术栈

- axum
- tokio
- Tower

## 编译与调试

### 代码检查

```shell
cargo clippy
# or
cargo clippy -p admin
```

### 调试模式

```shell
cd apps/admin

cargo run -p admin

# 自动重新加载
# 每当源代码发生更改时，应用程序都会重新编译并重新启动。它用于listenfd将连接从旧版本的应用程序迁移到新编译的版本。
# cargo install cargo-watch systemfd
systemfd --no-pid -s http::3000 -- cargo watch -x run -p admin
```

### 生产模式

```shell
cd apps/admin

cargo build -p admin
```

### 查询依赖

```shell
cd apps/admin

cargo +nightly udeps -p admin

cargo +nightly udeps --workspace
```

## 开发文档

```shell
cd apps/admin

cargo doc -p admin
```

## 服务

- [前端服务](http://127.0.0.1:3000/admin)
- [后端服务](http://127.0.0.1:3000/api/v1/)

## 相关文档

- [axum - 生态系统](https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md)
- [rust axum sse keep_alive](https://www.cnblogs.com/soarowl/p/18320061)
- [SSE(Server Send Event)服务端推送](https://cloud.tencent.com/developer/article/1988605)
- [tonic examples](https://github.com/hyperium/tonic/tree/master/examples)

- axum错误处理
  - [自定义 extractor](https://www.cnblogs.com/pythonClub/p/17804708.html)
  - [Axum的错误处理模块](https://yuxuetr.com/wiki/axum/axum-error-handling)
  - [axum中的各种响应](https://www.cnblogs.com/pythonClub/p/17804749.html)
  - [Axum的response模块](https://yuxuetr.com/wiki/axum/axum-response)

- axum中间件
  - [Axum中如何实现中间件？](https://yuxuetr.com/blog/2024/06/09/axum-middleware)
  - [Axum的middleware模块](https://yuxuetr.com/wiki/axum/axum-middleware)

- 数据库
  - [sea-orm](https://www.sea-ql.org/SeaORM/docs/index/)

- 参数校验
  - [validation](https://dev.to/chaudharypraveen98/form-validation-in-rust-404l)
  - [validator](https://lib.rs/crates/validator)

- web proto
  - [stephenh/ts-proto](https://github.com/stephenh/ts-proto)
  - [timostamm/protobuf-ts](https://github.com/timostamm/protobuf-ts)

- curd 设计参考
  - <https://github.com/zenlex/crustd/blob/main/src/crud_traits.rs>
