use rayon::prelude::*;

use crate::{
  constants::EPS,
  shapes::{point::Point, polygon::Polygon},
  traits::{common_boundary::CommonBoundary, intersection::IntersectsHeuristic},
  util::{ccw, cmp},
};

#[inline]
fn range_contains(a: f64, b: f64, x: f64) -> bool {
  if a > b {
    b <= x + EPS && x - EPS <= a
  } else {
    a <= x + EPS && x - EPS <= b
  }
}

fn face_left((a, b, c): (Point, Point, Point)) -> bool {
  a.seg(b).face_left() || b.seg(c).face_left()
}

fn face_right((a, b, c): (Point, Point, Point)) -> bool {
  a.seg(b).face_right() || b.seg(c).face_right()
}

fn collect_shifts(edges: &Polygon, vertices: &Polygon, right: bool, max_shift: f64) -> Vec<f64> {
  edges
    .edges()
    .filter(|w| !w.is_horizontal())
    .flat_map(|wall| {
      vertices
        .vertices()
        .filter(|(a, b, c)| ccw(*a, *b, *c))
        .filter(move |(_, b, _)| range_contains(wall.p.y, wall.q.y, b.y))
        .filter(move |v| !wall.face_right() || face_left(*v))
        .filter(move |v| !wall.face_left() || face_right(*v))
        .map(move |(_, b, _)| wall.horizontal_distance(b))
        .map(|x| if right { x } else { -x })
        .filter(|x| range_contains(EPS, max_shift - EPS, *x))
    })
    .collect()
}

fn collect_all_shifts(polygon1: &Polygon, polygon2: &Polygon, base1: f64, base2: f64) -> Vec<f64> {
  let max_shift = base1 + base2;
  let mut shifts = [
    vec![base1, base2],
    collect_shifts(polygon1, polygon2, true, max_shift),
    collect_shifts(polygon2, polygon1, false, max_shift),
  ]
  .concat();
  shifts.sort_unstable_by(cmp);
  shifts
}

#[allow(clippy::too_many_lines)]
fn optimal_shift(mut polygon1: Polygon, polygon2: &Polygon, base1: f64, base2: f64) -> (f64, f64) {
  let mut prev_shift_x = 0_f64;
  let mut solution = (0_f64, 0_f64);
  let mut prev = (0, 0);

  for x in collect_all_shifts(&polygon1, polygon2, base1, base2) {
    if x - prev_shift_x < 0.1 {
      continue;
    }

    polygon1
      .vertices
      .iter_mut()
      .for_each(|p| p.x += x - prev_shift_x);

    if !polygon1.intersects(polygon2, &mut prev) {
      let boundary = polygon1.common_boundary(polygon2);
      if boundary > solution.0 {
        solution.0 = boundary;
        solution.1 = x;
      }
    }

    prev_shift_x = x;
  }

  solution
}

fn pairs(a: usize, b: usize) -> Vec<(usize, usize)> {
  (0..a).flat_map(|i| (0..b).map(move |j| (i, j))).collect()
}

fn bases(polygon: &Polygon, rotations: &[Polygon]) -> Vec<f64> {
  (0..polygon.len())
    .into_par_iter()
    .map(|i| {
      let polygon = &rotations[i];
      polygon.vertices[i].dist(polygon.vertex_at((i + 1) as i32))
    })
    .collect()
}

fn both_rotations(polygon1: &Polygon, polygon2: &Polygon) -> (Vec<Polygon>, Vec<Polygon>) {
  let rotations1 = polygon1
    .rotations()
    .iter()
    .map(Polygon::negate)
    .collect::<Vec<Polygon>>();
  let rotations2 = polygon2.rotations();

  (rotations1, rotations2)
}

#[must_use]
#[allow(clippy::too_many_lines)]
pub fn best_match(polygon1: &Polygon, polygon2: &Polygon) -> (Polygon, Polygon, f64) {
  let (rotations1, rotations2) = both_rotations(polygon1, polygon2);

  let base1 = bases(polygon1, &rotations1);
  let base2 = bases(polygon2, &rotations2);

  pairs(rotations1.len(), rotations2.len())
    .par_iter()
    .map(|(i, j)| {
      (
        *i,
        *j,
        optimal_shift(
          rotations1[*i].clone(),
          &rotations2[*j],
          base1[*i],
          base2[*j],
        ),
      )
    })
    .max_by(|(_, _, (a, _)), (_, _, (b, _))| cmp(a, b))
    .map(|(i, j, (boundary, offset))| {
      let mut p1 = rotations1[i].clone();
      let p2 = rotations2[j].clone();

      p1.vertices.iter_mut().for_each(|p| p.x += offset);

      (p1, p2, boundary)
    })
    .expect("there should be at least one solution")
}
