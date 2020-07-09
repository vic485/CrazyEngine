using System;
using System.Runtime.InteropServices;
using EngineCore.Types;
using GLFW;

namespace EngineCore
{
    /// <summary>
    /// Wrapper for a window using GLFW
    /// </summary>
    public class GlfwWindow : IDisposable
    {
        private readonly NativeWindow _window;

        private X3DRenderer renderer;
        private X3DMesh mesh;

        public GlfwWindow(int width = 1280, int height = 720, string title = "whaddafrig")
        {
            if (!Glfw.Init())
            {
                // TODO: Proper logger
                Console.WriteLine("Failed to initialize GLFW!");
                Environment.Exit(0);
            }

            // TODO: Window hints
            _window = new NativeWindow(width, height, title);
            _window.CenterOnScreen();
            Glfw.MakeContextCurrent(_window);

            SetupRenderer();
        }

        public bool Closed() => _window.IsClosed;

        public void Update()
        {
            renderer.PrepareFrame();

            renderer.DrawMesh(mesh);

            renderer.FinishFrame();

            _window.SwapBuffers();
            Glfw.PollEvents();
        }

        public void Dispose()
        {
            _window?.Dispose();
        }

        private void SetupRenderer()
        {
            RendererDelegate rDel = GetGlProc;
            var ptr = Marshal.GetFunctionPointerForDelegate(rDel);
            RegisterRenderer(ptr);
            //CreateRenderer();
            //Initialize((uint) _window.ClientSize.Width, (uint) _window.ClientSize.Height);
            renderer = new X3DRenderer((uint) _window.ClientSize.Width, (uint) _window.ClientSize.Height);

            //Quickly test the mesh
            mesh = new X3DMesh(renderer);
        }

        private delegate IntPtr RendererDelegate(IntPtr s);

        private static IntPtr GetGlProc(IntPtr s)
        {
            return Glfw.GetProcAddress(new RustString(s).AsString());
        }

        #region Renderer Methods

        [DllImport("EngineRenderer", EntryPoint = "create_gl_load_callback", CallingConvention = CallingConvention.Cdecl)]
        private static extern int RegisterRenderer(IntPtr funcPtr);

        // [DllImport("EngineRenderer", EntryPoint = "initialize", CallingConvention = CallingConvention.Cdecl)]
        // private static extern X3dBinding Initialize(uint x, uint y);

        /*[DllImport("EngineRenderer", EntryPoint = "gl_callback", CallingConvention = CallingConvention.Cdecl)]
        private static extern void CreateRenderer();*/

        #endregion
    }
}
