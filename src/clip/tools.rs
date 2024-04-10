use indicatif::{ProgressBar, ProgressStyle};

pub fn create_bar(size: u64) -> ProgressBar{

    let bar = ProgressBar::new(size);
    // "[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({eta})"
    let style = ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{bar:50.cyan/blue}] {msg:<13} {pos}/{len} ({eta})",
    )
    .unwrap()
    .progress_chars("##>");

    bar.set_style(style);
    bar       
}