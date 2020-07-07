using System;
using System.Runtime.InteropServices;

namespace EngineCore.Types
{
    [StructLayout(LayoutKind.Sequential)]
    public struct Surface
    {
        public IntPtr window_size;
        public IntPtr gfx_state;
    }
}
