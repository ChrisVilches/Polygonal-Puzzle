use rayon::prelude::*;

use crate::{
  constants::EPS,
  shapes::{polygon::Polygon, segment::Segment},
  traits::{common_boundary::CommonBoundary, intersection::IntersectionHeuristic},
  util::{cmp, max, orientation},
};

fn range_contains(a: f64, b: f64, x: f64) -> bool {
  if a > b {
    b <= x + EPS && x - EPS <= a
  } else {
    a <= x + EPS && x - EPS <= b
  }
}

fn collect_shifts(
  polygon_edges: &Polygon,
  polygon_vertices: &Polygon,
  right: bool,
  max_shift: f64,
) -> Vec<f64> {
  let mut shifts = vec![];

  for wall in polygon_edges.edges() {
    if wall.is_horizontal() {
      continue;
    }

    for (v0, v1, v2) in polygon_vertices.vertices() {
      if !range_contains(wall.p.y, wall.q.y, v1.y) {
        continue;
      }

      if orientation(v0, v1, v2) == -1 {
        continue;
      }

      let point_left = Segment::new(v0, v1).face_left() || Segment::new(v1, v2).face_left();

      if wall.face_right() && !point_left {
        continue;
      }

      let point_right = Segment::new(v0, v1).face_right() || Segment::new(v1, v2).face_right();

      if wall.face_left() && !point_right {
        continue;
      }

      let mut x = wall.horizontal_distance(v1);
      x = if right { x } else { -x };

      if EPS < x && x < max_shift - EPS {
        shifts.push(x);
      }
    }
  }

  shifts
}

fn optimal_shift(polygon1: Polygon, polygon2: &Polygon, base1: f64, base2: f64) -> f64 {
  let mut polygon1 = polygon1;
  let max_shift = base1 + base2;
  let mut shifts = [
    vec![base1, base2],
    collect_shifts(&polygon1, polygon2, true, max_shift),
    collect_shifts(polygon2, &polygon1, false, max_shift),
  ]
  .concat();

  shifts.par_sort_unstable_by(cmp);

  let mut prev_shift_x = 0_f64;
  let mut res = 0_f64;
  let mut prev = (0, 0);

  for x in shifts {
    if x - prev_shift_x < 0.1 {
      continue;
    }

    polygon1
      .vertices
      .iter_mut()
      .for_each(|p| p.x += x - prev_shift_x);

    if !polygon1.intersects(polygon2, &mut prev) {
      res = max(res, polygon1.common_boundary(polygon2));
    }

    prev_shift_x = x;
  }

  res
}

fn pairs(a: usize, b: usize) -> Vec<(usize, usize)> {
  (0..a).flat_map(|i| (0..b).map(move |j| (i, j))).collect()
}

fn bases(polygon: &Polygon, rotations: &[Polygon]) -> Vec<f64> {
  (0..polygon.len())
    .map(|i| {
      let polygon = &rotations[i];
      polygon.vertices[i].dist(polygon.vertex_at((i + 1) as i32))
    })
    .collect()
}

pub fn best_match(polygon1: &Polygon, polygon2: &Polygon) -> f64 {
  let rotations1 = polygon1
    .rotations()
    .iter()
    .map(Polygon::negate)
    .collect::<Vec<Polygon>>();
  let rotations2 = polygon2.rotations();

  let base1 = bases(polygon1, &rotations1);
  let base2 = bases(polygon2, &rotations2);

  pairs(rotations1.len(), rotations2.len())
    .par_iter()
    .map(|(i, j)| {
      optimal_shift(
        rotations1[*i].clone(),
        &rotations2[*j],
        base1[*i],
        base2[*j],
      )
    })
    .max_by(cmp)
    .unwrap_or(0_f64)
}
