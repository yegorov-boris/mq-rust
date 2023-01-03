fn main() {
    println!("result {:?}", closest_pair(&[
        // (2.0, 2.0), (6.0, 3.0)
        (2.0, 2.0), (2.0, 8.0), (5.0, 9.0), (6.0, 3.0), (6.0, 7.0), (7.0, 4.0), (7.0, 9.0)
        // (2.0, 2.0), (2.0, 8.0), (5.0, 5.0), (5.0, 5.0), (6.0, 3.0), (6.0, 7.0), (7.0, 4.0), (7.0, 9.0)
        // (2.0, 2.0), (2.0, 8.0), (5.0, 5.0), (6.0, 3.0), (5.0, 5.0), (6.0, 7.0), (7.0, 4.0), (7.0, 9.0)
    ]));
}

#[derive(Debug)]
enum Index {
    Leaf(Option<(f64, f64)>),
    Node(f64, f64, Box<Index>, Box<Index>, Box<Index>, Box<Index>)
}

fn closest_pair(points: &[(f64, f64)]) -> ((f64, f64), (f64, f64)) {
    if points.len() == 2 {
        return (points[0], points[1]);
    }
    if points.len() == 3 {
        let (_, a, b) = get_closest_pair(&Box::from(Index::Leaf(None)), points);
        return (a, b);
    }
    let mut sorted_points = Vec::from(points);
    sorted_points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    for i in 1..sorted_points.len() {
        if sorted_points[i] == sorted_points[i-1] {
            return (sorted_points[i], sorted_points[i-1]);
        }
    }
    let mut xs: Vec<f64> = sorted_points.iter().map(|(x, _)| { *x }).collect();
    let mut ys: Vec<f64> = points.iter().map(|(_, y)| { *y }).collect();
    ys.sort_by(|a, b| a.partial_cmp(b).unwrap());
    xs.dedup();
    ys.dedup();
    let index: Index = make_index(&xs[..], &ys[..], &sorted_points);
    let (_, a, b) = get_closest_pair(&Box::from(index), &sorted_points[..]);
    (a, b)
}

fn get_closest_pair(idx: &Box<Index>, ps: &[(f64, f64)]) -> (f64, (f64, f64), (f64, f64)) {
    let n = ps.len();
    if n == 2 {
        return (dist(ps[0], ps[1]), ps[0], ps[1]);
    }
    if n == 3 {
        let mut d: f64 = dist(ps[0], ps[1]);
        let mut a: (f64, f64) = ps[0];
        let mut b: (f64, f64) = ps[1];
        let mut t: f64 = dist(ps[0], ps[2]);
        if t < d {
            d = t;
            b = ps[2];
        }
        t = dist(ps[1], ps[2]);
        if t < d {
            d = t;
            a = ps[1];
            b = ps[2];
        }
        return (d, a, b);
    }
    let x_min: f64 = ps[0].0;
    let x_max: f64 = ps[n-1].0;
    if x_min == x_max {
        let mut v: Vec<(f64, f64)> = Vec::from(ps);
        v.sort_by(|(_, ya), (_, yb)| ya.partial_cmp(yb).unwrap());
        let mut d = v[1].1 - v[0].1;
        let mut a = v[0];
        let mut b = v[1];
        for i in 2..n {
            let t = v[i].1 - v[i-1].1;
            if t < d {
                d = t;
                a = v[i-1];
                b = v[i];
            }
        }
        return (d, a, b);
    }
    let mx: f64 = (x_min + x_max)/2.0;
    let mut m: usize = 0;
    while ps[m + 1].0 <= mx {
        m += 1;
    }
    let border = ps[m+1].0 - ps[m].0;
    if m == 0 {
        let (dr, ar, br) = get_closest_pair(idx, &ps[m+1..]);
        let mut d = dr;
        let mut a = ar;
        let mut b = br;
        if border >= d {
            return (d, a, b);
        }
        for pr in search(idx, ps[m].0, ps[m].0 + d, ps[m].1 + d, ps[m].1 - d) {
            let t = dist(ps[m], pr);
            if t < d {
                d = t;
                a = ps[m];
                b = pr;
            }
        }
        return (d, a, b);
    }
    if m == n - 2 {
        let (dl, al, bl) = get_closest_pair(idx, &ps[..=m]);
        let mut d = dl;
        let mut a = al;
        let mut b = bl;
        if border >= d {
            return (d, a, b);
        }
        for pr in search(idx, ps[m+1].0 - d, ps[m+1].0, ps[m+1].1 + d, ps[m+1].1 - d) {
            let t = dist(ps[m+1], pr);
            if t < d {
                d = t;
                a = pr;
                b = ps[m+1];
            }
        }
        return (d, a, b);
    }
    let pl = &ps[..=m];
    let (dl, al, bl) = get_closest_pair(idx, pl);
    let (dr, ar, br) = get_closest_pair(idx, &ps[m+1..]);
    let mut d = dl;
    let mut a = al;
    let mut b = bl;
    if dr < dl {
        d = dr;
        a = ar;
        b = br;
    }
    if border >= d {
        return (d, a, b);
    }
    for p in pl {
        for pr in search(idx, p.0, p.0+d, p.1 + d, p.1 - d) {
            let t = dist(*p, pr);
            if t < d {
                d = t;
                a = *p;
                b = pr;
            }
        }
    }
    return (d, a, b);
}

