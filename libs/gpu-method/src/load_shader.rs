use comptu::Context;

pub fn load_shader(path: &str, ctx: &Context) -> wgpu::ShaderModule {
    let wgsl = std::fs::read_to_string(path).expect("Unable to open WGSL file");

    ctx.device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("compute_shader"),
            source: wgpu::ShaderSource::Wgsl(wgsl.into()),
        })
}
