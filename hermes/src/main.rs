use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use std::sync::atomic::{
    AtomicU64,
    Ordering,
};
use std::time::{Duration, Instant};

use daedalus::config::Difficulty;
use daedalus::exporter::{
    export_json,
    export_svg,
};
use daedalus::generator::generate_maze;

use poise::serenity_prelude as serenity;
use serde_json::Value;
use tokio::process::{
    Child,
    Command,
};
use tokio::sync::Mutex;

const UPDATE_INTERVAL_SECONDS: u64 = 600;
const PROCESS_CHECK_SECONDS: u64 = 10;

const CHECKPOINT_DIRECTORY: &str =
    "output/theseus/checkpoints";

const TRAINING_LOG_DIRECTORY: &str =
    "output/hermes/training_logs";

struct TrainingProcess {
    id: u64,
    child: Child,
    started_at: Instant,
    description: String,
    channel_id: serenity::ChannelId,
}

struct Data {
    training:
        Arc<Mutex<Option<TrainingProcess>>>,

    next_training_id:
        Arc<AtomicU64>,
}

type Error =
    Box<dyn std::error::Error + Send + Sync>;

type Context<'a> =
    poise::Context<'a, Data, Error>;

#[derive(Clone, Copy)]
enum TrainingMode {
    New,
    Resume,
}

#[derive(Clone)]
enum TrainingLimit {
    Mazes(u64),
    Hours(f64),
}

#[derive(Debug)]
struct CheckpointSummary {
    mazes_completed: u128,
    total_examples_trained: u128,

    latest_training_loss: f64,
    latest_training_accuracy: f64,

    theseus_solved: bool,
    theseus_nodes_explored: u128,
    theseus_duration_nanos: u128,
    theseus_path_length: u128,

    latest_teacher: String,
    teacher_nodes_explored: u128,
    teacher_duration_nanos: u128,
    teacher_path_length: u128,

    maze_width: u64,
    maze_height: u64,
}

