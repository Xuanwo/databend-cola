use common_config::InnerConfig;

fn main() {
    let conf: InnerConfig = InnerConfig::load().unwrap();
    println!("{:?}", conf);
}
