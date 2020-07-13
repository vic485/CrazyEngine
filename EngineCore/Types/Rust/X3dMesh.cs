using System;
using System.Runtime.InteropServices;
using EngineCore.Interfaces;
using AdvancedDLSupport;

namespace EngineCore.Types.Rust
{
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
                var NativeLibraryBuilder = new NativeLibraryBuilder();
                IX3DNative library = NativeLibraryBuilder.Default.ActivateInterface<IX3DNative>("EngineRenderer");

                library.x3d_drop_mesh(ref handle);

                // X3DMeshNative.CleanupX3DMesh(handle);
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

        private NativeLibraryBuilder nativeLibrary = new NativeLibraryBuilder();
        private IX3DNative library;

        public X3DMesh(X3DRenderer renderer)
        {
            library = nativeLibrary.ActivateInterface<IX3DNative>("EngineRenderer");

            db = library.x3d_new_mesh();

            // db = X3DMeshNative.CreateX3DMesh(renderer.GetHandle());

            //Check for errors
            RustError err = library.last_error_message();
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
