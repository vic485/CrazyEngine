using System;
using System.Runtime.InteropServices;

namespace EngineCore.Types
{
    [StructLayout(LayoutKind.Sequential)]
    public struct RustError
    {
        public bool err;
        public IntPtr message;
        public int code;
    }
}
