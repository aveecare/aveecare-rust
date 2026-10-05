//! cargo run --example blocking --features blocking  (with AVEECARE_API_KEY set)

use aveecare::blocking::AveeCare;
use aveecare::PatientListParams;

fn main() -> Result<(), aveecare::Error> {
    let avee = AveeCare::from_env()?;
    for patient in avee
        .patients()
        .list_auto_paging(PatientListParams::new().limit(100))
    {
        println!("{}", patient?.id);
    }
    Ok(())
}
