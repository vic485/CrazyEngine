using System;
using System.Runtime.InteropServices;

namespace EngineCore.Types
{
    internal class X3DRendererNative {
        #region Dll Imports

        [DllImport("EngineRenderer.dll", EntryPoint = "x3d_drop_renderer", CallingConvention = CallingConvention.Cdecl)]
        public static extern void CleanupX3DRenderer(IntPtr objPtr);
        [DllImport("EngineRenderer.dll", EntryPoint = "x3d_new_renderer", CallingConvention = CallingConvention.Cdecl)]
        public static extern X3DRendererHandle CreateX3DRenderer(uint width, uint height);
        [DllImport("EngineRenderer.dll", EntryPoint = "x3d_renderer_prepare_frame", CallingConvention = CallingConvention.Cdecl)]
        public static extern void X3DRendererPrepareFrame(X3DRendererHandle objPtr);

        #endregion
    }

    /// <summary>
    /// A handle to the raw rust renderer object sent through FFI
    /// </summary>
    public class X3DRendererHandle : SafeHandle
    {
        public X3DRendererHandle() : base(IntPtr.Zero, true) {}

        public override bool IsInvalid
        {
            get { return this.handle == IntPtr.Zero; }
        }

        protected override bool ReleaseHandle()
        {
            if (!this.IsInvalid)
            {
                X3DRendererNative.CleanupX3DRenderer(handle);
            }

            return true;
        }
    }

    /// <summary>
    /// Represents the 3d renderer as it is in rust
    /// </summary>
    public class X3DRenderer : IDisposable
    {
        private X3DRendererHandle db;

        public X3DRenderer(uint width, uint height)
        {
            db = X3DRendererNative.CreateX3DRenderer(width, height);
        }

        public void PrepareFrame()
        {
            X3DRendererNative.X3DRendererPrepareFrame(db);
        }

        public void Dispose()
        {
            db.Dispose();
        }

        public X3DRendererHandle GetHandle() {
            return db;
        }
    }
}
