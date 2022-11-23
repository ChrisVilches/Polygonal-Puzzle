use rayon::prelude::*;
use std::cmp::Ordering;

use crate::{
  constants::EPS,
  shapes::{polygon::Polygon, segment::Segment},
  traits::{common_boundary::CommonBoundary, intersection::IntersectionHeuristic},
  util::{max, orientation},
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
  shifts: &mut Vec<f64>,
  max_shift: f64,
) -> () {
  for i in 0..polygon_edges.len() {
    let wall = Segment {
      p: polygon_edges.vertices[i],
      q: polygon_edges.vertex_at((i as i32) + 1),
    };

    if wall.is_horizontal() {
      continue;
    }

    for j in 0..polygon_vertices.len() {
      let v = polygon_vertices.vertices[j];
      if !range_contains(wall.p.y, wall.q.y, v.y) {
        continue;
      }

      let v0 = polygon_vertices.vertex_at((j as i32) - 1);
      let v2 = polygon_vertices.vertex_at((j as i32) + 1);
      if orientation(v0, v, v2) == -1 {
        continue;
      }

      let point_left = Segment::new(v0, v).face_left() || Segment::new(v, v2).face_left();
      if wall.face_right() && !point_left {
        continue;
      }

      let point_right = Segment::new(v0, v).face_right() || Segment::new(v, v2).face_right();
      if wall.face_left() && !point_right {
        continue;
      }

      let mut x = wall.horizontal_distance(v);
      x = if right { x } else { -x };

      if EPS < x && x < max_shift - EPS {
        shifts.push(x);
      }
    }
  }
}

fn cmp(a: &f64, b: &f64) -> Ordering {
  if a < b {
    Ordering::Less
  } else {
    Ordering::Greater
  }
}

fn optimal_shift(polygon1: Polygon, polygon2: &Polygon, base1: f64, base2: f64) -> f64 {
  let mut polygon1 = polygon1;
  let max_shift = base1 + base2;
  let mut shifts = vec![base1, base2];

  collect_shifts(&polygon1, polygon2, true, &mut shifts, max_shift);
  collect_shifts(polygon2, &polygon1, false, &mut shifts, max_shift);

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
  (0..a)
    .flat_map(|i| (0..b).map(|j| (i, j)).collect::<Vec<(usize, usize)>>())
    .collect::<Vec<(usize, usize)>>()
}

fn bases(polygon: &Polygon, rotations: &Vec<Polygon>) -> Vec<f64> {
  (0..polygon.len())
    .map(|i| {
      let polygon = &rotations[i];
      polygon.vertices[i].dist(polygon.vertex_at((i + 1) as i32))
    })
    .collect::<Vec<f64>>()
}

pub fn best_match(polygon1: &Polygon, polygon2: &Polygon) -> f64 {
  let rotations1 = polygon1
    .rotations()
    .iter()
    .map(Polygon::negate)
    .collect::<Vec<Polygon>>();
  let rotations2 = polygon2.rotations();

  let base1 = bases(&polygon1, &rotations1);
  let base2 = bases(&polygon2, &rotations2);

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
    .max_by(|a, b| {
      if a < b {
        Ordering::Less
      } else {
        Ordering::Greater
      }
    })
    .unwrap_or(0_f64)
}
