use std::cell::RefCell;
use std::cmp::Reverse;
use std::fs;
use std::rc::Rc;

// leave enough space to the left and right for all offsprings to grow
const MARGIN: usize = 306;

const GRID_HEIGHT: usize = 102;

#[derive(Clone, Copy)]
struct Rule {
    right: Option<u8>,
    left: Option<u8>,
    top: Option<u8>,
}

#[derive(Clone)]
struct Part {
    x: usize,
    y: usize,
    tree: usize,
    tpe: u8,
    stem: bool,
    rules: Rc<Vec<Option<Rule>>>,
}

struct Tree {
    alive: bool,
    stem: Vec<Rc<RefCell<Part>>>,
    sprouts: Vec<Rc<RefCell<Part>>>,
}

impl Tree {
    fn deep_clone(&self) -> Self {
        Self {
            alive: self.alive,
            stem: self
                .stem
                .iter()
                .map(|s| Rc::new(RefCell::new(s.borrow().clone())))
                .collect(),
            sprouts: self
                .sprouts
                .iter()
                .map(|s| Rc::new(RefCell::new(s.borrow().clone())))
                .collect(),
        }
    }

    fn grow(&mut self, grid: &mut Grid) {
        let sprouts_to_process = self.sprouts.drain(..).collect::<Vec<_>>();

        // convert all sprouts to stem
        for s in &sprouts_to_process {
            s.borrow_mut().stem = true;
        }

        for s in &sprouts_to_process {
            let s = s.borrow();
            self.try_set(
                s.rules[s.tpe as usize].unwrap().left,
                s.x - 1,
                s.y,
                s.tree,
                &s.rules,
                grid,
            );
            self.try_set(
                s.rules[s.tpe as usize].unwrap().right,
                s.x + 1,
                s.y,
                s.tree,
                &s.rules,
                grid,
            );
            self.try_set(
                s.rules[s.tpe as usize].unwrap().top,
                s.x,
                s.y + 1,
                s.tree,
                &s.rules,
                grid,
            );
        }

        self.stem.extend(sprouts_to_process);

        // filter out sprouts that were overwritten during the growing process
        self.sprouts.retain(|s| s.borrow().tree != usize::MAX);
    }

    fn try_set(
        &mut self,
        tpe: Option<u8>,
        x: usize,
        y: usize,
        tree_id: usize,
        rules: &Rc<Vec<Option<Rule>>>,
        grid: &mut Grid,
    ) {
        let Some(tpe) = tpe else { return };

        let can_place = grid
            .get(x, y)
            .map(|p| {
                let p = p.borrow();
                p.tree == tree_id && !p.stem && p.tpe < tpe
            })
            .unwrap_or(true);

        if can_place {
            if let Some(existing) = grid.get(x, y) {
                // if we overwrite a sprout make it invalid - we will filter it
                // out at the end of the growing process
                existing.borrow_mut().tree = usize::MAX;
            }

            let p = Rc::new(RefCell::new(Part {
                x,
                y,
                tree: tree_id,
                tpe,
                stem: false,
                rules: Rc::clone(rules),
            }));
            grid.set(Rc::clone(&p));
            self.sprouts.push(p);
        }
    }

    fn required_energy(&self) -> usize {
        self.mass() * 3
    }

    fn produced_energy(&self, grid: &Grid) -> usize {
        let mut produced_energy = 0;

        for s in &self.stem {
            let s = s.borrow();
            let height = s.y.min(10);
            let mut factor = 3;
            for y in s.y + 1..=grid.max_y {
                if let Some(other) = grid.get(s.x, y)
                    && other.borrow().stem
                {
                    factor -= 1;
                    if factor == 0 {
                        break;
                    }
                }
            }
            produced_energy += height * factor;
        }

        produced_energy
    }

    fn mass(&self) -> usize {
        self.stem.len() + self.sprouts.len()
    }
}

struct Grid {
    inner: Vec<Option<Rc<RefCell<Part>>>>,
    width: usize,
    max_y: usize,
}

impl Grid {
    fn new(width: usize) -> Self {
        Self {
            inner: vec![None; width * GRID_HEIGHT],
            width,
            max_y: 0,
        }
    }

    fn set(&mut self, value: Rc<RefCell<Part>>) {
        let x = value.borrow().x;
        let y = value.borrow().y;
        self.inner[y * self.width + x] = Some(value);
        self.max_y = self.max_y.max(y);
    }

