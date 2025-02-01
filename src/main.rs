use std::env;

use dotenv::dotenv;
use regex::Regex;
use rpassword::read_password;
use tokio::signal;

use grammers_client::{
    grammers_tl_types as tl, session::Session, Client, Config, SignInError, Update::*,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let api_id = env::var("API_ID")?.parse::<i32>()?;
    let api_hash = env::var("API_HASH")?;
    let channel_username = env::var("CHANNEL_USERNAME")?;
    let session_name = "auth.session";

    let session = Session::load_file_or_create(session_name)?;

    let client = Client::connect(Config {
        session,
        api_id,
        api_hash,
        params: Default::default(),
    })
    .await?;

    if !client.is_authorized().await? {
        let mut phone = String::new();
        println!("Please enter your phone number: ");
        std::io::stdin().read_line(&mut phone)?;
        let token = client.request_login_code(phone.as_str()).await?;

        let mut code = String::new();
        println!("Please enter the code you received: ");
        std::io::stdin().read_line(&mut code)?;

        match client.sign_in(&token, &code).await {
            Ok(user) => user,
            Err(SignInError::PasswordRequired(password_token)) => {
                println!("Password is required for this account");
                println!("Please enter your password: ");
                let password = read_password()?;

                println!("Signing in...");
                client.check_password(password_token, password).await?
            }
            Err(err) => {
                println!("Failed to sign in as a user :(\n{}", err);
                return Err(err.into());
            }
        };

        let _ = client.session().save_to_file(session_name)?;
    }

    let user = client.get_me().await?;

    println!("Signed in as {}", user.full_name());

    let channel_username = channel_username.trim().to_string();

    println!("Listening for messages from {} ...", channel_username);
    let url_regex = Regex::new(r"https://t\.me/\+(\S+)")?;

    tokio::spawn(async move {
        loop {
            match client.next_update().await {
                Ok(update) => {
                    match update {
                        NewMessage(message) => {
                            let chat = message.chat();
                            let username = chat.username();

                            match username {
                                Some(username) => {
                                    if username == channel_username.as_str() {
                                        if let Some(captures) = url_regex.captures(message.text()) {
                                            if let Some(hash) = captures.get(1) {
                                                let req =
                                                    tl::functions::messages::ImportChatInvite {
                                                        hash: hash.as_str().to_string(),
                                                    };

                                                if let Err(e) = client.invoke(&req).await {
                                                    println!("Error joining chat: {}", e);
                                                }

                                                println!("Joined chat: {}", hash.as_str());
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            };
                        }
                        _ => {}
                    };
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            };
        }
    });

    signal::ctrl_c().await?;
    println!("Shutting down...");

    Ok(())
}
