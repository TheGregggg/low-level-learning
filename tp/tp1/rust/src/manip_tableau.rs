use rand::distributions::{Distribution, Uniform};
use std::vec::Vec;

pub fn init_tab(nb_elem: usize) -> Vec<i32> {
    let range = Uniform::from(1..100);
    let mut rng = rand::thread_rng();
    let mut my_vec: Vec<i32> = Vec::with_capacity(nb_elem);

    for _ in 1..nb_elem {
        my_vec.push(range.sample(&mut rng));
    }
    my_vec
}

pub fn print_tab(tab: &Vec<i32>) {
    for v in tab {
        print!("{} ", v);
    }
    print!("\n");
}

pub fn copie_tab(tab1: &Vec<i32>, tab2: &mut Vec<i32>) {
    if tab2.capacity() < tab1.len() {
        tab2.resize(tab1.len(), 0);
    }
    tab2.clear();
    for v in tab1 {
        tab2.push(*v);
    }
}

pub fn recherche_sequentielle(tab: &Vec<i32>, val: i32) -> usize {
    for (i, item) in tab.iter().enumerate() {
        if *item == val {
            return i;
        }
    }
    return usize::MAX;
}

fn _recherche_dichotomique_recursif(
    tab: &Vec<i32>,
    valeur: i32,
    start: usize,
    end: usize,
) -> usize {
    let index = start + (end - start) / 2;
    let valeur_at_index = tab[index];
    if valeur_at_index == valeur {
        return index;
    } else if valeur_at_index < valeur {
        return _recherche_dichotomique_recursif(tab, valeur, index, end);
    } else {
        return _recherche_dichotomique_recursif(tab, valeur, start, index);
    }
}

pub fn recherche_dichotomique(tab: &Vec<i32>, valeur: i32) -> usize {
    return _recherche_dichotomique_recursif(tab, valeur, 0, tab.len());
}

pub fn tri_selection(tab: &mut Vec<i32>) {
    let mut index_ppe: usize;
    let mut temp: i32;

    for i in 0..tab.len() {
        index_ppe = i;
        for j in i + 1..tab.len() {
            if tab[j] < tab[index_ppe] {
                index_ppe = j;
            }
        }

        temp = tab[index_ppe];
        tab[index_ppe] = tab[i];
        tab[i] = temp;
    }
}
