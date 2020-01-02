use clap::App;

pub fn init() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .get_matches();
}