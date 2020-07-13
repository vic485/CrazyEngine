using System;
using System.Runtime.InteropServices;
using AdvancedDLSupport;
using EngineCore.Interfaces;

namespace EngineCore.Types.Rust
{
    /// <summary>
    /// A handle to the raw rust shader object sent through FFI
    /// </summary>
    public class X3DShaderHandle : SafeHandle
    {
        public X3DShaderHandle() : base(IntPtr.Zero, true) {}

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

                library.x3d_drop_shader(handle);
            }

            return true;
        }
    }

    /// <summary>
    /// Represents the 3d shader as it is in rust
    /// </summary>
    public class X3DShader : IDisposable
    {
        private X3DShaderHandle db;

        private NativeLibraryBuilder nativeLibrary = new NativeLibraryBuilder();
        private IX3DNative library;

        public X3DShader(string vs, string gs, string ts, string fs)
        {
            library = nativeLibrary.ActivateInterface<IX3DNative>("EngineRenderer");

            db = library.x3d_new_shader(vs, gs, ts, fs);

            // db = X3DShaderNative.CreateX3DShader(vs, gs, ts, fs);

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

        public X3DShaderHandle GetHandle()
        {
            return db;
        }
    }
}
