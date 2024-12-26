use cursive::views::{Dialog, TextView};
use cursive::{Cursive, CursiveExt};
use std::process::Command;

fn main() {
    // create a new cursive context
    let mut siv = Cursive::default();

    let output = Command::new("cmd")
        .args(&["/c", "dir"])
        .output()
        .expect("failed to execute process");

    let out_str = String::from_utf8_lossy(&output.stdout);

    siv.add_layer(
        Dialog::around(TextView::new(out_str.to_string()))
            .title("Output")
            .button("Close", |s| s.quit()),
    );
    siv.run();
}
