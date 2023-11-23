# Actix backend with postgres & sqlx

## Prerequisite

- Rust, postgres & sqlx binary should be installed.
- Postgres url should be set in env file.

### Starting the server
- Server can be started with `cargo run`
- Run migration with `sqlx migrate run`
- Revert migration to delete tables with `sqlx migrate revert`

### Postman queries

- Create user _Post_ - `127.0.0.1:8081/create_user?user_mail=test@gmail.com&first_name=Some&second_name=thing`
- Create task _Post_ - `127.0.0.1:8081/create_task?user_mail=test@gmail.com&task_des=Buy Onion`
- Read task _Get_ - `127.0.0.1:8081/read_task/3`
- Update task _Put_ - `127.0.0.1:8081/update_task?user_mail=test@gmail.com&task_id=2&task_description=SellOldCar`
- Delete task _Delete_ - `127.0.0.1:8081/delete_task/2`
