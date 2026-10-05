//! cargo run --example quickstart  (with AVEECARE_API_KEY set)

use aveecare::{AveeCare, PatientLifecycleStage, PatientListParams};

#[tokio::main]
async fn main() -> Result<(), aveecare::Error> {
    let avee = AveeCare::new(std::env::var("AVEECARE_API_KEY").expect("set AVEECARE_API_KEY"));

    let me = avee.me().retrieve().await?;
    println!(
        "Connected as key \"{}\" for {}",
        me.name.as_deref().unwrap_or("(unnamed)"),
        me.organization.name.as_deref().unwrap_or("your agency")
    );

    let page = avee
        .patients()
        .list(
            PatientListParams::new()
                .lifecycle_stage(PatientLifecycleStage::Client)
                .limit(5),
        )
        .await?;
    for patient in page.data {
        println!(
            "{} {} {}",
            patient.id,
            patient.first_name.as_deref().unwrap_or(""),
            patient.last_name.as_deref().unwrap_or("")
        );
    }
    Ok(())
}
