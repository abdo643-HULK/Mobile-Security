#[derive(Debug, Clone)]
pub struct Plugboard {
    plugs: [char; 26],
}

impl Plugboard {
    fn new() -> Self {
        let mut plugs: [char; 26] = collect_array('A'..='Z');

        Self { plugs }
    }
}

fn collect_array<T, I, const N: usize>(itr: I) -> [T; N]
where
    T: Default + Copy,
    I: IntoIterator<Item = T>,
{
    let mut res = [T::default(); N];
    for (it, elem) in res.iter_mut().zip(itr) {
        *it = elem
    }
    res
}
