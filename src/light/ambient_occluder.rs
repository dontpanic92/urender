use super::*;
use utility::*;
use sampler::*;
use std::error::Error;

pub struct AmbientOccluder {
    color: RGBColor,
    sampler: Box<Sampler>,
    max_distance: f64
}

impl AmbientOccluder {
    pub fn new(color: RGBColor) -> AmbientOccluder {
        AmbientOccluder { color: color,  sampler: Box::new(MultiJitteredSampler::new(8, 8)), max_distance: 200.}
    }
    
    pub fn new_from_dict(map: &Dictionary) -> Result<AmbientOccluder, Box<Error>> {
        let color = RGBColor::from_hex(map.get("color").ok_or("color is missing")?)?;

        Ok(AmbientOccluder::new(color))
    }
}

impl Light for AmbientOccluder {
    fn color(&self) -> RGBColor {
        self.color
    }

    fn incident_radiance_at(&self, point: &HitPoint, world: &World) -> RGBColor {
        let up = Vector3D::new(0.0072, 1., 0.0034);
        let w = point.normal();
        let v = w.cross(up).normalize();
        let u = w.cross(v);

        let mut color = BLACK;
        let samples = self.sampler.sample_hemisphere();
        for sample in samples {
            let direction = (u * sample.x() + v * sample.y() + w * sample.z()).normalize();
            let ray = Ray::new(point.coord(), direction);
            let delta = match world.hit_objects(&ray) {
                None => self.color,
                Some(point) => {
                    let t = (point.coord() - ray.origin()).norm();
                    if t > self.max_distance {
                        self.color
                    } else {
                        self.color * (t / self.max_distance)
                    }
                }
            };
            color = color + delta;
        }

        color / self.sampler.sample_num() as f64
    }

    fn direction(&self, _: Coord3D) -> Vector3D {
        Vector3D::new(0, 0, 0)
    }

    fn is_in_shadow(&self, _: Coord3D, _: &World) -> bool {
        true
    }
}
