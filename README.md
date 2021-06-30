# actix-graphql-demo

[![Build Status](https://travis-ci.org/dwisulfahnur/actix-graphql-demo.svg?branch=master)](https://travis-ci.org/dwisulfahnur/actix-graphql-demo)

***Published on the [actix/examples repository](https://github.com/actix/examples/tree/master/graphql/juniper-advanced)***

GraphQL Implementation in Rust using Actix, Juniper, and Mysql as Database

# Prerequites
- Rust Installed
- MySql as Database

# Database Configuration

Create a new database for this project, and import the existing database schema has been provided named ```mysql-schema.sql```.

Create ```.env``` file on the root directory of this project and set environment variable named ```DATABASE_URL```, the example file has been provided named ```.env.example```, you can see the format on there.

# Run


```sh
# go to the root dir
cd actix-graphql-demo

# Run
cargo run
```

### GraphQL Playground

http://127.0.0.1:8080/graphiql
