using System;
using System.IO;
using System.Runtime.InteropServices;
using EngineCore.Types;

namespace EngineCore
{
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
            RegisterErrorMessage(ptr);
            RegisterWarningMessage(ptr);
            RegisterLogMessage(ptr);
            RegisterDebugMessage(ptr);
            RegisterTraceMessage(ptr);
        }

        public static void Log(object message)
        {
            Console.WriteLine(message);
            WriteToFile(message.ToString());
        }

        private static void WriteToFile(string message)
        {
            using var writer = File.AppendText(_file);
            writer.WriteLine(message);
        }

        #region Delegates

        private delegate void RustLogDel(IntPtr rs);

        private static void RustLog(IntPtr rs)
        {
            Log(new RustString(rs));
        }

        #endregion

        #region Rust imports

        [DllImport("EngineRenderer", EntryPoint = "register_logger", CallingConvention = CallingConvention.Cdecl)]
        private static extern void RegisterLogger();

        [DllImport("EngineRenderer", EntryPoint = "logger_register_error", CallingConvention = CallingConvention.Cdecl)]
        private static extern void RegisterErrorMessage(IntPtr funcPtr);

        [DllImport("EngineRenderer", EntryPoint = "logger_register_warn", CallingConvention = CallingConvention.Cdecl)]
        private static extern void RegisterWarningMessage(IntPtr funcPtr);

        [DllImport("EngineRenderer", EntryPoint = "logger_register_info", CallingConvention = CallingConvention.Cdecl)]
        private static extern void RegisterLogMessage(IntPtr funcPtr);

        [DllImport("EngineRenderer", EntryPoint = "logger_register_debug", CallingConvention = CallingConvention.Cdecl)]
        private static extern void RegisterDebugMessage(IntPtr funcPtr);

        [DllImport("EngineRenderer", EntryPoint = "logger_register_trace", CallingConvention = CallingConvention.Cdecl)]
        private static extern void RegisterTraceMessage(IntPtr funcPtr);

        #endregion
    }
}
