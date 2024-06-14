use std::sync::Arc;

use axum::{Extension, Router};
use axum::routing::{get, MethodFilter, on};
use juniper::{EmptyMutation, EmptySubscription, FieldError, graphql_object, ID, RootNode};
use juniper::futures::stream::BoxStream;
use juniper_axum::{graphiql, graphql, playground};
use juniper_graphql_ws::ConnectionConfig;
use sea_orm::{DatabaseConnection};
use crate::graphql::{KiwiServiceQuery, GraphQLSchema, KiwiServiceMutation};


pub fn graphql_endpoint(
    database_connection: DatabaseConnection
) -> Router<()> {
    let schema = GraphQLSchema::new(KiwiServiceQuery {
        
    }, KiwiServiceMutation{
        
    }, EmptySubscription::new());

    let router = Router::new()
        .route(
            "/subscriptions",
            get(juniper_axum::ws::<Arc<GraphQLSchema>>(ConnectionConfig::new(()))),
        )
        .route(
            "/graphql",
            on(
                MethodFilter::GET.or(MethodFilter::POST),
                graphql::<Arc<GraphQLSchema>>,
            ),
        )
        .route("/graphiql", get(graphiql("/graphql", "/subscriptions")))
        .route("/playground", get(playground("/graphql", "/subscriptions")))
        .layer(Extension(Arc::new(schema)));

    return router;
}
