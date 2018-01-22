pub struct ViewPlane {
    hres: u32,
    vres: u32,
    pixel_size: f64,
    gamma: f64,
    inv_gamma: f64
}

impl ViewPlane {
    pub fn new() -> ViewPlane {
        ViewPlane {
            hres: 1920, 
            vres: 1080, 
            pixel_size: 1., 
            gamma: 1., 
            inv_gamma: 1.}
    }
    
    pub fn set_hres(&mut self, hres: u32) {
        self.hres = hres;
    }
    
    pub fn hres(&self) -> u32 {
        self.hres
    }
    
    pub fn set_vres(&mut self, vres: u32) {
        self.vres = vres;
    }
    
    pub fn vres(&self) -> u32 {
        self.vres
    }
        
    pub fn set_pixel_size(&mut self, pixel_size: f64) {
        self.pixel_size = pixel_size
    }
    
    pub fn pixel_size(&self) -> f64 {
        self.pixel_size
    }
    
    pub fn set_gamma(&mut self, gamma: f64) {
        self.gamma = gamma;
        self.inv_gamma = 1.0 / gamma;
    }
    
    pub fn gamma(&self) -> f64 {
        self.gamma
    }
    
    pub fn inv_gamma(&self) -> f64 {
        self.inv_gamma
    }
}
