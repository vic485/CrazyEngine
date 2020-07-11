using System;
using System.Runtime.InteropServices;

namespace EngineCore.Types.Rust
{
    internal class X3DShaderNative {
        #region Dll Imports

        [DllImport("EngineRenderer", EntryPoint = "x3d_drop_shader", CallingConvention = CallingConvention.Cdecl)]
        public static extern void CleanupX3DShader(IntPtr objPtr);
        [DllImport("EngineRenderer", EntryPoint = "x3d_new_shader", CallingConvention = CallingConvention.Cdecl)]
        public static extern X3DShaderHandle CreateX3DShader(string vs, string gs, string ts, string fs);

        #endregion
    }

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
                X3DShaderNative.CleanupX3DShader(handle);
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

        public X3DShader(string vs, string gs, string ts, string fs)
        {
            db = X3DShaderNative.CreateX3DShader(vs, gs, ts, fs);

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

        public X3DShaderHandle GetHandle()
        {
            return db;
        }
    }
}
