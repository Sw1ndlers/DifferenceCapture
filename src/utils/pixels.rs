use opencv::{
    core::{Mat, Vec3b},
    hub_prelude::{MatTrait, MatTraitConst},
};

pub fn mut_pixel_from_position(mat: &mut Mat, x: i32, y: i32) -> &mut Vec3b {
    mat.at_2d_mut(y, x).unwrap()
}

pub fn pixel_from_position(mat: &Mat, x: i32, y: i32) -> Vec3b {
    *mat.at_2d(y, x).unwrap()
}
