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


## TODOs
- [ ] validate 1 <= table_id <= 100
- [ ] test with >10 threads of clientsfetch
