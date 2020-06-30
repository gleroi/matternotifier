extern crate hyper;
extern crate futures;

use mm::apis::client::*;
use mm::apis::configuration::*;
use futures::executor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = Configuration::new(hyper::Client::new());
    cfg.base_path = "https://host/api/v4".to_owned();
    cfg.basic_auth = Some(("username".to_owned(), Some("password".to_owned())));

    let client = APIClient::new(cfg);
    let call = async {
        let teams = client.teams_api().teams_get(None, None, None).await;
        for team in teams {
            println!("team: {:?}", team);
        }
    };
    executor::block_on(call);
    Ok(())
}
