# How to run this project
## Environment prepare
We use Ubuntu as the environment.

To use diesel_cli, make sure that you have installed `libpq-dev`, `libsqlite3-dev` and `libmysqlclient-dev`. Then run the below command:
```
cargo install diesel_cli
```
After installation you can migrate the tables into your database.

And we use `unzip` Command to unzip zip files, so you may need to install `unzip` by:
```
sudo apt install unzip
```