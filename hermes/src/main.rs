use daedalus::config::Difficulty;
use daedalus::exporter::{export_json, export_svg};
use daedalus::generator::generate_maze;

use poise::serenity_prelude as serenity;
use std::fs;
use std::path::PathBuf;
use tokio::process::Command;

struct Data;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command)]
async fn generate(
    ctx: Context<'_>,
    #[description = "Difficulty: easy, medium, hard, labyrinthian"] difficulty: String,
    #[description = "Generate a square maze"] square: Option<bool>,
) -> Result<(), Error> {
    ctx.defer().await?;

    let difficulty = match difficulty.to_lowercase().as_str() {
        "easy" => Difficulty::Easy,
        "medium" => Difficulty::Medium,
        "hard" => Difficulty::Hard,
        "labyrinthian" => Difficulty::Labyrinthian,
        _ => {
            ctx.say("Invalid difficulty. Use easy, medium, hard, or labyrinthian.")
                .await?;
            return Ok(());
        }
    };

    let square = square.unwrap_or(false);

    let width = difficulty.random_size();
    let height = if square {
        width
    } else {
        difficulty.random_size()
    };

    let maze = generate_maze(width, height);

    fs::create_dir_all("output/hermes")?;

    let json_path = "output/hermes/maze.json";
    let svg_path = "output/hermes/maze.svg";

    export_json(&maze, json_path)?;
    export_svg(&maze, svg_path)?;

    let message = format!(
        "Generated {} maze: {}x{}",
        difficulty.label(),
        maze.width,
        maze.height
    );

    ctx.send(
        poise::CreateReply::default()
            .content(message)
            .attachment(serenity::CreateAttachment::path(svg_path).await?)
            .attachment(serenity::CreateAttachment::path(json_path).await?),
    )
    .await?;

    Ok(())
}

#[poise::command(slash_command)]
async fn theseus(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    // Move from target/debug back to the workspace root.
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();

    let output = Command::new("cargo")
        .current_dir(&workspace)
        .args(["run", "-p", "theseus"])
        .output()
        .await?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !output.status.success() {
        ctx.say(format!(
            "Theseus failed.\n```text\n{}\n{}\n```",
            stdout, stderr
        ))
        .await?;

        return Ok(());
    }

    let bfs_svg = workspace.join("output/solved_maze_bfs.svg");
    let dfs_svg = workspace.join("output/solved_maze_dfs.svg");

    let mut reply = poise::CreateReply::default().content(format!(
        "Theseus completed successfully.\n```text\n{}\n```",
        stdout
    ));

    if bfs_svg.exists() {
        reply = reply.attachment(serenity::CreateAttachment::path(&bfs_svg).await?);
    }

    if dfs_svg.exists() {
        reply = reply.attachment(serenity::CreateAttachment::path(&dfs_svg).await?);
    }

    ctx.send(reply).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let token = std::env::var("DISCORD_TOKEN")
        .expect("Missing DISCORD_TOKEN in .env file");

    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                generate(),
                theseus(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(
                    ctx,
                    &framework.options().commands,
                )
                .await?;

                Ok(Data)
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await
        .expect("Failed to create Discord client");

    println!("Hermes is online.");

    client.start().await.expect("Discord client error");
}