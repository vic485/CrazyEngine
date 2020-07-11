using System;
using System.Runtime.InteropServices;
using EngineCore.Types.Rust;
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
        private static DateTime _lastFrame = DateTime.Now;

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
                var currentFrame = DateTime.Now;
                var deltaTime = (currentFrame - _lastFrame).TotalSeconds;
                var deltaTimeMs = (currentFrame - _lastFrame).TotalMilliseconds;
                _mainWindow.UpdateTitle($"Crazy Engine - {deltaTimeMs} ms -- FPS: {1.0 / deltaTime}");
                // TestRender();
                _mainWindow.Update();
                _lastFrame = currentFrame;
            }

            _mainWindow.Dispose();
        }

        [DllImport("EngineRenderer", EntryPoint = "error_handling_test", CallingConvention = CallingConvention.Cdecl)]
        private static extern void TestError();
    }
}
