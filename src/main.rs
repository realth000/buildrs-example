mod constants {
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/constants.generated.rs"));
}

use self::constants::*;

fn main() {
    println!("GIT_TAG_VERSION={GIT_TAG_VERSION}");
    println!("GIT_COMMIT_REVISION={GIT_COMMIT_REVISION}");
    println!("GIT_COMMIT_REVISION_LONG={GIT_COMMIT_REVISION_LONG}");
    println!("GIT_COMMIT_TIME={GIT_COMMIT_TIME}");
    println!("GIT_COMMIT_TIME_LONG={GIT_COMMIT_TIME_LONG}");
}
