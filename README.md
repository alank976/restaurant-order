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

In terms of the choice of data structures, HashMap<u8 


## TODOs
- [ ] separate and unit test
- [ ] validate 1 <= table_id <= 100
- [ ] assign random cooking time to 5-15 min
- [ ] test with >10 threads of clients
