# web-rs
a web repo used rs.

## Note
### set env config_file_path
    config_file_path=**

### sqlx, must set env for DATABASE_URL like DATABASE_URL=mysql://maomao:maomao@123@192.168.9.111:3306/test
    DATABASE_URL=mysql://maomao:maomao@123@192.168.9.111:3306/test
### tracing-subscriber(features: time, local-time), set compiled param
    RUSTFLAGS="--cfg unsound_local_offset"

### postman collection json
[Postman Collection Json](./web-be/postman.json)

## Framework
- axum

## Database
- Redis
- Mysql
- MongoDB
