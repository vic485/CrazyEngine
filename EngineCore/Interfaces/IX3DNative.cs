using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using EngineCore.Types.Rust;

namespace EngineCore.Interfaces
{
    public interface IX3DNative 
    {
        public void x3d_drop_renderer(ref IntPtr x3DRendererHandle);
        public X3DRendererHandle x3d_new_renderer(uint width, uint height);

        public void x3d_renderer_prepare_frame(X3DRendererHandle x3DRendererHandle);

        public void x3d_renderer_finish_frame(X3DRendererHandle x3DRendererHandle);

        public void x3d_renderer_draw_mesh(X3DRendererHandle objPtr, X3DCameraHandle camPtr, X3DMeshHandle meshPtr, X3DMaterialHandle matPtr);

        public void x3d_drop_shader(IntPtr objPtr);

        public X3DShaderHandle x3d_new_shader(string vs, string gs, string ts, string fs);
    }
}
