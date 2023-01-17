# web-rs
a web repo used rs.

## Note
### set env config_file_path
    config_file_path=**


### because sqlx, mut set env for DATABASE_URL like DATABASE_URL=mysql://maomao:maomao@123@192.168.9.111:3306/test
    export DATABASE_URL=mysql://maomao:maomao@123@192.168.9.111:3306/test
    cargo run --package web-be --bin web-be 
### because tracing-subscriber(features: time, local-time), set compiled param
    RUSTFLAGS="--cfg unsound_local_offset"

### postman collection json
[Postman Collection Json](./web-be/postman.json)
