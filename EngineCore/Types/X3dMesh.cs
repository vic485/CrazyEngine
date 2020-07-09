using System;
using System.Runtime.InteropServices;

namespace EngineCore.Types
{
    internal class X3DMeshNative {
        #region Dll Imports

        [DllImport("EngineRenderer.dll", EntryPoint = "x3d_drop_mesh", CallingConvention = CallingConvention.Cdecl)]
        public static extern void CleanupX3DMesh(IntPtr objPtr);
        [DllImport("EngineRenderer.dll", EntryPoint = "x3d_new_mesh", CallingConvention = CallingConvention.Cdecl)]
        public static extern X3DMeshHandle CreateX3DMesh(X3DRendererHandle rendererPtr);

        #endregion
    }

    /// <summary>
    /// A handle to the raw rust mesh object sent through FFI
    /// </summary>
    public class X3DMeshHandle : SafeHandle
    {
        public X3DMeshHandle() : base(IntPtr.Zero, true) {}

        public override bool IsInvalid
        {
            get { return this.handle == IntPtr.Zero; }
        }

        protected override bool ReleaseHandle()
        {
            if (!this.IsInvalid)
            {
                X3DMeshNative.CleanupX3DMesh(handle);
            }

            return true;
        }
    }

    /// <summary>
    /// Represents the 3d mesh as it is in rust
    /// </summary>
    public class X3DMesh : IDisposable
    {
        private X3DMeshHandle db;

        public X3DMesh(X3DRenderer renderer)
        {
            db = X3DMeshNative.CreateX3DMesh(renderer.GetHandle());

            //Check for errors
            RustError err = Native.LastErrorMessage();
            RustString message = new RustString(err.message);
            Console.WriteLine(message.AsString());
            //Not checking anything right now though :)
        }

        public void Dispose()
        {
            db.Dispose();
        }

        public X3DMeshHandle GetHandle()
        {
            return db;
        }
    }
}
