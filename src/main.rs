use color_eyre::Result;
use sub0_tangle_blueprint_challenges as blueprint;
use gadget_sdk as sdk;
use gadget_sdk::job_runner::MultiJobRunner;
use sdk::tangle_subxt::*;

#[sdk::main(env)]
async fn main() -> Result<()> {
    init_logger();
    let signer = env.first_sr25519_signer()?;
    let client = subxt::OnlineClient::from_url(&env.rpc_endpoint).await?;

    if env.should_run_registration() {
        todo!();
    }

    let service_id = env.service_id.expect("should exist");

    // Create your service context
    // Here you can pass any configuration or context that your service needs.
    let context = blueprint::ServiceContext {
        config: env.clone(),
    };

    // Create the event handler from the job
    let say_hello_job = blueprint::SayHelloEventHandler {
        service_id,
        client,
        signer,
        context,
    };

    tracing::info!("Starting the event watcher ...");
    MultiJobRunner::new(env).job(say_hello_job).run().await?;

    tracing::info!("Exiting...");
    Ok(())
}

fn init_logger() {
    let env_filter = tracing_subscriber::EnvFilter::from_default_env();
    tracing_subscriber::fmt()
        .compact()
        .with_target(true)
        .with_env_filter(env_filter)
        .init();
}
