# restaurant-order

## used libraries/tools
### web server
- actix: rank 1st in techempower benchmark; great docs
### Serialize/Deserialize
- serde
### REST client
- reqwest: for client in integration test
### CI
- github action: brand new, free, well-integrated
### other dev tools
- vscode devcontainer: always keep DEV environment as declared in JSON

## Design
Instead of using an actual database, this application works with in memory hashmap for simplicity.
A stateful struct called `OrderService` wraps the hashmap in order to segregate the responsibilities of HTTP request handling and business logics about order management application.
With actix-web, it allows share state across multiple worker threads so the `OrderService` is being shared.

In terms of the choice of data structures, although there is finite number of tables (i.e. 100), scarce store reserves memory such that hash map is chosen. 
The values are used with `Vector` because the food name can be duplicated by common sense.  


## To run locally
- `cargo run`
- HTTP server is hosted at `http://localhost:8000`
- endpoints are:
  - `GET /tables/{id}/order-items` to get ordered items of a table
  - `POST /tables/{id}/order-items` create order for a table
  - `DELETE /tables/{id}/order-items/{name}`

## clients and server interaction
- done in integration test for happy path: `cargo test`
- load test with > 10 threads for sort of benchmark `cargo test`