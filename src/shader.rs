use gfx::traits::FactoryExt;
use gfx;

static VERTEX: &'static [u8] = b"
    #version 150 core
    uniform mat4 u_projection, u_view;

    //in vec2 at_tex_coord;
    in vec3 at_color, at_position;

    //out vec2 v_tex_coord;
    out vec3 v_color;

    void main() {
        //v_tex_coord = at_tex_coord;
        v_color = at_color;
        gl_Position = u_projection * u_view * vec4(at_position, 1.0);
    }
";

static FRAGMENT: &'static [u8] = b"
    #version 150 core
    out vec4 out_color;

    //uniform sampler2D s_texture;

    //in vec2 v_tex_coord;
    in vec3 v_color;

    void main() {
        //vec4 tex_color = texture(s_texture, v_tex_coord);
        //if(tex_color.a == 0.0) // Discard transparent pixels.
        //    discard;
        out_color = vec4(v_color, 1.0);
    }
";

gfx_vertex_struct!( Vertex {
    xyz: [f32; 3] = "at_position",
    //uv: [f32; 2] = "at_tex_coord",
    rgb: [f32; 3] = "at_color",
});

gfx_pipeline!( pipe {
    vbuf: gfx::VertexBuffer<Vertex> = (),
    transform: gfx::Global<[[f32; 4]; 4]> = "u_projection",
    view: gfx::Global<[[f32; 4]; 4]> = "u_view",
    //color: gfx::TextureSampler<[f32; 4]> = "s_texture",
    out_color: gfx::RenderTarget<gfx::format::Srgba8> = "out_color",
    out_depth: gfx::DepthTarget<gfx::format::DepthStencil> =
        gfx::preset::depth::LESS_EQUAL_WRITE,
});

pub struct RenderOutput<R>
    where R: gfx::Resources
{
    out_color: gfx::handle::RenderTargetView<R, gfx::format::Srgba8>,
    out_depth: gfx::handle::DepthStencilView<R, gfx::format::DepthStencil>,
}


impl <R> RenderOutput<R>
    where R: gfx::Resources
{
    pub fn new(out_color: &gfx::handle::RenderTargetView<R, gfx::format::Srgba8>, out_depth: &gfx::handle::DepthStencilView<R, gfx::format::DepthStencil>) -> RenderOutput<R>  {
        RenderOutput {
            out_color: out_color.clone(),
            out_depth: out_depth.clone(),
        }
    }
}


pub struct Renderer<R: gfx::Resources, C: gfx::CommandBuffer<R>> {
    pub pipe: gfx::PipelineState<R, pipe::Meta>,
    encoder: gfx::Encoder<R, C>,
    clear_color: [f32; 4],
    clear_depth: f32,
    clear_stencil: u8,
}

impl<R: gfx::Resources, C: gfx::CommandBuffer<R>> Renderer<R, C> {

    pub fn new<F: gfx::Factory<R>>(factory: &mut F, encoder: gfx::Encoder<R, C>) -> Renderer<R, C> {

        /*let sampler = factory.create_sampler(
                gfx::texture::SamplerInfo::new(
                    gfx::texture::FilterMethod::Scale,
                    gfx::texture::WrapMode::Tile
                )
            );*/

        //let texture_view = factory.view_texture_as_shader_resource::<gfx::format::Rgba8>(
        //    &tex, (0, 0), gfx::format::Swizzle::new()).unwrap();

        let prog = factory.link_program(VERTEX, FRAGMENT).unwrap();

        let mut rasterizer = gfx::state::Rasterizer::new_fill();
        rasterizer.front_face = gfx::state::FrontFace::Clockwise;
        let pipe = factory.create_pipeline_from_program(&prog, gfx::Primitive::TriangleList,
            rasterizer, pipe::new()).unwrap();


        Renderer {
            pipe: pipe,
            encoder: encoder,
            clear_color: [0.81, 0.8, 1.0, 1.0],
            clear_depth: 1.0,
            clear_stencil: 0,
        }
    }

    pub fn clear<'a, 'b>(&mut self, target: &RenderOutput<R>) {
        self.encoder.clear(&target.out_color, self.clear_color);
        self.encoder.clear_depth(&target.out_depth, self.clear_depth);
        self.encoder.clear_stencil(&target.out_depth, self.clear_stencil);
    }

    pub fn flush<D: gfx::Device<Resources=R, CommandBuffer=C> + Sized>(&mut self, device: &mut D) {
        self.encoder.flush(device);
    }

    pub fn create_buffer<F: gfx::Factory<R>>(&mut self, factory: &mut F, data: &[Vertex]) -> gfx::handle::Buffer<R, Vertex> {
        let vbuf = factory.create_vertex_buffer(data);
        vbuf
    }

    pub fn render<'a, 'b>(&mut self, target: &RenderOutput<R>, transform: [[f32; 4]; 4], view: [[f32; 4]; 4], buffer: gfx::handle::Buffer<R, Vertex>) {
        let slice = gfx::Slice::new_match_vertex_buffer(&buffer);
        let data = pipe::Data {
            vbuf: buffer,
            transform,
            view,
            out_color: target.out_color.clone(),
            out_depth: target.out_depth.clone(),
        };
        self.encoder.draw(&slice, &self.pipe, &data);
    }
}