    fn get(&self, x: usize, y: usize) -> Option<Rc<RefCell<Part>>> {
        self.inner[y * self.width + x].as_ref().map(Rc::clone)
    }

    fn clear(&mut self) {
        self.inner.fill(None);
    }

    fn contains(&self, x: usize, y: usize) -> bool {
        self.inner[y * self.width + x].is_some()
    }
}

fn simulate(mut trees: Vec<Tree>, grid: &mut Grid, rounds: usize) -> usize {
    let mut result = 0;

    grid.clear();
    for tree in &trees {
        grid.set(Rc::clone(&tree.sprouts[0]));
    }

    for _ in 0..rounds {
        let mut dead_trees = 0;

        for year in 1..=100 {
            // let alive trees grow
            for tree in &mut trees {
                if tree.alive {
                    tree.grow(grid);
                }
            }

            if year >= 5 {
                for tree in trees.iter_mut() {
                    if !tree.alive {
                        continue;
                    }

                    let required_energy = tree.required_energy();
                    let produced_energy = tree.produced_energy(grid);

                    if required_energy > produced_energy {
                        tree.alive = false;
                        dead_trees += 1;
                    }
                }

                if dead_trees == trees.len() {
                    break;
                }
            }
        }

        result = trees.iter().map(|t| t.mass()).sum::<usize>();

        // convert sprouts to new trees and place them on the floor
        let mut sprouts = trees
            .into_iter()
            .flat_map(|t| t.sprouts)
            .collect::<Vec<_>>();
        // sort by x direction (so left-most trees are processed first) and by
        // decreasing y direction (so top-most sprouts are planted first)
        sprouts.sort_unstable_by_key(|s| (s.borrow().x, Reverse(s.borrow().y)));
        trees = Vec::new();
        grid.clear();
        for (i, s) in sprouts.into_iter().enumerate() {
            // don't overwrite top-most sprout
            if !grid.contains(s.borrow().x, 1) {
                let new_s = Rc::new(RefCell::new(Part {
                    x: s.borrow().x,
                    y: 1,
                    tree: i,
                    tpe: 0,
                    stem: false,
                    rules: Rc::clone(&s.borrow().rules),
                }));
                grid.set(Rc::clone(&new_s));
                trees.push(Tree {
                    alive: true,
                    stem: Vec::new(),
                    sprouts: vec![new_s],
                });
            }
        }
    }

    result
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not read file");
    let blocks = input.split("\n\n").collect::<Vec<_>>();

    // parse trees
    let mut orig_trees = Vec::new();
    for (i, block) in blocks.into_iter().enumerate() {
        let (row1, row2) = block.split_once("\n").unwrap();
        let row1 = row1.split_ascii_whitespace().collect::<Vec<_>>();
        let row2 = row2.split_ascii_whitespace().collect::<Vec<_>>();
        let row2 = row2.chunks_exact(3).collect::<Vec<_>>();
        let mut rules = vec![None; 100];
        for rule in row1.into_iter().zip(row2) {
            let from = rule.1[1].parse::<usize>().unwrap();
            let right = rule.1[2].parse::<u8>().ok();
            let left = rule.1[0].parse::<u8>().ok();
            let top = rule.0.parse::<u8>().ok();
            let rule = Rule { right, left, top };
            rules[from] = Some(rule);
        }
        orig_trees.push(Tree {
            alive: true,
            stem: Vec::new(),
            sprouts: vec![Rc::new(RefCell::new(Part {
                x: MARGIN + 10 * i,
                y: 1,
                tree: i,
                tpe: 0,
                stem: false,
                rules: Rc::new(rules),
            }))],
        });
    }

    let mut grid = Grid::new(MARGIN + orig_trees.len() * 10 + MARGIN);

    // part 1
    let trees = orig_trees.iter().map(|t| t.deep_clone());
    let mut total = 0;
    for tree in trees {
        total += simulate(vec![tree], &mut grid, 1);
    }
    println!("{total}");

    // part 2
    let trees = orig_trees
        .iter()
        .map(|t| t.deep_clone())
        .collect::<Vec<_>>();
    println!("{}", simulate(trees, &mut grid, 1));

    // part 3
    let trees = orig_trees
        .iter()
        .map(|t| t.deep_clone())
        .collect::<Vec<_>>();
    println!("{}", simulate(trees, &mut grid, 3));
}
