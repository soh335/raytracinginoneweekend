use hitable::*;
use ray::Ray;
use material::Material;

pub struct HitableList {
    list: Vec<Box<Hitable>>
}

impl HitableList {
    pub fn new() -> Self {
        HitableList{list: Vec::new()}
    }
    pub fn push(&mut self, v: Box<Hitable>) {
        self.list.push(v);
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<(HitRecord, &Box<Material>)> {
        let mut closest_so_far = t_max;
        let mut ret :Option<(HitRecord, &Box<Material>)> = None;
        for v in self.list.iter() {
            if let Some((_rec, material)) = v.hit(r, t_min, closest_so_far) {
                closest_so_far = _rec.t;
                ret = Some((_rec, material));
            }
        }
        ret
    }
}
