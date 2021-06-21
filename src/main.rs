fn merge_and_count(a: Vec<u32>, b: Vec<u32>) -> (usize, Vec<u32>) {
    let len = a.len() + b.len();

    // Manter um vetor de resultado e um ponteiro para o próximo elemento
    let mut l = vec![0; len];
    let mut i = 0usize;

    // Manter um ponteiro atual em cada lista, inicializado para apontar para os elementos da frente
    let mut ai = 0usize;
    let mut bj = 0usize;

    let mut inversions = 0usize; // Mantenha uma variável contador para o número de inversões, inicializado em 0

    // Enquanto ambas as listas não estejam vazias:
    while ai < a.len() && bj < b.len() {
        // Sejam ai_elem e bj_elem os elementos apontados pelo ponteiro Atual
        let ai_elem = a[ai];
        let bj_elem = b[bj];

        if ai_elem <= bj_elem {
            l[i] = a[ai]; // Anexe o menor desses dois à lista de saída
            ai += 1; // Avança o ponteiro Atual na lista da qual o elemento menor foi selecionado.
        } else {
            // Se bj for o elemento menor, então
            l[i] = b[bj]; // Anexe o menor desses dois à lista de saída
            bj += 1; // Avança o ponteiro Atual na lista da qual o elemento menor foi selecionado.
            inversions += a.len() - ai; // Aumente a contagem pelo número de elementos restantes em A
        }
        i += 1;
    }

    // Quando uma lista estiver vazia, acrescente o restante da outra lista ao resultado
    let dst = &mut l[i..len];
    if ai < a.len() {
        dst.copy_from_slice(&a[ai..a.len()]);
    } else {
        dst.copy_from_slice(&b[bj..b.len()]);
    }

    // Retorne a Contagem e lista mesclada
    (inversions, l)
}

fn sort_and_count(l: &[u32]) -> (usize, Vec<u32>) {
    if l.len() == 1 { // Se a lista tiver um elemento, então
        return (0, vec![l[0]]); // não há inversões
    }

    let middle = l.len() / 2; // Divida a lista em duas metades:

    let a = &l[0..middle]; // A contém os primeiros [n / 2] elementos
    let b = &l[middle..l.len()]; // B contém os [n / 2] elementos restantes

    let (ra, a) = sort_and_count(a); // (𝑟𝐴, A) = Classificar e Contar (A)
    let (rb, b) = sort_and_count(b); // (𝑟𝐵, B) = Classificar e Contar (B)

    let (r, ml) = merge_and_count(a, b); // (r, L) = Mesclar e contar (A, B

    (r + ra + rb, ml) // Retorne r = 𝑟𝐴+ 𝑟𝐵+ r, e a lista classificada L
}

fn naive_count(a: &[u32]) -> usize {
    let mut res = 0usize;

    for i in 0..a.len() {
        let elem = a[i];
        for j in (i + 1)..a.len() {
            if elem > a[j] {
                res += 1;
            }
        }
    }

    res
}

fn main() {
    let a = vec![88, 70, 68, 34, 81, 74, 49, 86, 59, 37, 42, 5, 25, 15, 17, 80, 84, 82, 83, 72, 55, 79, 85, 69, 62, 58, 52, 96, 78, 10, 71, 89, 13, 76, 99, 31, 77, 9, 1, 87, 92, 41, 44, 100, 91, 24, 94, 19, 48, 63, 54, 97, 22, 39, 65, 98, 61, 90, 4, 56, 66, 7, 60, 29, 95, 28, 38, 50, 14, 45, 12, 51, 32, 2, 67, 33, 36, 64, 46, 40, 26, 21, 30, 43, 73, 16, 8, 23, 6, 18, 11, 93, 3, 20, 35, 47, 53, 27, 75, 57];
    println!("{:?}", naive_count(&a));
    println!("{:?}", sort_and_count(&a));
}

#[cfg(test)]
mod tests {
    use crate::{naive_count, sort_and_count};

    #[test]
    fn five_elems() {
        let a = vec![2, 4, 1, 3, 5];
        assert_eq!(3, naive_count(&a));
        assert_eq!((3, vec![1, 2, 3, 4, 5]), sort_and_count(&a));
    }

    #[test]
    fn hundred_elems() {
        let mut a = vec![88, 70, 68, 34, 81, 74, 49, 86, 59, 37, 42, 5, 25, 15, 17, 80, 84, 82, 83, 72, 55, 79, 85, 69, 62, 58, 52, 96, 78, 10, 71, 89, 13, 76, 99, 31, 77, 9, 1, 87, 92, 41, 44, 100, 91, 24, 94, 19, 48, 63, 54, 97, 22, 39, 65, 98, 61, 90, 4, 56, 66, 7, 60, 29, 95, 28, 38, 50, 14, 45, 12, 51, 32, 2, 67, 33, 36, 64, 46, 40, 26, 21, 30, 43, 73, 16, 8, 23, 6, 18, 11, 93, 3, 20, 35, 47, 53, 27, 75, 57];
        let (count, sorted) = sort_and_count(&a);
        assert_eq!(count, naive_count(&a));

        a.sort();
        assert_eq!(sorted, a);
    }

    #[test]
    fn two_hundred_plus_elems_some_duplicated() {
        let mut a = vec![88, 88, 88, 70, 68, 34, 81, 74, 49, 86, 59, 37, 42, 5, 25, 15, 17, 80, 84, 82, 83, 72, 55, 79, 85, 69, 62, 58, 52, 96, 78, 10, 71, 89, 13, 76, 99, 31, 77, 9, 1, 87, 92, 41, 44, 100, 91, 24, 94, 19, 48, 63, 54, 97, 22, 39, 65, 98, 61, 90, 4, 56, 66, 7, 60, 29, 95, 28, 38, 50, 14, 45, 12, 51, 32, 2, 67, 33, 36, 64, 46, 40, 26, 21, 30, 43, 73, 16, 8, 23, 6, 18, 11, 93, 3, 20, 35, 47, 53, 27, 75, 57, 41, 44, 100, 91, 24, 94, 19, 48, 63, 54, 97, 22, 39, 65, 98, 61, 90, 4, 56, 66, 7, 60, 3, 29, 95, 28, 38, 88, 50, 14, 45, 12, 51, 81, 51, 32, 88, 2, 11, 51, 81, 93, 3, 51, 44, 20, 35, 47, 53, 27, 75, 57, 88, 70, 68, 34, 81, 74, 49, 86, 59, 37, 42, 5, 25, 15, 17, 80, 84, 82, 83, 72, 55, 79, 85, 69, 62, 58, 52, 96, 78, 10, 71, 89, 13, 76, 99, 31, 77, 9, 1, 87, 92, 67, 33, 36, 64, 46, 40, 26, 21, 81, 30, 43, 73, 16, 8, 23, 6, 18, 81];
        let (count, sorted) = sort_and_count(&a);
        assert_eq!(count, naive_count(&a));

        a.sort();
        assert_eq!(sorted, a);
    }
}
