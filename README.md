"# rust web app " 

Tokio è per scrivere codice asincrono su Rust
https://crates.io/crates/tokio

## For DB
```sh
# Start database
docker run --rm -p 5432:5432 -e "POSTGRES_PASSWORD=postgres" --name pg postgres:14

# optional psql (un altro terminale)
docker exec -it -u postgres pq psql

```


## Dev test
```sh
cargo watch -q -c -w src/ -x 'test model_db -- --test-threads=1 --nocapture'
```
-q: quite
-c: clear
-w: watch
src/: path dove andare a cercare i test
-x: per specificare i parametri dopo
test: cosa andare a "runnare"
model_db: filtro di ricerca per cosa andare a runnare
--test-threads: quanti thread andare a utilizzare
--nocaputre: per solo sviluppo


## Dev Web
Bisogna essere all'inerno di rust ovviamente
```sh
cargo watch -q -c -w src/ -x 'run -- ../frontend/web'
```

Test
```
cargo watch -q -c -w src/ -x 'test web -- --test-threads=1 --nocapture'
