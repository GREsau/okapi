# Contribute to this project

In this document are some instructions that might help you contribute to this project.

Main maintainers:
- [@Ralpha](https://github.com/ralpha)
- [@GREsau](https://github.com/GREsau)

## Error while testing
If you run the tests `cargo test` and get the error:
```
  = note: /usr/bin/ld: cannot find -lsqlite3: No such file or directory
          collect2: error: ld returned 1 exit status

error: could not compile `rocket_okapi` (test "db_pool") due to 1 previous error
```
You need to install `libsqlite3-dev`. You can do this using your package manager.
```bash
sudo apt install libsqlite3-dev
```
