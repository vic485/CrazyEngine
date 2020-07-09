using System;
using System.Runtime.InteropServices;

namespace EngineCore.Types.Rust
{
    [StructLayout(LayoutKind.Sequential)]
    public struct RustError
    {
        public bool err;
        public IntPtr message;
        public int code;
    }
}
