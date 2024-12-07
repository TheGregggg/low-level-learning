use crate::manip_tableau::{copie_tab, init_tab, print_tab, tri_selection};

pub mod manip_tableau;

fn main() {
    let mut tab: Vec<i32> = init_tab(10);

    // let mut tab2: Vec<i32> = Vec::new();
    // copie_tab(&my_vec, &mut tab2);
    print_tab(&tab);

    tri_selection(&mut tab);
    print_tab(&tab);
}
