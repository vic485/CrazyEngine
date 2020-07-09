using System;
using System.IO;
using System.Runtime.InteropServices;
using EngineCore.Types;

namespace EngineCore
{
    [StructLayout(LayoutKind.Sequential)]
    public struct RustLogMessage {
        public int level;
        public IntPtr message;
        public IntPtr target;
    }

    public static class Debug
    {
        public static readonly string _file;

        static Debug()
        {
            var fileName = $"log_{DateTime.Now:yyyy-MM-dd_HH-mm}.txt";
            // TODO: make a logs dir, or store in AppData
            _file = Path.Combine(AppDomain.CurrentDomain.BaseDirectory, fileName);

            // Setup rust callbacks
            RegisterLogger();

            // Log
            RustLogDel del = RustLog;
            var ptr = Marshal.GetFunctionPointerForDelegate(del);
            // TODO: I was lazy
            RegisterLogMessage(ptr);
            // RegisterErrorMessage(ptr);
            // RegisterWarningMessage(ptr);
            // RegisterLogMessage(ptr);
            // RegisterDebugMessage(ptr);
            // RegisterTraceMessage(ptr);
        }

        public static void Log(object message)
        {
            Console.WriteLine(message.ToString());
            WriteToFile(message.ToString());
        }

        private static void WriteToFile(string message)
        {
            using var writer = File.AppendText(_file);
            writer.WriteLine(message);
        }

        #region Delegates

        private delegate void RustLogDel(RustLogMessage rs);

        private static void RustLog(RustLogMessage rs)
        {
            RustString message = new RustString(rs.message);
            RustString target = new RustString(rs.target);
            Log(target.ToString() + " > " + message.ToString());
        }

        #endregion

        #region Rust imports

        [DllImport("EngineRenderer", EntryPoint = "register_logger", CallingConvention = CallingConvention.Cdecl)]
        private static extern void RegisterLogger();

        [DllImport("EngineRenderer", EntryPoint = "logger_register_func", CallingConvention = CallingConvention.Cdecl)]
        private static extern void RegisterLogMessage(IntPtr funcPtr);

        #endregion
    }
}