fn dist(a: (f64, f64), b: (f64, f64)) -> f64 {
    f64::sqrt(f64::powi(a.0 - b.0, 2) + f64::powi(a.1 - b.1, 2))
}

fn make_index(xs: &[f64], ys: &[f64], ps: &Vec<(f64, f64)>) -> Index {
    match ps.len() {
        0 => Index::Leaf(None),
        1 => Index::Leaf(Some(ps[0])),
        _ => {
            let mx: usize = xs.len() / 2 - 1;
            let my: usize = ys.len() / 2 - 1;
            let mut tl: Vec<(f64, f64)> = Vec::new();
            let mut tr: Vec<(f64, f64)> = Vec::new();
            let mut bl: Vec<(f64, f64)> = Vec::new();
            let mut br: Vec<(f64, f64)> = Vec::new();
            for (x, y) in ps {
                if *x <= xs[mx] {
                    if *y <= ys[my] {
                        bl.push((*x, *y));
                    } else {
                        tl.push((*x, *y));
                    }
                } else {
                    if *y <= ys[my] {
                        br.push((*x, *y));
                    } else {
                        tr.push((*x, *y));
                    }
                }
            }
            Index::Node(
                xs[mx],
                ys[my],
                Box::from(make_index(&xs[..=mx], &ys[my+1..], &tl)),
                Box::from(make_index(&xs[mx+1..], &ys[my+1..], &tr)),
                Box::from(make_index(&xs[..=mx], &ys[..=my], &bl)),
                Box::from(make_index(&xs[mx+1..], &ys[..=my], &br)),
            )
        }
    }
}

fn search(idx: &Box<Index>, xl: f64, xr: f64, yt: f64, yb: f64) -> Vec<(f64, f64)> {
    match &**idx {
        Index::Leaf(None) => Vec::new(),
        Index::Leaf(Some(p)) => Vec::from([*p]),
        Index::Node(x, y, tl, tr, bl, br) => {
            let mut result= Vec::new();
            if xl <= *x {
                if yb <= *y {
                    let mut result_bl = search(&bl, xl, f64::min(xr, *x), f64::min(yt, *y), yb);
                    result.append(&mut result_bl);
                }
                if yt > *y {
                    let mut result_tl = search(&tl, xl, f64::min(xr, *x), yt, f64::max(yb, *y));
                    result.append(&mut result_tl);
                }
            }
            if xr > *x {
                if yb <= *y {
                    let mut result_br = search(&br, f64::max(xl, *x), xr, f64::min(yt, *y), yb);
                    result.append(&mut result_br);
                }
                if yt > *y {
                    let mut result_tr = search(&tr, f64::max(xl, *x), xr, yt, f64::max(yb, *y));
                    result.append(&mut result_tr);
                }
            }
            result
        }
    }
}