#[poise::command(slash_command)]
async fn generate(
    ctx: Context<'_>,

    #[description = "Difficulty: easy, medium, hard, labyrinthian"]
    difficulty: String,

    #[description = "Generate a square maze"]
    square: Option<bool>,
) -> Result<(), Error> {
    ctx.defer().await?;

    let difficulty =
        match difficulty.to_lowercase().as_str() {
            "easy" => Difficulty::Easy,
            "medium" => Difficulty::Medium,
            "hard" => Difficulty::Hard,
            "labyrinthian" => {
                Difficulty::Labyrinthian
            }
            _ => {
                ctx.say(
                    "Invalid difficulty. Use easy, \
                     medium, hard, or labyrinthian.",
                )
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

    let maze =
        generate_maze(width, height);

    fs::create_dir_all("output")?;

    let json_path = "output/maze.json";
    let svg_path = "output/maze.svg";

    export_json(&maze, json_path)?;
    export_svg(&maze, svg_path)?;

    let message = format!(
        "Generated {} maze.\n\
         Size: {}x{}\n\
         Square: {}\n\
         Files saved:\n\
         - `{}`\n\
         - `{}`",
        difficulty.label(),
        maze.width,
        maze.height,
        square,
        json_path,
        svg_path
    );

    ctx.say(message).await?;

    Ok(())
}

#[poise::command(slash_command)]
async fn training_start(
    ctx: Context<'_>,

    #[description = "Number of mazes to train on"]
    mazes: Option<i64>,

    #[description = "Number of hours to train"]
    hours: Option<f64>,
) -> Result<(), Error> {
    ctx.defer().await?;

    let limit =
        match parse_training_limit(
            mazes,
            hours,
        ) {
            Ok(limit) => limit,
            Err(message) => {
                ctx.say(message).await?;
                return Ok(());
            }
        };

    start_training_process(
        ctx,
        TrainingMode::New,
        limit,
    )
    .await
}

#[poise::command(slash_command)]
async fn training_resume(
    ctx: Context<'_>,

    #[description = "Additional mazes to train on"]
    mazes: Option<i64>,

    #[description = "Additional hours to train"]
    hours: Option<f64>,
) -> Result<(), Error> {
    ctx.defer().await?;

    let limit =
        match parse_training_limit(
            mazes,
            hours,
        ) {
            Ok(limit) => limit,
            Err(message) => {
                ctx.say(message).await?;
                return Ok(());
            }
        };

    start_training_process(
        ctx,
        TrainingMode::Resume,
        limit,
    )
    .await
}

#[poise::command(slash_command)]
async fn training_status(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.defer().await?;

    let workspace = workspace_path();

    let running_description = {
        let mut guard =
            ctx.data().training.lock().await;

        refresh_finished_process(&mut guard)?;

        guard.as_ref().map(|training| {
            (
                training.description.clone(),
                training.started_at.elapsed(),
            )
        })
    };

    let checkpoint =
        load_newest_checkpoint_summary(
            &workspace,
        )?;

    let message = format_status_message(
        running_description,
        checkpoint.as_ref(),
    );

    ctx.say(message).await?;

    Ok(())
}

#[poise::command(slash_command)]
async fn training_data(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.defer().await?;

    let workspace = workspace_path();

    let checkpoint =
        load_newest_checkpoint_summary(
            &workspace,
        )?;

    let message = match checkpoint {
        Some(checkpoint) => {
            format_checkpoint_message(
                "Theseus Training Data",
                &checkpoint,
                None,
            )
        }

        None => {
            "No compatible Theseus checkpoint was found."
                .to_string()
        }
    };

    ctx.say(message).await?;

    Ok(())
}

async fn start_training_process(
    ctx: Context<'_>,
    mode: TrainingMode,
    limit: TrainingLimit,
) -> Result<(), Error> {
    let workspace = workspace_path();

    let (
        theseus_command,
        command_arguments,
        description,
    ) = training_command(mode, &limit);

    let training_id = ctx
        .data()
        .next_training_id
        .fetch_add(1, Ordering::Relaxed);

    fs::create_dir_all(
        workspace.join(
            TRAINING_LOG_DIRECTORY,
        ),
    )?;

    let stdout_path = workspace
        .join(TRAINING_LOG_DIRECTORY)
        .join(format!(
            "training_{training_id}_stdout.log"
        ));

    let stderr_path = workspace
        .join(TRAINING_LOG_DIRECTORY)
        .join(format!(
            "training_{training_id}_stderr.log"
        ));

    let stdout_file =
        File::create(&stdout_path)?;

    let stderr_file =
        File::create(&stderr_path)?;

    let mut training_guard =
        ctx.data().training.lock().await;

    refresh_finished_process(
        &mut training_guard,
    )?;

    if let Some(training) =
        training_guard.as_ref()
    {
        ctx.say(format!(
            "Theseus is already training.\n\
             Current session: {}\n\
             Elapsed: {}",
            training.description,
            format_duration(
                training.started_at.elapsed(),
            )
        ))
        .await?;

        return Ok(());
    }

    let child = Command::new(
        theseus_command,
    )
    .current_dir(&workspace)
    .args(command_arguments)
    .stdin(Stdio::null())
    .stdout(Stdio::from(stdout_file))
    .stderr(Stdio::from(stderr_file))
    .spawn()?;

    let channel_id =
        ctx.channel_id();

    *training_guard = Some(
        TrainingProcess {
            id: training_id,
            child,
            started_at: Instant::now(),
            description:
                description.clone(),
            channel_id,
        },
    );

    drop(training_guard);

    ctx.say(format!(
        "Theseus training started.\n\
         Mode: {}\n\
         Limit: {}\n\
         Progress updates will be sent every 10 minutes.",
        match mode {
            TrainingMode::New => "New model",
            TrainingMode::Resume => {
                "Resume checkpoint"
            }
        },
        limit_description(&limit),
    ))
    .await?;

    let http =
        ctx.serenity_context().http.clone();

    let shared_training =
        ctx.data().training.clone();

    tokio::spawn(async move {
        monitor_training(
            http,
            shared_training,
            workspace,
            training_id,
            stdout_path,
            stderr_path,
        )
        .await;
    });

    Ok(())
}

async fn monitor_training(
    http: Arc<serenity::Http>,
    shared_training:
        Arc<Mutex<Option<TrainingProcess>>>,
    workspace: PathBuf,
    training_id: u64,
    stdout_path: PathBuf,
    stderr_path: PathBuf,
) {
    let mut last_update =
        Instant::now();

    loop {
        tokio::time::sleep(
            Duration::from_secs(
                PROCESS_CHECK_SECONDS,
            ),
        )
        .await;

        let process_state = {
            let mut guard =
                shared_training.lock().await;

            let Some(training) =
                guard.as_mut()
            else {
                return;
            };

            if training.id != training_id {
                return;
            }

            match training.child.try_wait() {
                Ok(Some(status)) => {
                    let channel_id =
                        training.channel_id;

                    let elapsed =
                        training
                            .started_at
                            .elapsed();

                    *guard = None;

                    Some((
                        channel_id,
                        elapsed,
                        Some(status.success()),
                    ))
                }

                Ok(None) => {
                    Some((
                        training.channel_id,
                        training
                            .started_at
                            .elapsed(),
                        None,
                    ))
                }

                Err(error) => {
                    let channel_id =
                        training.channel_id;

                    *guard = None;

                    let message = format!(
                        "Theseus training monitor failed: \
                         {error}"
                    );

                    let _ = channel_id
                        .say(&http, message)
                        .await;

                    return;
                }
            }
        };

        let Some((
            channel_id,
            elapsed,
            completion,
        )) = process_state
        else {
            return;
        };

        if let Some(success) = completion {
            let checkpoint =
                load_newest_checkpoint_summary(
                    &workspace,
                )
                .ok()
                .flatten();

            let title = if success {
                "Theseus Training Finished"
            } else {
                "Theseus Training Failed"
            };

            let message =
                match checkpoint {
                    Some(checkpoint) => {
                        let mut message =
                            format_checkpoint_message(
                                title,
                                &checkpoint,
                                Some(elapsed),
                            );

                        if !success {
                            message.push_str(
                                "\n\nCheck the Hermes training logs:",
                            );
                            message.push_str(
                                &format!(
                                    "\n`{}`\n`{}`",
                                    stdout_path.display(),
                                    stderr_path.display(),
                                ),
                            );
                        }

                        message
                    }

                    None => {
                        format!(
                            "{title}\n\
                             Elapsed: {}\n\
                             No compatible checkpoint was found.\n\
                             Logs:\n`{}`\n`{}`",
                            format_duration(elapsed),
                            stdout_path.display(),
                            stderr_path.display(),
                        )
                    }
                };

            let _ =
                channel_id.say(
                    &http,
                    message,
                )
                .await;

            return;
        }

        if last_update.elapsed()
            >= Duration::from_secs(
                UPDATE_INTERVAL_SECONDS,
            )
        {
            let checkpoint =
                load_newest_checkpoint_summary(
                    &workspace,
                )
                .ok()
                .flatten();

            let message =
                match checkpoint {
                    Some(checkpoint) => {
                        format_checkpoint_message(
                            "Theseus 10-Minute Update",
                            &checkpoint,
                            Some(elapsed),
                        )
                    }

                    None => {
                        format!(
                            "Theseus 10-Minute Update\n\
                             State: Running\n\
                             Elapsed: {}\n\
                             No completed checkpoint is available yet.",
                            format_duration(elapsed),
                        )
                    }
                };

            let _ =
                channel_id.say(
                    &http,
                    message,
                )
                .await;

            last_update = Instant::now();
        }
    }
}

fn training_command(
    mode: TrainingMode,
    limit: &TrainingLimit,
) -> (
    &'static str,
    Vec<String>,
    String,
) {
    let command =
        match mode {
            TrainingMode::New => {
                "train-new"
            }
            TrainingMode::Resume => {
                "train-resume"
            }
        };

    let mut arguments = vec![
        "run".to_string(),
        "--release".to_string(),
        "-p".to_string(),
        "theseus".to_string(),
        "--".to_string(),
        command.to_string(),
    ];

    match limit {
        TrainingLimit::Mazes(mazes) => {
            arguments.push(
                "--mazes".to_string(),
            );
            arguments.push(
                mazes.to_string(),
            );
        }

        TrainingLimit::Hours(hours) => {
            arguments.push(
                "--hours".to_string(),
            );
            arguments.push(
                hours.to_string(),
            );
        }
    }

    let description = format!(
        "{} — {}",
        match mode {
            TrainingMode::New => {
                "New training"
            }
            TrainingMode::Resume => {
                "Resume training"
            }
        },
        limit_description(limit),
    );

    (
        "cargo",
        arguments,
        description,
    )
}

fn parse_training_limit(
    mazes: Option<i64>,
    hours: Option<f64>,
) -> Result<TrainingLimit, String> {
    match (mazes, hours) {
        (Some(mazes), None) => {
            if mazes <= 0 {
                return Err(
                    "Mazes must be greater than zero."
                        .to_string(),
                );
            }

            Ok(TrainingLimit::Mazes(
                mazes as u64,
            ))
        }

        (None, Some(hours)) => {
            if !hours.is_finite()
                || hours <= 0.0
            {
                return Err(
                    "Hours must be a finite number greater than zero."
                        .to_string(),
                );
            }

            Ok(TrainingLimit::Hours(hours))
        }

        (Some(_), Some(_)) => {
            Err(
                "Supply either `mazes` or `hours`, not both."
                    .to_string(),
            )
        }

        (None, None) => {
            Err(
                "Supply either a maze count or a number of hours."
                    .to_string(),
            )
        }
    }
}

fn refresh_finished_process(
    training:
        &mut Option<TrainingProcess>,
) -> Result<(), Error> {
    let Some(process) =
        training.as_mut()
    else {
        return Ok(());
    };

    if process.child.try_wait()?.is_some() {
        *training = None;
    }

    Ok(())
}

fn load_newest_checkpoint_summary(
    workspace: &Path,
) -> Result<Option<CheckpointSummary>, Error> {
    let checkpoint_directory =
        workspace.join(
            CHECKPOINT_DIRECTORY,
        );

    if !checkpoint_directory.exists() {
        return Ok(None);
    }

    let mut newest:
        Option<CheckpointSummary> = None;

    for slot in 1..=5 {
        let path =
            checkpoint_directory.join(
                format!(
                    "checkpoint_{slot}.json"
                ),
            );

        if !path.exists() {
            continue;
        }

        let json =
            match fs::read_to_string(
                &path,
            ) {
                Ok(json) => json,
                Err(_) => continue,
            };

        let value: Value =
            match serde_json::from_str(
                &json,
            ) {
                Ok(value) => value,
                Err(_) => continue,
            };

        let summary =
            match checkpoint_from_value(
                &value,
            ) {
                Some(summary) => summary,
                None => continue,
            };

        let replace = newest
            .as_ref()
            .map(|current| {
                summary.mazes_completed
                    > current.mazes_completed
            })
            .unwrap_or(true);

        if replace {
            newest = Some(summary);
        }
    }

    Ok(newest)
}

fn checkpoint_from_value(
    value: &Value,
) -> Option<CheckpointSummary> {
    Some(CheckpointSummary {
        mazes_completed:
            get_u128(
                value,
                "mazes_completed",
            )?,

        total_examples_trained:
            get_u128(
                value,
                "total_examples_trained",
            )?,

        latest_training_loss:
            value
                .get(
                    "latest_training_loss",
                )?
                .as_f64()?,

        latest_training_accuracy:
            value
                .get(
                    "latest_training_accuracy",
                )?
                .as_f64()?,

        theseus_solved:
            value
                .get(
                    "theseus_solved",
                )?
                .as_bool()?,

        theseus_nodes_explored:
            get_u128(
                value,
                "theseus_nodes_explored",
            )?,

        theseus_duration_nanos:
            get_u128(
                value,
                "theseus_duration_nanos",
            )?,

        theseus_path_length:
            get_u128(
                value,
                "theseus_path_length",
            )?,

        latest_teacher:
            value
                .get(
                    "latest_teacher",
                )?
                .as_str()?
                .to_string(),

        teacher_nodes_explored:
            get_u128(
                value,
                "teacher_nodes_explored",
            )?,

        teacher_duration_nanos:
            get_u128(
                value,
                "teacher_duration_nanos",
            )?,

        teacher_path_length:
            get_u128(
                value,
                "teacher_path_length",
            )?,

        maze_width:
            value
                .get("maze_width")?
                .as_u64()?,

        maze_height:
            value
                .get("maze_height")?
                .as_u64()?,
    })
}

fn get_u128(
    value: &Value,
    field: &str,
) -> Option<u128> {
    let field_value =
        value.get(field)?;

    if let Some(number) =
        field_value.as_u64()
    {
        return Some(number as u128);
    }

    field_value
        .as_str()?
        .parse::<u128>()
        .ok()
}

fn format_status_message(
    running:
        Option<(String, Duration)>,
    checkpoint:
        Option<&CheckpointSummary>,
) -> String {
    match (running, checkpoint) {
        (
            Some((description, elapsed)),
            Some(checkpoint),
        ) => {
            let mut message =
                format_checkpoint_message(
                    "Theseus Training Status",
                    checkpoint,
                    Some(elapsed),
                );

            message.push_str(
                &format!(
                    "\nSession: {description}"
                ),
            );

            message
        }

        (
            Some((description, elapsed)),
            None,
        ) => {
            format!(
                "Theseus Training Status\n\
                 State: Running\n\
                 Session: {description}\n\
                 Elapsed: {}\n\
                 No completed checkpoint is available yet.",
                format_duration(elapsed),
            )
        }

        (None, Some(checkpoint)) => {
            format_checkpoint_message(
                "Theseus Training Status",
                checkpoint,
                None,
            )
        }

        (None, None) => {
            "Theseus Training Status\n\
             State: Not running\n\
             No compatible checkpoint was found."
                .to_string()
        }
    }
}

fn format_checkpoint_message(
    title: &str,
    checkpoint: &CheckpointSummary,
    elapsed: Option<Duration>,
) -> String {
    let difference =
        checkpoint
            .theseus_nodes_explored
            as i128
            - checkpoint
                .teacher_nodes_explored
                as i128;

    let difficulty =
        difficulty_from_size(
            checkpoint.maze_width,
        );

    let state =
        if checkpoint.theseus_solved {
            "Solved"
        } else {
            "Failed"
        };

    let mut message = format!(
        "{title}\n\
         Mazes completed: {}\n\
         Examples trained: {}\n\
         Latest maze: {} ({}x{})\n\
         Theseus: {}\n\
         Theseus nodes: {}\n\
         Teacher: {}\n\
         Teacher nodes: {}\n\
         Node difference: {:+}\n\
         Loss: {:.6}\n\
         Accuracy: {:.2}%",
        checkpoint.mazes_completed,
        checkpoint.total_examples_trained,
        difficulty,
        checkpoint.maze_width,
        checkpoint.maze_height,
        state,
        checkpoint.theseus_nodes_explored,
        checkpoint.latest_teacher,
        checkpoint.teacher_nodes_explored,
        difference,
        checkpoint.latest_training_loss,
        checkpoint.latest_training_accuracy
            * 100.0,
    );

    if let Some(elapsed) = elapsed {
        message.push_str(
            &format!(
                "\nElapsed: {}",
                format_duration(elapsed),
            ),
        );
    }

    message
}

fn difficulty_from_size(
    size: u64,
) -> &'static str {
    match size {
        20..=49 => "Easy",
        50..=99 => "Medium",
        100..=249 => "Hard",
        _ => "Labyrinthian",
    }
}

fn limit_description(
    limit: &TrainingLimit,
) -> String {
    match limit {
        TrainingLimit::Mazes(mazes) => {
            format!("{mazes} maze(s)")
        }

        TrainingLimit::Hours(hours) => {
            format!("{hours} hour(s)")
        }
    }
}

fn format_duration(
    duration: Duration,
) -> String {
    let total_seconds =
        duration.as_secs();

    let hours =
        total_seconds / 3_600;

    let minutes =
        (total_seconds % 3_600)
            / 60;

    let seconds =
        total_seconds % 60;

    format!(
        "{hours:02}:{minutes:02}:{seconds:02}"
    )
}

fn workspace_path() -> PathBuf {
    PathBuf::from(
        env!("CARGO_MANIFEST_DIR"),
    )
    .parent()
    .expect(
        "Hermes crate should be inside the workspace",
    )
    .to_path_buf()
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let token =
        std::env::var("DISCORD_TOKEN")
            .expect(
                "Missing DISCORD_TOKEN in .env file",
            );

    let intents =
        serenity::GatewayIntents::non_privileged();

    let framework =
        poise::Framework::builder()
            .options(
                poise::FrameworkOptions {
                    commands: vec![
                        generate(),
                        training_start(),
                        training_resume(),
                        training_status(),
                        training_data(),
                    ],
                    ..Default::default()
                },
            )
            .setup(
                |ctx, _ready, framework| {
                    Box::pin(async move {
                        poise::builtins::register_globally(
                            ctx,
                            &framework
                                .options()
                                .commands,
                        )
                        .await?;

                        Ok(Data {
                            training:
                                Arc::new(
                                    Mutex::new(
                                        None,
                                    ),
                                ),

                            next_training_id:
                                Arc::new(
                                    AtomicU64::new(
                                        1,
                                    ),
                                ),
                        })
                    })
                },
            )
            .build();

    let mut client =
        serenity::ClientBuilder::new(
            token,
            intents,
        )
        .framework(framework)
        .await
        .expect(
            "Failed to create Discord client",
        );

    println!("Hermes is online.");

    client
        .start()
        .await
        .expect("Discord client error");
}