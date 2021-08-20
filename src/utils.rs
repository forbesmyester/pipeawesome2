pub fn remove_vec_vec_index<X>(vals: &mut Vec<Vec<X>>, index_0: usize, index_1: usize) {
    if let Some(v) = vals.get_mut(index_0) {
        if index_1 < v.len() {
            v.remove(index_1);
        }
    }
}

pub fn remove_vec_vec<X>(vals: &mut Vec<Vec<X>>) {
    for i in (0..vals.len()).rev() {
        let mut inner_len = 99;
        if let Some(iv) = vals.get(i) {
            inner_len = iv.len();
        }
        if inner_len == 0 {
            vals.remove(i);
        }
    }
}

pub fn take_bytes(buf: &[u8], n: usize) -> Vec<u8> {
    buf.split_at(n).0.iter().copied().collect()
}


#[test]
fn test_remove_vec_vec_index() {
    let mut vals = vec![
        vec![0, 1, 2],
        vec![3, 4, 5],
        vec![6, 7, 8],
    ];
    remove_vec_vec_index(&mut vals, 3, 1);
    assert_eq!(
        vals,
        vec![
            vec![0, 1, 2],
            vec![3, 4, 5],
            vec![6, 7, 8],
        ]
    );
    remove_vec_vec_index(&mut vals, 2, 3);
    assert_eq!(
        vals,
        vec![
            vec![0, 1, 2],
            vec![3, 4, 5],
            vec![6, 7, 8],
        ]
    );
    remove_vec_vec_index(&mut vals, 1, 1);
    assert_eq!(
        vals,
        vec![
            vec![0, 1, 2],
            vec![3, 5],
            vec![6, 7, 8],
        ]
    );
    remove_vec_vec_index(&mut vals, 0, 1);
    assert_eq!(
        vals,
        vec![
            vec![0, 2],
            vec![3, 5],
            vec![6, 7, 8],
        ]
    );
    remove_vec_vec_index(&mut vals, 2, 1);
    assert_eq!(
        vals,
        vec![
            vec![0, 2],
            vec![3, 5],
            vec![6, 8],
        ]
    );
    remove_vec_vec_index(&mut vals, 0, 0);
    assert_eq!(
        vals,
        vec![
            vec![2],
            vec![3, 5],
            vec![6, 8],
        ]
    );
    remove_vec_vec_index(&mut vals, 2, 1);
    assert_eq!(
        vals,
        vec![
            vec![2],
            vec![3, 5],
            vec![6],
        ]
    );
}


#[test]
fn test_remove_vec_vec() {
    let mut vals_0 = vec![
        vec![0, 1, 2],
        vec![3, 4, 5],
        vec![6, 7, 8],
    ];
    remove_vec_vec(&mut vals_0);
    assert_eq!(
        vals_0,
        vec![
            vec![0, 1, 2],
            vec![3, 4, 5],
            vec![6, 7, 8],
        ]
    );
    let mut vals_1 = vec![
        vec![0, 1, 2],
        vec![],
        vec![6, 7, 8],
    ];
    remove_vec_vec(&mut vals_1);
    assert_eq!(
        vals_1,
        vec![
            vec![0, 1, 2],
            vec![6, 7, 8],
        ]
    );
    let mut vals_2 = vec![
        vec![],
        vec![3, 4, 5],
        vec![6, 7, 8],
    ];
    remove_vec_vec(&mut vals_2);
    assert_eq!(
        vals_2,
        vec![
            vec![3, 4, 5],
            vec![6, 7, 8],
        ]
    );
    let mut vals_3 = vec![
        vec![],
        vec![3, 4, 5],
        vec![],
    ];
    remove_vec_vec(&mut vals_3);
    assert_eq!(
        vals_3,
        vec![
            vec![3, 4, 5],
        ]
    );
    let mut vals_4: Vec<Vec<usize>> = vec![
        vec![],
        vec![],
        vec![],
    ];
    remove_vec_vec(&mut vals_4);
    assert_eq!(vals_4.len(), 0);
}
