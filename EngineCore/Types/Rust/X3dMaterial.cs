using System;
using System.Runtime.InteropServices;
using EngineCore.Interfaces;
using AdvancedDLSupport;

namespace EngineCore.Types.Rust
{
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
                var NativeLibraryBuilder = new NativeLibraryBuilder();
                IX3DNative library = NativeLibraryBuilder.Default.ActivateInterface<IX3DNative>("EngineRenderer");

                library.x3d_drop_renderer(ref handle);

                // X3DMaterialNative.CleanupX3DMaterial(handle);
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

        private NativeLibraryBuilder nativeLibrary = new NativeLibraryBuilder();
        private IX3DNative library;

        public X3DMaterial(X3DShader shader)
        {
            library = nativeLibrary.ActivateInterface<IX3DNative>("EngineRenderer");

            db = library.x3d_new_material(shader.GetHandle());
            // db = X3DMaterialNative.CreateX3DMaterial(shader.GetHandle());
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
