/// We can use the 'pub' keyword to mark instrument as
/// public for it to be used.
pub mod instrument {
    pub fn clarinet() {
        // Super calls from the parent crate, similar to
        // "../" in relative folder path terms.
        // This enables clarinet to call breath_in() from
        // mod sound.
        super::breath_in();
        println!("Playing the Clarinet")
    }
}

fn breath_in() {
    println!("Breathing in!")
}
