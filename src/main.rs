mod CollectData;
use CollectData::collect_data::{self, collect_data};
use CollectData::datastructure::ProjDetails;

fn main() {
    //let proj = ProjDetails{name : String::from("test")} ;

    let mut proj_details: ProjDetails = ProjDetails::default();
    //Rust doesn't understand the expansion of ~, its s *nix thing
    //Absolute paths must
    collect_data(
        "/home/navin/github/homebrew-internethome/",
        &mut proj_details,
    );
    println!("{}",proj_details.name);
}
