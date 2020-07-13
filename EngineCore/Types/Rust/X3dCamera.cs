using System;
using System.Runtime.InteropServices;
using EngineCore.Interfaces;
using AdvancedDLSupport;

namespace EngineCore.Types.Rust
{
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
                var NativeLibraryBuilder = new NativeLibraryBuilder();
                IX3DNative library = NativeLibraryBuilder.Default.ActivateInterface<IX3DNative>("EngineRenderer");

                // X3DCameraNative.CleanupX3DCamera(handle);
                library.x3d_drop_camera(ref handle);
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

        private NativeLibraryBuilder nativeLibrary = new NativeLibraryBuilder();
        private IX3DNative library;

        public X3DCamera()
        {
            library = nativeLibrary.ActivateInterface<IX3DNative>("EngineRenderer");

            db = library.x3d_new_camera_default();
        }

        public X3DCamera(float fovy, float z_near, float z_far, float aperture, float shutter_speed, float iso, RustVector3 position) //, Quaternion rotation
        {
            library = nativeLibrary.ActivateInterface<IX3DNative>("EngineRenderer");

            db = library.x3d_new_camera(fovy, z_near, z_far, aperture, shutter_speed, iso, position); //, rotation
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
