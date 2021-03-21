use deles::delegators::{Person, Delegator, ListOptions};
use serde::Serialize;
use deles::GenericError;
use warp::http::StatusCode;
use futures::TryStreamExt;
use warp::Filter;

fn with_ctx(db: Delegator) -> impl Filter<Extract = (Delegator,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
fn json_body() -> impl Filter<Extract = (Person,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn respond<T: Serialize>(result: Result<T, GenericError>, status: warp::http::StatusCode) -> Result<impl warp::Reply, warp::Rejection> {
    match result {
        Ok(response) => {
            Ok(warp::reply::with_status(warp::reply::json(&response), status))
        }
        Err(err) => {
            log::error!("Error while trying to respond: {}", err.to_string());
            Err(warp::reject::custom(err))
        }
    }
}

pub fn party(
    ctx: Delegator,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    persons_list(ctx.clone())
        // .or(todos_create(db.clone()))
        // .or(todos_update(db.clone()))
        // .or(todos_delete(db))
}

/// GET /persons?offset=3&limit=5
pub fn persons_list(
    ctx: Delegator,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("persons")
        .and(warp::get())
        .and(warp::query::<ListOptions>())
        .and(with_ctx(ctx))
        .and_then(list_persons)
}

pub async fn list_persons(opts: ListOptions, ctx: Delegator) -> Result<impl warp::Reply, warp::Rejection> {
    respond( ctx.list_with_options::<Person>("Person", opts).await,
             warp::http::StatusCode::OK)
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    fn pretty<T>(val: &T) -> String
        where
            T: ?Sized + Serialize, {
        serde_json::to_string_pretty(val).unwrap()
    }

    async fn print_person(party_id: &str, items: &Vec<Person>) -> anyhow::Result<()> {
        let ex_sts: Vec<&Person> = items.iter()
            .filter(|&n| n.party_id == Some(party_id.to_string()))
            .collect();
        for ex in &ex_sts {
            println!("{} => ", ex.last_name.as_ref().unwrap());
            println!("{}", pretty(ex));
        }
        Ok(())
    }

    #[tokio::test]
    async fn list_ent_works() -> Result<(), GenericError> {
        let delegator = Delegator::new().await?;
        let rs: Vec<Person> = delegator.list("Person").await?;
        println!("total {}", rs.len());
        print_person("SCRUMADMIN", &rs).await?;
        Ok(())
    }
}


