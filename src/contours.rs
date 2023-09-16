//! grpcontours
//!
//! to control publication imported modules
//!
//! # Requirements
//!
//! - [ shapelib-rs ]( https://crates.io/crates/shapelib-rs )
//!
//! - ./shapelib/include/shapefil.h
//!

#![allow(unused)]
// #![allow(unused_imports)]
// #![allow(unused_attributes)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

mod cshapefil;
use cshapefil::*;

use std::error::Error;
use std::collections::BTreeMap;

pub use shapelib::shape; // {Pt2d, ShpContoursInf};

/// get min max from (Bounds or Contour2(d) Vec (shape::Pt2d or Pt))
pub fn get_min_max(points: &Vec<shape::Pt2d>) -> Vec<shape::Pt2d> {
  let mut r: Vec<shape::Pt2d> = (0..2).into_iter().map(|_| {
    shape::Pt2d{x: points[0].x, y: points[0].y} }).collect();
  for p in points {
    if r[0].x > p.x { r[0].x = p.x };
    if r[0].y > p.y { r[0].y = p.y };
    if r[1].x < p.x { r[1].x = p.x };
    if r[1].y < p.y { r[1].y = p.y };
  }
  r
}

/// get min max of contours from (Contours2(d) Vec Vec (shape::Pt2d or Pt))
pub fn get_mm_of_contours(contours: &shape::Contours2d) -> Vec<shape::Pt2d> {
  let mut r = Vec::<shape::Pt2d>::with_capacity(2);
  for (i, contour) in contours.iter().enumerate() {
    let mut q = get_min_max(&contour);
    if i == 0 {
      for p in q { r.push(p); } // move p initialize for r
    } else {
      for p in &r { q.push(shape::Pt2d{x: p.x, y: p.y}); } // not use push(*p)
      for (j, p) in get_min_max(&q).into_iter().enumerate() {
        r[j] = p; // move p
      }
    }
  }
  r
}

/// Pt
#[derive(Debug)]
pub struct Pt {
  /// x
  pub x: i32,
  /// y
  pub y: i32
}

/// Contour2
pub type Contour2 = Vec<Pt>;

/// Contours2
pub type Contours2 = Vec<Contour2>;

/// MapContours
pub type MapContours = BTreeMap<i32, Contours2>;

/// GrpContoursInf
#[derive(Debug)]
pub struct GrpContoursInf {
  /// scale
  pub scale: f64,
  /// offset
  pub offset: shape::Pt2d,
  /// mm
  pub mm: Vec<shape::Pt2d>,
  /// grp_contours
  pub grp_contours: Vec<i32>,
  /// grp_scaled_contours
  pub grp_scaled_contours: MapContours,
  /// sci
  pub sci: shape::ShpContoursInf
}

/// GrpContoursInf
impl GrpContoursInf {
  /// constructor
  pub fn new(sci: shape::ShpContoursInf) ->
    Result<GrpContoursInf, Box<dyn Error>> {
    Ok(GrpContoursInf{
      scale: 0.0,
      offset: shape::Pt2d{x: 0.0, y: 0.0},
      mm: (0..2).into_iter().map(|_| shape::Pt2d{x: 0.0, y: 0.0}).collect(),
      grp_contours: vec![],
      grp_scaled_contours: vec![].into_iter().collect(),
      sci: sci})
  }

  /// get_grp_contours
  pub fn get_grp_contours(&mut self, scale: f64, w_pref: i32, w_city: i32,
    ignore: bool) -> Result<(), Box<dyn Error>> {
    for si in 0..self.sci.shp.len() as i32 {
      let flds = &self.sci.rec[&si];
      let (pref, city) = match shape::get_pref_city(flds[0].as_str()) {
      Err(e) => { if !ignore { println!("{} at {}\x07", e, si) }; (0, 0) },
      Ok(r) => r
      };
/*
      if pref != 26 { continue; } // 26 1177-1212
      if city != 343 { continue; } // 343 1204 etc
*/
      if w_pref != 0 && pref != w_pref { continue; }
      if w_city != 0 && city != w_city { continue; }
      let shp_k = &self.sci.shp[&si];
      let mut mmc = get_mm_of_contours(shp_k);
      if self.grp_contours.len() == 0 {
        // self.mm = Vec::<shape::Pt2d>::with_capacity(2); // clear self.mm
        self.mm.clear();
        for p in mmc { self.mm.push(p); } // move p initialize for self.mm
      } else {
        for p in &self.mm { mmc.push(shape::Pt2d{x: p.x, y: p.y}); } // copy *p
        for (j, p) in get_min_max(&mmc).into_iter().enumerate() {
          self.mm[j] = p; // move p
        }
      }
      self.grp_contours.push(si);
    }
    let range = shape::Pt2d{
      x: self.mm[1].x - self.mm[0].x, y: self.mm[1].y - self.mm[0].y};
    self.offset = shape::Pt2d{x: self.mm[0].x, y: self.mm[0].y};
    let xscale = (self.sci.minmax[1][0] - self.sci.minmax[0][0]) / range.x;
    let yscale = (self.sci.minmax[1][1] - self.sci.minmax[0][1]) / range.y;
    self.scale = scale * (if xscale < yscale { xscale } else { yscale });
/*
    print!("({:4} {:4}) scale{:7.1}", 1600, 1200, self.scale);
    println!(" range({:9.4},{:9.4}) offset({:9.4},{:9.4})",
      range.x * self.scale, range.y * self.scale,
      self.offset.x * self.scale, self.offset.y * self.scale);
*/
/*
all prefectures range=(maxBound - minBound)(Points)
(  32   24) scale    1.0 range(  31.0528,  21.5116) offset( 122.9339,  24.0456)
( 640  480) scale   20.0 range( 621.0552, 430.2325) offset(2458.6783, 480.9123)
(1280  960) scale   40.0 range(1242.1105, 860.4649) offset(4917.3565, 961.8246)
(1600 1200) scale   50.0 range(1552.6381,1075.5812) offset(6146.6957,1202.2808)
shapeId=1204
(x) scale17907.4 range(1552.6381, 648.8804) offset(2431661.2717,623000.3392)
*/
    Ok(())
  }

  /// get_scaled_contours
  pub fn get_scaled_contours(&mut self, si: i32, ofs: &shape::Pt2d) ->
    Result<i32, Box<dyn Error>> {
    let contours = self.grp_scaled_contours.entry(si).or_insert_with(|| {
      let mut cts = Vec::<Contour2>::with_capacity(self.sci.shp[&si].len());
      for pts in &self.sci.shp[&si] {
        let mut contour = Vec::<Pt>::with_capacity(pts.len());
        for p in pts {
          contour.push(Pt{ // add offset before scale
            x: ((ofs.x + p.x) * self.scale) as i32,
            y: ((ofs.y + p.y) * self.scale) as i32});
        }
        cts.push(contour);
      }
      cts
    });
    Ok(contours.len() as i32)
  }

  /// whole_scaled
  pub fn whole_scaled(&mut self) -> Result<i32, Box<dyn Error>> {
    let mut total = 0i32;
    let offset = shape::Pt2d{x: 0.0, y: 0.0};
    for i in 0..self.grp_contours.len() { // borrow '&si in &self.grp_contours'
      total += self.get_scaled_contours(self.grp_contours[i], &offset)?;
    }
    Ok(total)
  }
}
