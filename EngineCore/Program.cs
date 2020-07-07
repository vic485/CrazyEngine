using System;
using System.Runtime.InteropServices;
using EngineCore.Types;
using GLFW;

namespace EngineCore
{
    class Program
    {
        delegate IntPtr RendererDelegate(IntPtr s);

        static void Main(string[] args)
        {
            if (!Glfw.Init())
            {
                Console.WriteLine("Glfw failed to init");
                return;
            }

            var window = new NativeWindow();

            RendererDelegate rDel = GetGlProc;
            var ptr = Marshal.GetFunctionPointerForDelegate(rDel);
            RegisterRenderer(ptr);
            CreateRenderer();

            while (!window.IsClosing)
            {
                TestRender();
                window.SwapBuffers();
                Glfw.PollEvents();
            }
        }

        private static IntPtr GetGlProc(IntPtr s)
        {
            //var rs = new RustString().ChangeHandle(s);
            var rs = new RustString(s);
            var text = rs.AsString();
            //Console.WriteLine(text);
            var addr = Glfw.GetProcAddress(text);
            //Console.WriteLine(addr);
            return addr;
        }

        [DllImport("EngineRenderer.dll", EntryPoint = "create_gl_load_callback", CallingConvention = CallingConvention.Cdecl)]
        private static extern int RegisterRenderer(IntPtr funcPtr);

        [DllImport("EngineRenderer.dll", EntryPoint = "gl_callback", CallingConvention = CallingConvention.Cdecl)]
        private static extern void CreateRenderer();

        [DllImport("EngineRenderer.dll", EntryPoint = "test_render", CallingConvention = CallingConvention.Cdecl)]
        private static extern void TestRender();
    }
}
