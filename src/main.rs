mod initializer;

fn main() {
    initializer::init();
    puppeteer::run();
}
