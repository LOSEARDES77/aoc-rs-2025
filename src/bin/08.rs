advent_of_code::solution!(8);

#[derive(Clone, Copy)]
struct JunctionBox {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Clone, Copy)]
struct Edge {
    jb1: usize, // Index of jb1
    jb2: usize, // Index of jb2
    distance: f32,
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }
        self.parent[x] = self.find(self.parent[x]);
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);

        if ra == rb {
            return false;
        }

        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }

        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        true
    }
}

impl Edge {
    fn calc_edge(boxes: &[JunctionBox], indexes: (usize, usize)) -> Self {
        Edge {
            jb1: indexes.0,
            jb2: indexes.1,
            distance: Self::calc_distance(&boxes[indexes.0], &boxes[indexes.1]),
        }
    }

    fn calc_distance(jb1: &JunctionBox, jb2: &JunctionBox) -> f32 {
        let dx = jb1.x as isize - jb2.x as isize;
        let dy = jb1.y as isize - jb2.y as isize;
        let dz = jb1.z as isize - jb2.z as isize;

        ((dx * dx + dy * dy + dz * dz) as f32).sqrt()
    }
}

pub fn split_in_3<'a1>(s: &'a1 str, del: &str) -> (&'a1 str, &'a1 str, &'a1 str) {
    let (p1, rest) = s.split_once(del).unwrap();
    let (p2, p3) = rest.split_once(del).unwrap();

    (p1, p2, p3)
}
impl JunctionBox {
    fn parse(s: &str) -> Self {
        let (x, y, z) = split_in_3(s, ",");

        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
            z: z.parse().unwrap(),
        }
    }
}

pub fn part_one_with_take(input: &str, take: usize) -> Option<u64> {
    let mut jbs = Vec::new();
    for l in input.lines() {
        jbs.push(JunctionBox::parse(l));
    }

    let mut edges = Vec::new();
    for i in 0..jbs.len() {
        for j in i + 1..jbs.len() {
            if i == j {
                continue;
            }
            edges.push(Edge::calc_edge(&jbs, (i, j)));
        }
    }

    edges.sort_by(|a, b| a.distance.total_cmp(&b.distance));

    // Union-Find
    let mut uf = UnionFind::new(jbs.len());
    for edge in edges.iter().take(take) {
        uf.union(edge.jb1, edge.jb2);
    }

    // Obtener tamaños finales de los componentes
    let mut groups = Vec::new();

    // Comprimir todos los nodos para asegurar leaders correctos
    for i in 0..jbs.len() {
        uf.find(i);
    }

    // Recoger tamaños únicos de líderes
    for i in 0..jbs.len() {
        if uf.parent[i] == i {
            groups.push(uf.size[i]);
        }
    }

    // Ordenarlos por tamaño descendente
    groups.sort_unstable_by(|a, b| b.cmp(a));

    if groups.len() < 3 {
        return None;
    }

    let result = groups[0] as u64 * groups[1] as u64 * groups[2] as u64;

    Some(result)
}

pub fn part_one(input: &str) -> Option<u64> {
    part_one_with_take(input, 1000)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut jbs = Vec::new();
    for l in input.lines() {
        jbs.push(JunctionBox::parse(l));
    }

    let mut edges = Vec::new();
    for i in 0..jbs.len() {
        for j in i + 1..jbs.len() {
            if i == j {
                continue;
            }
            edges.push(Edge::calc_edge(&jbs, (i, j)));
        }
    }

    edges.sort_by(|a, b| a.distance.total_cmp(&b.distance));

    let mut uf = UnionFind::new(jbs.len());
    let mut components = jbs.len();

    for edge in edges {
        if uf.union(edge.jb1, edge.jb2) {
            components -= 1;
            if components == 1 {
                // Esta es la última conexión necesaria
                let x1 = jbs[edge.jb1].x as u64;
                let x2 = jbs[edge.jb2].x as u64;
                return Some(x1 * x2);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_with_take(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
