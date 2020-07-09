using System;
using System.Runtime.InteropServices;
using EngineCore.Types;
using EngineCore.Vfs;
using GLFW;

namespace EngineCore
{
    public static class Native
    {
        [DllImport("EngineRenderer", EntryPoint = "last_error_message", CallingConvention = CallingConvention.Cdecl)]
        public static extern RustError LastErrorMessage();
    }

    internal static class Program
    {
        private static GlfwWindow _mainWindow;

        private static void Main(string[] args)
        {
            FileLoader.TestPath();
            Debug.Log("Opening window");
            _mainWindow = new GlfwWindow();

            TestError();

            RustError err = Native.LastErrorMessage();
            RustString message = new RustString(err.message);
            Console.WriteLine(message.AsString());

            while (!_mainWindow.Closed())
            {
                // TestRender();
                _mainWindow.Update();
            }

            _mainWindow.Dispose();
        }

        [DllImport("EngineRenderer", EntryPoint = "error_handling_test", CallingConvention = CallingConvention.Cdecl)]
        private static extern void TestError();
    }
}
