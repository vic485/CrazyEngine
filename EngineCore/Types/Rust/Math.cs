using System;
using System.Runtime.InteropServices;

namespace EngineCore.Types.Rust
{
    #region Vectors

    /// <summary>
    /// Vector 1, maps to cgmath::Vector1<f32> in rust
    /// </summary>
    [StructLayout(LayoutKind.Sequential)]
    public struct Vector1 {
        public float x;

        public Vector1(float x) {
            this.x = x;
        }
    }

    /// <summary>
    /// Vector 2, maps to cgmath::Vector2<f32> in rust
    /// </summary>
    [StructLayout(LayoutKind.Sequential)]
    public struct Vector2 {
        public float x;
        public float y;

        public Vector2(float x, float y) {
            this.x = x;
            this.y = y;
        }
    }

    /// <summary>
    /// Vector 3, maps to cgmath::Vector3<f32> in rust
    /// </summary>
    [StructLayout(LayoutKind.Sequential)]
    public struct Vector3 {
        public float x;
        public float y;
        public float z;

        public Vector3(float x, float y, float z) {
            this.x = x;
            this.y = y;
            this.z = z;
        }
    }

    /// <summary>
    /// Vector 4, maps to cgmath::Vector4<f32> in rust
    /// </summary>
    [StructLayout(LayoutKind.Sequential)]
    public struct Vector4 {
        public float x;
        public float y;
        public float z;
        public float w;

        public Vector4(float x, float y, float z, float w) {
            this.x = x;
            this.y = y;
            this.z = z;
            this.w = w;
        }
    }

    #endregion

    #region Quaternion

    /// <summary>
    /// Quaternion, maps to cgmath::Quaternion<f32> in rust
    /// </summary>
    [StructLayout(LayoutKind.Sequential)]
    public struct Quaternion {
        public float s;
        public Vector3 v;
    }

    #endregion
}
