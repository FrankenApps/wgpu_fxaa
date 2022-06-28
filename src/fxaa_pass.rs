pub struct FxaaPass {
    
}

impl FxaaPass {
    pub fn new(
        device: &wgpu::Device, 
        queue: &wgpu::Queue, 
        size: &wgpu::Extent3d
    ) -> FxaaPass {
        FxaaPass {  }
    }

    pub fn start_frame(&mut self, view: &wgpu::TextureView) {

    }

    pub fn resolve(self) {
        std::mem::drop(self);
    }
}