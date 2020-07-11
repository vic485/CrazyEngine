using System;
using System.Runtime.InteropServices;
using System.Numerics;

namespace EngineCore.Types.Rust
{
    #region RustVectors

    /// <summary>
    /// RustVector 2, maps to cgmath::Vector2<f32> in rust
    /// </summary>
    [StructLayout(LayoutKind.Sequential)]
    public struct RustVector2 {
        public float x;
        public float y;

        public RustVector2(float a) {
            this.x = a;
            this.y = a;
        }

        public RustVector2(float x, float y) {
            this.x = x;
            this.y = y;
        }

        public RustVector2(Vector2 vec) {
            this.x = vec.x;
            this.y = vec.y;
        }

        public Vector2 GetNative() {
            return new Vector2(x,y);
        }
    }

    /// <summary>
    /// RustVector 3, maps to cgmath::Vector3<f32> in rust
    /// </summary>
    [StructLayout(LayoutKind.Sequential)]
    public struct RustVector3 {
        public float x;
        public float y;
        public float z;

        public RustVector3(float a) {
            this.x = a;
            this.y = a;
            this.z = a;
        }

        public RustVector3(float x, float y, float z) {
            this.x = x;
            this.y = y;
            this.z = z;
        }

        public RustVector3(Vector3 vec) {
            this.x = vec.x;
            this.y = vec.y;
            this.z = vec.z;
        }

        public Vector3 GetNative() {
            return new Vector3(x,y,z);
        }
    }

    /// <summary>
    /// RustVector 4, maps to cgmath::Vector4<f32> in rust
    /// </summary>
    [StructLayout(LayoutKind.Sequential)]
    public struct RustVector4 {
        public float x;
        public float y;
        public float z;
        public float w;

        public RustVector4(float a) {
            this.x = a;
            this.y = a;
            this.z = a;
            this.w = a;
        }

        public RustVector4(float x, float y, float z, float w) {
            this.x = x;
            this.y = y;
            this.z = z;
            this.w = w;
        }

        public RustVector4(Vector4 vec) {
            this.x = vec.x;
            this.y = vec.y;
            this.z = vec.z;
            this.w = vec.w;
        }

        public Vector4 GetNative() {
            return new Vector4(x,y,z,w);
        }
    }

    #endregion

    #region Quaternion

    /// <summary>
    /// Quaternion, maps to cgmath::Quaternion<f32> in rust
    /// </summary>
    [StructLayout(LayoutKind.Sequential)]
    public struct RustQuaternion {
        public float s;
        public RustVector3 v;

        public RustQuaternion(float x, float y, float z, float w) {
            this.s = w;
            this.v = new RustVector3(x,y,z);
        }

        public RustQuaternion(float s, RustVector3 v) {
            this.s = s;
            this.v = v;
        }

        public Quaternion GetNative() {
            return new Quaternion(v.GetNative(), s);
        }
    }

    #endregion
}
