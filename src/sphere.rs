use std::rc::Rc;

use crate::{vec3::Point3, hit::{Hit, HitRecord}, material::Scatter};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<dyn Scatter>,
}

impl Sphere {
    pub fn new(cen: Point3, r: f64, m: Rc<dyn Scatter>) -> Sphere {
        Sphere {
            center: cen,
            radius: r,
            mat: m, 
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length().powi(2);
        let half_b = oc.dot(r.direction());
        let c = oc.length().powi(2) - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
    
        if discriminant < 0.0 {
            return None;
        }

        // Find the nearest root that lies in the acceptable range
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) /a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = r.at(root);
        let mut rec = HitRecord {
            t: root,
            p: p,
            normal: (p - self.center) / self.radius,
            mat: self.mat.clone(),
            front_face: false
        };

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        Some(rec)
    }
}
