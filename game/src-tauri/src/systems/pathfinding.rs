use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

/// A* pathfinding on a 2D grid (x, z).
/// Returns the path from start to goal (inclusive), or None if unreachable.
pub fn astar(
    start: (i32, i32),
    goal: (i32, i32),
    is_walkable: impl Fn(i32, i32) -> bool,
) -> Option<Vec<(i32, i32)>> {
    if start == goal { return Some(vec![start]); }

    let h = |p: (i32, i32)| (p.0 - goal.0).abs() + (p.1 - goal.1).abs();

    let mut open: BinaryHeap<(Reverse<i32>, (i32, i32))> = BinaryHeap::new();
    let mut g: HashMap<(i32, i32), i32> = HashMap::new();
    let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();

    g.insert(start, 0);
    open.push((Reverse(h(start)), start));

    while let Some((_, cur)) = open.pop() {
        if cur == goal {
            let mut path = vec![cur];
            let mut node = cur;
            while let Some(&prev) = came_from.get(&node) {
                path.push(prev);
                node = prev;
            }
            path.reverse();
            return Some(path);
        }

        let cur_g = *g.get(&cur).unwrap_or(&i32::MAX);

        for (dx, dz) in [(-1,0),(1,0),(0,-1),(0,1)] {
            let nb = (cur.0 + dx, cur.1 + dz);
            if !is_walkable(nb.0, nb.1) { continue; }
            let ng = cur_g + 1;
            if ng < *g.get(&nb).unwrap_or(&i32::MAX) {
                g.insert(nb, ng);
                came_from.insert(nb, cur);
                open.push((Reverse(ng + h(nb)), nb));
            }
        }
    }
    None
}
