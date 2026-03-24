mod countdown;
mod pagestructs;
mod woaupdate;

fn main() {
    println!("WizardOfArc updater!");
    match countdown::update() {
        Ok(_) => println!("Countdown Updated."),
        Err(err) => {
            println!("Countdown failed to update: {:?}", err);
            return;
        }
    }
    let new_content = woaupdate::commit_data_update("update_blogpost");
    println!("{}", new_content);
    let new_content2 = woaupdate::render_woa_templates();
    println!("{}", new_content2);
    println!("committing and pushing changes to WOA...");
    let new_content3 = woaupdate::commit_woa_update("commit changes to WOA after blog update");
    println!("{}", new_content3);
}
