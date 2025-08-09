use tokio;
use discord_oxide::establish_gateway;

#[tokio::main]
async fn main(){
    match std::env::var("DISCORD_TOKEN"){
        Ok(var) => {
            match establish_gateway(&var, "succubot", &32776).await {
                Ok(_f) => {},
                Err(e) => eprintln!("{}", e.to_string())
            };
        },
        Err(e) => eprintln!("{}", &e)
    };
}
