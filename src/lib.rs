use std::collections::hash_map::{HashMap, Entry};
use std::collections::binary_heap::BinaryHeap;
use std::cmp::max;

type Pos = (usize,usize);
pub type Score = i32;

const INDEL: Score = -1;
const MATCH: Score = 0;
const MISMATCH: Score = -1;

fn diag(pos: &Pos) -> i32 {
    (pos.1 as i32) - (pos.0 as i32)
}

// find the distance from the diagonal of `goal` times INDEL
fn heuristic(pos: &Pos, goal: &Pos) -> Score {
    let goal_diag = diag(goal);
    let our_diag = diag(pos);
    let indel_dist = (our_diag - goal_diag).abs();
    // support non-zero MATCH cost
    let total_dist = max(goal.0 - pos.0, goal.1 - pos.1) as i32;
    let match_dist = total_dist - indel_dist;
    return (indel_dist * INDEL) + (match_dist * MATCH);
}

fn add_state(scores: &mut HashMap<Pos,Score>, q: &mut BinaryHeap<(Score,Pos)>, goal: &Pos, new_score: Score, pos: Pos) {
    match scores.entry(pos) {
        Entry::Vacant(e) => { e.insert(new_score); },
        Entry::Occupied(ref mut e) => {
            if *(e.get()) >= new_score { return; } // obsolete
            e.insert(new_score);
        }
    }

    let priority = new_score + heuristic(&pos, goal);
    q.push((priority, pos));
}

#[allow(dead_code)]
fn debug_print_table(scores: &HashMap<Pos,Score>, size: Pos) {
    let (am, bm) = size;
    for ai in 0..am {
        for bi in 0..bm {
            let inside = scores.contains_key(&(ai,bi));
            let chr = if inside { 'X' } else { ' ' };
            print!("{}", chr);
        }
        println!("");
    }
}

pub fn alignment_score(a: &[u8], b: &[u8]) -> Score {
    let start = (0,0);
    let goal = (a.len(),b.len());
    let mut q = BinaryHeap::with_capacity(a.len()+b.len());
    q.push((heuristic(&start, &goal), start));

    let mut scores: HashMap<Pos,Score> = HashMap::with_capacity(a.len()+b.len());
    scores.insert(start,0);

    let mut pop_count: usize = 0;
    let mut eval_count: usize = 0;
    loop {
        let (score, pos) = q.pop().expect("should always reach goal");
        let score = score - heuristic(&pos, &goal);
        pop_count += 1;

        if pos == goal {
            println!("scores: {:?}", scores.len());
            println!("pops: {:?}", pop_count);
            println!("evals: {:?}", eval_count);
            // debug_print_table(&scores, goal.clone());
            return score; // found it!
        }

        // this should be the best score we've found so far for this position
        let best_score = scores[&pos];
        if score > best_score {
            panic!("loop in DAG scores: {} < {}", score, best_score);
        } else if score < best_score {
            continue; // we've been here before, but better
        }

        eval_count += 1;

        // explore successor states
        let (ai,bi) = pos;

        // indel
        if ai < a.len() {
            add_state(&mut scores, &mut q, &goal, score + INDEL, (ai+1, bi));
        }
        if bi < b.len() {
            add_state(&mut scores, &mut q, &goal, score + INDEL, (ai, bi+1));
        }
        if ai < a.len() && bi < b.len() {
            let diag_score = if a[ai] == b[bi] {
                MATCH
            } else {
                MISMATCH
            };
            add_state(&mut scores, &mut q, &goal, score + diag_score, (ai+1, bi+1));
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let a = "GCATGCU";
        let b = "GATTACA";
        let score = alignment_score(a.as_bytes(), b.as_bytes());
        assert_eq!(-4, score);
    }

    #[test]
    fn heuristic_1() {
        let tests = vec![
            ((0,0), (4,4), 0),
            ((4,3), (4,7), -4),
            ((1,0), (4,4), -1),
            ((1,2), (4,4), -1),
        ];
        for (pos, goal, correct) in tests.into_iter() {
            let goal_diag = diag(&goal);
            assert_eq!(correct, heuristic(&pos, goal_diag));
        }
    }
}
