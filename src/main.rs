mod cli;
mod ddf;
mod list;

fn main() {
    ddf::read_ddf_target();
    ddf::read_ddf_list();
}
