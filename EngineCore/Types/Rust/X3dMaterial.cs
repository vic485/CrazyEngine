using System;
using System.Runtime.InteropServices;

namespace EngineCore.Types.Rust
{
    internal class X3DMaterialNative {
        #region Dll Imports

        [DllImport("EngineRenderer", EntryPoint = "x3d_drop_material", CallingConvention = CallingConvention.Cdecl)]
        public static extern void CleanupX3DMaterial(IntPtr objPtr);
        [DllImport("EngineRenderer", EntryPoint = "x3d_new_material", CallingConvention = CallingConvention.Cdecl)]
        public static extern X3DMaterialHandle CreateX3DMaterial(X3DShaderHandle shader);

        #endregion
    }

    /// <summary>
    /// A handle to the raw rust mesh object sent through FFI
    /// </summary>
    public class X3DMaterialHandle : SafeHandle
    {
        public X3DMaterialHandle() : base(IntPtr.Zero, true) {}

        public override bool IsInvalid
        {
            get { return this.handle == IntPtr.Zero; }
        }

        protected override bool ReleaseHandle()
        {
            if (!this.IsInvalid)
            {
                X3DMaterialNative.CleanupX3DMaterial(handle);
            }

            return true;
        }
    }

    /// <summary>
    /// Represents the 3d camera as it is in rust
    /// </summary>
    public class X3DMaterial : IDisposable
    {
        private X3DMaterialHandle db;
        public X3DShader shader;

        public X3DMaterial(X3DShader shader)
        {
            db = X3DMaterialNative.CreateX3DMaterial(shader.GetHandle());
            this.shader = shader;
        }

        public void Dispose()
        {
            db.Dispose();
        }

        public X3DMaterialHandle GetHandle()
        {
            return db;
        }
    }
}
