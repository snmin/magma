<img width = "50%" src="logo/default.png">


[![Rust](https://github.com/MagmaERP/magma-services/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/MagmaERP/magma-services/actions/workflows/rust.yml)&nbsp;&nbsp;![Discord](https://img.shields.io/discord/898797518190698538)&nbsp;&nbsp;


# Magma

This project is meant to be a learning process for me and something I kinda of want to pursue. If this works out, ill be suprised lmao.

## Creating a service

The base-service has all that needs to be done. Rename the Cargo.toml and directorys for the appropriate service being created. If there are larger models needed put them in a ```models/``` folder. If database access is needed, the folder should be called ```db/``` for consistency.

After that, make sure to add the service to the mono-Cargo.toml at the root of the project. This is so github actions and vscode know where it is.

```
    let schema = Schema::new(Query, EmptyMutation, EmptySubscription);

    warp::serve(graphql(schema).and_then(
        |(schema, request): (
            Schema<Query, EmptyMutation, EmptySubscription>,
            async_graphql::Request,
        )| async move {
            Ok::<_, Infallible>(warp::reply::json(&schema.execute(request).await).into_response())
        },
    ))
    .run(([0, 0, 0, 0], 7980))
    .await;
```

Inside the ```main.rs``` file in ```base-service/``` each of the schema *types* are entered into the Schema. Once you need to mutate something or do something with subscriptions. Each of the three action types have structs that are their names (ie: ```pub struct Query; || pub struct Subscription; || pub struct Mutation;```)




## Gateway

Currently the gateway exists mearly as a single point to connect to all of the running services. At some point, i would like the ability for services to communicate with eachother through NAT or through the gateway someway. Though, this node server is fine for now.

Inside ```gateway/index.js``` add your service address to the serviceList.
supergraphsdl is the next step.



## Services

Documentation for each of the services.

### **Inventory Service**
port: 7980

