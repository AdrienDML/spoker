
pub trait F32Ext {
    fn decay(self, b: f32, decay: f32, dt: f32) -> f32;
}

impl F32Ext for f32 {
    fn decay(self, b: f32, decay: f32, dt: f32) -> f32 {
        b + (self - b) * (decay * dt).exp()
    }

}
