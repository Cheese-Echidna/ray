use crate::*;
use winit::dpi::Position;
use winit::event::VirtualKeyCode::P;

#[derive(Debug)]
pub struct LightSource {
    position: Vec3,
    intensity: f64,
    colour: Srgb,
}

impl RenderObject for LightSource {
    fn intersects(&self, ray: Ray) -> Option<Vec3> {
        self.sphere().intersects(ray)
    }

    fn normal_at(&self, point: Vec3) -> Vec3 {
        self.sphere().normal_at(point)
    }

    fn colour(&self) -> Srgb {
        self.colour
    }
}

impl LightSource {
    pub fn new(position: Vec3, intensity: f64) -> Self {
        Self {
            position,
            intensity,
            colour: WHITE.into(),
        }
    }

    fn sphere(&self) -> Sphere {
        Sphere::new(self.position, 0.1, self.colour)
    }

    pub fn ray_from(&self, intersection: Vec3) -> Ray {
        Ray::new(intersection, (self.position - intersection).normalize())
    }

    pub fn intensity_for(
        &self,
        original_ray: Ray,
        intersection: Vec3,
        object: &Box<dyn RenderObject>,
        scene: &Scene,
    ) -> f64 {
        let to_light_ray = self.ray_from(intersection);
        if let Some((_new_object, new_intersection)) = to_light_ray.trace(scene, 0.001) {
            // println!(
            //     "v(({}, {}, {}), ({}, {}, {}))",
            //     intersection.x,
            //     intersection.y,
            //     intersection.z,
            //     new_intersection.x,
            //     new_intersection.y,
            //     new_intersection.z
            // );
            // println!("Distance: {}", new_intersection.distance(intersection));
            // println!();
            return 0.0;
        };
        return 0.5;
        // let reflect_dir = bounce_across_normal(original_ray.direction().normalize(), object.normal_at(intersection).normalize());
        //
        // let dot_p = reflect_dir.dot(to_light_ray.direction());
        //
        // let reflected_light = (0.5 * dot_p + 0.5) * self.intensity;
        // reflected_light
    }
}

/*

v((0.9310780639460621, 1.3936272624740766, 1.0912821322313704), (0.39646439822643764, 1.50422655219915, 1.2570276292102758))
Distance: 0.5705396955783615

v((0.9225375665544364, 1.4001092911968254, 1.0902377772761938), (0.4032143607664027, 1.5067074645046235, 1.2518988759791225))
Distance: 0.554250912003181

v((0.9010126406940442, 1.4161846022048126, 1.0874729393357523), (0.42017767139995854, 1.512950419400667, 1.2387218222394623))
Distance: 0.5132661255657205

v((0.8923322409037224, 1.422562763697024, 1.0863049089396704), (0.4269983111237777, 1.5154639645238617, 1.2333050776377519))
Distance: 0.49676488293435095

v((0.8704518415681886, 1.4383779766892024, 1.0832277635317604), (0.4441378597957307, 1.5217889984459465, 1.2193850112679552))
Distance: 0.4552359890389676

v((0.8571985962178514, 1.4477776134587392, 1.0812726522990648), (0.45448157991999255, 1.525612368149982, 1.2107656237526865))
Distance: 0.430125183998315


 */
