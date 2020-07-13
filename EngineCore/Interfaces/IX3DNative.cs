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
        #region Generic
        public RustError last_error_message();
        #endregion

        #region Renderer
        public void x3d_drop_renderer(ref IntPtr x3DRendererHandle);
        public X3DRendererHandle x3d_new_renderer(uint width, uint height);
        public void x3d_renderer_prepare_frame(X3DRendererHandle x3DRendererHandle);
        public void x3d_renderer_finish_frame(X3DRendererHandle x3DRendererHandle);
        public void x3d_renderer_draw_mesh(X3DRendererHandle objPtr, X3DCameraHandle camPtr, X3DMeshHandle meshPtr, X3DMaterialHandle matPtr);
        #endregion

        #region Shaders
        public void x3d_drop_shader(IntPtr objPtr);
        public X3DShaderHandle x3d_new_shader(string vs, string gs, string ts, string fs);
        #endregion

        #region Camera
        public void x3d_drop_camera(ref IntPtr objPtr);
        public X3DCameraHandle x3d_new_camera_default();
        public X3DCameraHandle x3d_new_camera(float fovy, float z_near, float z_far, float aperture, float shutter_speed, float iso, RustVector3 position); //, RustQuaternion rotation
        #endregion

        #region Material
        public void x3d_drop_material(ref IntPtr objPtr);
        public X3DMaterialHandle x3d_new_material(X3DShaderHandle shader);
        #endregion

        #region Mesh
        public void x3d_drop_mesh(ref IntPtr objPtr);
        public X3DMeshHandle x3d_new_mesh();
        #endregion
    }
}
