using System;
using System.Runtime.InteropServices;
using System.Text;

namespace EngineCore.Types
{
    /// <summary>
    /// Represents rust lang strings
    /// </summary>
    public class RustString : SafeHandle
    {
        #region Dll Imports

        [DllImport("EngineRenderer", EntryPoint = "string_cleanup", CallingConvention = CallingConvention.Cdecl)]
        private static extern void CleanupString(IntPtr stringPtr);

        #endregion

        public RustString() : base(IntPtr.Zero, true)
        {
        }

        public RustString(IntPtr handle) : base(handle, true)
        {
        }

        public RustString ChangeHandle(IntPtr newHandle)
        {
            SetHandle(newHandle);
            return this;
        }

        public string AsString()
        {
            var len = 0;
            while (Marshal.ReadByte(handle, len) != 0)
                ++len;

            var buffer = new byte[len];
            Marshal.Copy(handle, buffer, 0, buffer.Length);
            return Encoding.UTF8.GetString(buffer);
        }

        protected override bool ReleaseHandle()
        {
            if (!IsInvalid)
                CleanupString(handle);

            return true;
        }

        public override bool IsInvalid => handle == IntPtr.Zero;

        public override string ToString()
        {
            return AsString();
        }
    }
}
