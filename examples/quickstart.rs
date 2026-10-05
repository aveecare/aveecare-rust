//! cargo run --example quickstart  (with AVEECARE_API_KEY set)

use aveecare::{AveeCare, PatientListParams, PatientStatus};

#[tokio::main]
async fn main() -> Result<(), aveecare::Error> {
    let avee = AveeCare::new(std::env::var("AVEECARE_API_KEY").expect("set AVEECARE_API_KEY"));

    let me = avee.me().retrieve().await?;
    println!(
        "Connected as key \"{}\" for {}",
        me.name, me.organization.name
    );

    let page = avee
        .patients()
        .list(
            PatientListParams::new()
                .status(PatientStatus::Active)
                .limit(5),
        )
        .await?;
    for patient in page.data {
        println!(
            "{} {} {}",
            patient.id, patient.first_name, patient.last_name
        );
    }
    Ok(())
}
