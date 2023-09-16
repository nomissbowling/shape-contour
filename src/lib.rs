#![doc(html_root_url = "https://docs.rs/shape-contour/0.2.2")]
//! Rust crate shape-contour supports ESRI J shapefile (C bindings)
//!
//! # Requirements
//!
//! - [ shapelib-rs ]( https://crates.io/crates/shapelib-rs )
//! - [ OSGeo ]( https://OSGeo.org/ )
//! - [ OSGeo shapelib (C) ]( https://github.com/OSGeo/shapelib )
//! - [ shapelib ]( http://shapelib.maptools.org/ )
//! - [ ESRI J shapefile ]( https://www.esrij.com/products/japan-shp/ )
//!
//! link shapelib_i.lib
//!

pub mod contours;

#[cfg(test)]
mod tests {
  use super::contours::{self, shape};
  use std::path::PathBuf;

  /// with [-- --nocapture] or with [-- --show-output]
  #[test]
  fn check_shape_contour() {
    let rp = "../shapelib-rs";
    let s_path: String = if cfg!(docsrs) {
      std::env::var("OUT_DIR").unwrap()
    }else{
      rp.to_string()
    }; // to keep lifetime
    let o_path: &str = s_path.as_str();
    if o_path != rp { return; }
    let bp = PathBuf::from(o_path).join("shp").join("ESRIJ_com_japan_ver84");
    println!("{}", bp.join("japan_ver84.shp").to_str().unwrap());
    println!("{}", bp.join("japan_ver84.dbf").to_str().unwrap());
    let s = bp.join("japan_ver84"); // to keep lifetime
    let u = s.to_str().unwrap(); // to keep lifetime
    let shp = shape::ShapeF::new(u, "cp932").unwrap();
    shp.disp_record_inf().unwrap();
    let sci = shp.get_shp_contours(false).unwrap();
    drop(shp);
    let mut gci = contours::GrpContoursInf::new(sci).unwrap();
    gci.grp_contours.clear();
    gci.get_grp_contours(50.0, 26, 343, false).unwrap(); // 50.0 (1600, 1200)
    println!("n_grp_contours: {}", gci.grp_contours.len());
    gci.grp_contours.clear();
    gci.get_grp_contours(50.0, 0, 0, false).unwrap(); // 1-47, 0
    println!("n_grp_contours: {}", gci.grp_contours.len());
  }
}
