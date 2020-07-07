using System;
using System.Runtime.InteropServices;
using EngineCore.Types;
using GLFW;

namespace EngineCore
{
    internal static class Program
    {
        private static GlfwWindow _mainWindow;

        private static void Main(string[] args)
        {
            _mainWindow = new GlfwWindow();

            while (!_mainWindow.Closed())
            {
                TestRender();
                _mainWindow.Update();
            }
            
            _mainWindow.Dispose();
        }

        [DllImport("EngineRenderer.dll", EntryPoint = "test_render", CallingConvention = CallingConvention.Cdecl)]
        private static extern void TestRender();
    }
}
