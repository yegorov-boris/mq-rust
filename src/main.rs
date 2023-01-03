fn main() {
    println!("result {:?}", closest_pair(&[
        // (2.0, 2.0), (6.0, 3.0)
        (2.0, 2.0), (2.0, 8.0), (5.0, 9.0), (6.0, 3.0), (6.0, 7.0), (7.0, 4.0), (7.0, 9.0)
        // (2.0, 2.0), (2.0, 8.0), (5.0, 5.0), (5.0, 5.0), (6.0, 3.0), (6.0, 7.0), (7.0, 4.0), (7.0, 9.0)
        // (2.0, 2.0), (2.0, 8.0), (5.0, 5.0), (6.0, 3.0), (5.0, 5.0), (6.0, 7.0), (7.0, 4.0), (7.0, 9.0)
    ]));
}

fn closest_pair(points: &[(f64, f64)]) -> ((f64, f64), (f64, f64)) {
    if points.len() == 2 {
        return (points[0], points[1]);
    }
    if points.len() == 3 {
        let (_, a, b) = get_closest_pair(points);
        return (a, b);
    }
    let mut sorted_points = Vec::from(points);
    sorted_points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    for i in 1..sorted_points.len() {
        if sorted_points[i] == sorted_points[i-1] {
            return (sorted_points[i], sorted_points[i-1]);
        }
    }
    let (_, a, b) = get_closest_pair(&sorted_points[..]);
    (a, b)
}

fn get_closest_pair(ps: &[(f64, f64)]) -> (f64, (f64, f64), (f64, f64)) {
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
    let pl = &ps[..=m];
    let pr = &ps[m+1..];
    let mut d: Option<f64> = None;
    let mut a: Option<(f64, f64)> = None;
    let mut b: Option<(f64, f64)> = None;
    if pl.len() >= 2 {
        let (dl, al, bl) = get_closest_pair(pl);
        d = Some(dl);
        a = Some(al);
        b = Some(bl);
    }
    if pr.len() >= 2 {
        let (dr, ar, br) = get_closest_pair(pr);
        match d {
            None => {
                d = Some(dr);
                a = Some(ar);
                b = Some(br);
            },
            Some(dl) => {
                if dr < dl {
                    d = Some(dr);
                    a = Some(ar);
                    b = Some(br);
                }
            }
        };
    }
    let mut d_min = d.unwrap();
    let mut a_min = a.unwrap();
    let mut b_min = b.unwrap();
    if border >= d_min {
        return (d_min, a_min, b_min);
    }
    for p in pl {
        for candidate in pr {
            if candidate.0 - p.0 < d_min {
                if f64::abs(candidate.1 - p.1) < d_min {
                    let t = dist(*p, *candidate);
                    if t < d_min {
                        d_min = t;
                        a_min = *p;
                        b_min = *candidate;
                    }
                }
            } else {
                break;
            }
        }
    }
    return (d_min, a_min, b_min);
}

fn dist(a: (f64, f64), b: (f64, f64)) -> f64 {
    f64::sqrt(f64::powi(a.0 - b.0, 2) + f64::powi(a.1 - b.1, 2))
}
