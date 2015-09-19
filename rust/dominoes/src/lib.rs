pub type Domino = (usize, usize);

// we only need to find 1 path, not all paths => depth first search
// pick one domino d1
// if there are more, then try to chain from end of d1.
// if no matches, flip d1 around and try again.
// n, list, for each try front and then back.  if neither matches n, put in list of skipped and try
// with next.
// after match, get end of match, and pass along with remainder of list.

pub fn chain(input: &Vec<Domino>) -> Option<Vec<Domino>> {
    println!("input: {:?}", input);
    if input.len() == 0 {
        return Some(vec!())
    }
    if input.len() == 1 {
        if input[0].0 == input[0].1 {
            return Some(vec!(input[0]))
        } else {
            return None;
        }
    }
    for i in 0..input.len() {
        let domino = input[i];
        println!("dom: {:?}", domino);
        let mut rest = input.clone();
        rest.remove(i);
        let rest = rest;
        match subchain(domino.0, rest.clone()) {
            Some(cx) => {
                println!("dom0: {:?}, rest:{:?}, cx:{:?}", domino.0, rest, cx);
                let mut m = vec!(domino);
                m.extend(cx.iter().cloned() );
                return Some(m);
            },
            None => ()
        }
        match subchain(domino.1, rest.clone()) {
            Some(cx) => {
                let mut m = vec!((domino.1, domino.0));
                m.extend(cx.iter().cloned() );
                return Some(m);
            },
            None => ()
        }
    }
    None
}

pub fn match_head(head: usize, d: Domino) -> Option<Domino>{
    match d {
        (m, n) if m == head => Some((m, n)),
        (m, n) if n == head => Some((n, m)),
        _ => None
    }
}

pub fn subchain(head: usize, dx: Vec<Domino>) -> Option<Vec<Domino>> {
    for i in 0..dx.len() {
        match match_head(head, dx[i]) {
            Some(d1) => {
                let mut rest = dx.clone();
                rest.remove(i);
                match subchain(d1.1, rest) {
                    Some(cx) => {
                        let mut m = vec!(d1);
                        m.extend(cx.iter().cloned() );
                        return Some(m);
                    },
                    None => ()
                }
            },
            None => ()
        }
    }
    None
}
