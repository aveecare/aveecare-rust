//! cargo run --example pagination  (with AVEECARE_API_KEY set)

use aveecare::futures_util::{pin_mut, StreamExt};
use aveecare::{AveeCare, PatientListParams};

#[tokio::main]
async fn main() -> Result<(), aveecare::Error> {
    let avee = AveeCare::from_env()?;

    // Auto-paging: a Stream of every patient, fetched one page at a time.
    let patients = avee
        .patients()
        .list_auto_paging(PatientListParams::new().limit(100));
    pin_mut!(patients);
    let mut count = 0;
    while let Some(patient) = patients.next().await {
        let patient = patient?;
        count += 1;
        if count <= 3 {
            println!("{}", patient.id);
        }
    }
    println!("{count} patients in total");

    // Manual paging: pass next_cursor back as the cursor.
    let mut page = avee
        .patients()
        .list(PatientListParams::new().limit(25))
        .await?;
    println!("first page: {}", page.data.len());
    while let Some(cursor) = page.next_cursor.clone().filter(|_| page.has_more) {
        page = avee
            .patients()
            .list(PatientListParams::new().limit(25).cursor(cursor))
            .await?;
        println!("next page: {}", page.data.len());
    }
    Ok(())
}
