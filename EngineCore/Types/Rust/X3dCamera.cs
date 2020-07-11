using System;
using System.Runtime.InteropServices;

namespace EngineCore.Types.Rust
{
    internal class X3DCameraNative {
        #region Dll Imports

        [DllImport("EngineRenderer", EntryPoint = "x3d_drop_camera", CallingConvention = CallingConvention.Cdecl)]
        public static extern void CleanupX3DCamera(IntPtr objPtr);
        [DllImport("EngineRenderer", EntryPoint = "x3d_new_camera_default", CallingConvention = CallingConvention.Cdecl)]
        public static extern X3DCameraHandle CreateX3DCameraDefault();
        [DllImport("EngineRenderer", EntryPoint = "x3d_new_camera", CallingConvention = CallingConvention.Cdecl)]
        public static extern X3DCameraHandle CreateX3DCamera(float fovy, float z_near, float z_far, float aperture, float shutter_speed, float iso, RustVector3 position); //, Quaternion rotation

        #endregion
    }

    /// <summary>
    /// A handle to the raw rust mesh object sent through FFI
    /// </summary>
    public class X3DCameraHandle : SafeHandle
    {
        public X3DCameraHandle() : base(IntPtr.Zero, true) {}

        public override bool IsInvalid
        {
            get { return this.handle == IntPtr.Zero; }
        }

        protected override bool ReleaseHandle()
        {
            if (!this.IsInvalid)
            {
                X3DCameraNative.CleanupX3DCamera(handle);
            }

            return true;
        }
    }

    /// <summary>
    /// Represents the 3d camera as it is in rust
    /// </summary>
    public class X3DCamera : IDisposable
    {
        private X3DCameraHandle db;

        public X3DCamera()
        {
            db = X3DCameraNative.CreateX3DCameraDefault();
        }

        public X3DCamera(float fovy, float z_near, float z_far, float aperture, float shutter_speed, float iso, RustVector3 position) //, Quaternion rotation
        {
            db = X3DCameraNative.CreateX3DCamera(fovy, z_near, z_far, aperture, shutter_speed, iso, position); //, rotation
        }

        public void Dispose()
        {
            db.Dispose();
        }

        public X3DCameraHandle GetHandle()
        {
            return db;
        }
    }
}
