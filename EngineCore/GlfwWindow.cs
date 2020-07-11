using System;
using System.Runtime.InteropServices;
using EngineCore.Types.Rust;
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

        private X3DCamera cam;
        private X3DMesh mesh;
        private X3DMaterial material;

        // For fps testing
        public void UpdateTitle(string title)
        {
            _window.Title = title;
        }

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
            Glfw.SwapInterval(0);
            Glfw.MakeContextCurrent(_window);

            SetupRenderer();
        }

        public bool Closed() => _window.IsClosed;

        public void Update()
        {
            renderer.PrepareFrame();

            renderer.DrawMesh(mesh, cam);
            // renderer.DrawMesh(mesh, cam);

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
            // cam = new X3DCamera();

            RustVector3 cam_pos = new RustVector3(0.0f, 0.0f, 0.0f);
            cam = new X3DCamera(60f, 0.02f, 100f, 16f, 1f / 100f, 100f, cam_pos);
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
