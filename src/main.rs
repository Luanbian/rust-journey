mod features;

#[tokio::main]
async fn main() {
    features::sort::main();
    features::guessing_game::main();
    features::syncronizing::main();
    features::dependency_injection::main().await;
}
