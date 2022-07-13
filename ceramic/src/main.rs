mod gpu;

const APPS: &[&str] = &["test", "foo", "bar"];

fn main() {
    gpu::clear();
    gpu::str_update("test", 1, 2);
    gpu::render();
}

fn home() {

}