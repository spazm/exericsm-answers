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
    let mut soln = None;
    'dominoes: for i in 0..input.len() {
        let domino = input[i];
        println!("dom: {:?}", domino);
        let mut rest = input.clone();
        rest.remove(i);
        let rest = rest;
        println!("dom0: {:?}, rest:{:?}", domino.0, rest);
        match subchain(domino.1, rest.clone(), domino.0) {
            Some(cx) => {
                let mut m = vec!(domino);
                m.extend(cx.iter().cloned() );
                println!("dom1: {:?}, rest:{:?}, cx:{:?} -> m:{:?}", domino.1, rest, cx, m);
                // validate that solution head == tail
                soln = Some(m);
                break 'dominoes;
            },
            None => ()
        }
        match subchain(domino.0, rest.clone(), domino.1) {
            Some(cx) => {
                println!("dom0: {:?}, rest:{:?}, cx:{:?}", domino.0, rest, cx);
                let mut m = vec!((domino.1, domino.0));
                m.extend(cx.iter().cloned() );
                println!("dom1: {:?}, rest:{:?}, cx:{:?} -> m:{:?}", domino.1, rest, cx, m);
                // validate that solution head == tail
                soln = Some(m);
                break 'dominoes;
            },
            None => ()
        }
    }
    soln
}

pub fn match_head(head: usize, d: Domino) -> Option<Domino>{
    match d {
        (m, n) if m == head => Some((m, n)),
        (m, n) if n == head => Some((n, m)),
        _ => None
    }
}

/// subchain returns an ordering of dx that starts with head and ends with tail.
pub fn subchain(head: usize, dx: Vec<Domino>, tail: usize) -> Option<Vec<Domino>> {
    for i in 0..dx.len() {
        println!("i:{}, head:{}, tail:{}, dx[i]:{:?}", i, head, tail, dx[i]);
        match match_head(head, dx[i]) {
            Some(d1) => {
                let mut rest = dx.clone();
                rest.remove(i);
                println!("matched d1:{:?} rest:{:?}", d1, rest);
                if rest.len() == 0 {
                    return if d1.1 == tail { Some(vec!(d1)) } else { None }
                }
                match subchain(d1.1, rest, tail) {
                    Some(cx) => {
                        let mut m = vec!(d1);
                        m.extend(cx.iter().cloned() );
                        return Some(m);
                    },
                    None => ()
                }
            },
            None => println!("match_head returned None for head:{}, dx[i]:{:?}", head, dx[i])
        }
    }
    None
}
