// Example for the following slash command structure:
// {
//     "name": "foo",
//     "description": "hello",
//     "options": [
//         {
//             "type": 3,
//             "name": "text",
//             "description": "aaa",
//             "required": true
//         },
//         {
//             "type": 7,
//             "name": "channel",
//             "description": "bbb",
//             "required": true
//         },
//         {
//             "type": 4,
//             "name": "integer",
//             "description": "ccc",
//             "required": false
//         }
//     ]
// }
//
// Additionally, set the DISCORD_TOKEN and DISCORD_ID environment variables

use serenity::client::{Context, EventHandler};
use serenity::model::interactions::application_command::ApplicationCommandInteraction;
use serenity::model::interactions::{Interaction, InteractionResponseType};
use serenity::{async_trait, Client};
use serenity_slash_decode::Error as SlashError;
use serenity_slash_decode::{process, SlashMap};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

enum CustomError<'a> {
    SlashError(SlashError<'a>),
    CommandNotFound(String),
}

impl Display for CustomError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            // serenity-slash-decode's error type implements Display
            CustomError::SlashError(e) => e.fmt(f),
            CustomError::CommandNotFound(s) => f.write_str(&*format!("Command `{}` not found", s)),
        }
    }
}

impl<'a> From<SlashError<'a>> for CustomError<'a> {
    fn from(e: SlashError<'a>) -> Self {
        CustomError::SlashError(e)
    }
}

type CustomResult<'a, T> = Result<T, CustomError<'a>>;

async fn handle_command<'a>(
    ctx: &'a Context,
    interaction: &'a ApplicationCommandInteraction,
    args: &'a SlashMap,
) -> CustomResult<'a, ()> {
    let text = args.get_string("text")?;
    let mut message = format!(
        "text: {}\nchannel: {}",
        text,
        args.get_channel("channel")?.name
    );
    if let Ok(s) = args.get_integer("integer") {
        message.push_str(&*format!("\ninteger: {}", s));
    };
    interaction
        .create_interaction_response(ctx.http.clone(), |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|data| data.content(message))
        })
        .await
        .unwrap();
    Ok(())
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        // only handle slash commands
        let data = match &interaction {
            Interaction::ApplicationCommand(s) => &s.data,
            _ => return,
        };
        let (path, args) = process(data);
        let command = interaction.application_command().unwrap();
        match match path.as_str() {
            "foo" => handle_command(&ctx, &command, &args).await,
            _ => Err(CustomError::CommandNotFound(path)),
        } {
            Ok(_) => {}
            Err(e) => {
                command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|data| data.content(format!("Error: {}", e)))
                    })
                    .await
                    .unwrap();
            }
        };
    }
}

#[tokio::main]
async fn main() {
    // make sure to set these environment variables!
    let mut client = Client::builder(std::env::var("DISCORD_TOKEN").unwrap())
        .application_id(u64::from_str(&*std::env::var("DISCORD_ID").unwrap()).unwrap())
        .event_handler(Handler)
        .await
        .unwrap();
    if let Err(e) = client.start().await {
        println!("Runtime error: {:?}", e);
    }
}
