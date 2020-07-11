using System;
using System.Runtime.InteropServices;

namespace EngineCore.Types.Rust
{
    /// Math done according to the GLSL spec, to make it easier to use for
    /// rendering purposes. This means that adding a scalar to a vector
    /// will add the scalar to each component of the vector.
    /// Same goes for subtraction and other common operations.
    /// NOTE: See [https://www.khronos.org/registry/OpenGL/specs/gl/GLSLangSpec.4.60.pdf](GLSL specs 4.6)
    ///       Chapter 5, specifically subchapter 5.9 "Expressions"

    #region Vectors

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

        public static Vector2 operator +(Vector2 a, Vector2 b) => new Vector2(a.x + b.x, a.y + b.y);
        public static Vector2 operator +(Vector2 a,   float b) => new Vector2(a.x + b,   a.y + b  );
        public static Vector2 operator +(float a,   Vector2 b) => new Vector2(  a + b.x,   a + b.y);
        public static Vector2 operator -(Vector2 a, Vector2 b) => new Vector2(a.x - b.x, a.y - b.y);
        public static Vector2 operator -(Vector2 a,   float b) => new Vector2(a.x - b,   a.y - b  );
        public static Vector2 operator -(float a,   Vector2 b) => new Vector2(  a - b.x,   a - b.y);

        public static Vector2 operator /(Vector2 a, Vector2 b) => new Vector2(a.x / b.x, a.y / b.y);
        public static Vector2 operator /(Vector2 a,   float b) => new Vector2(a.x / b,   a.y / b  );
        public static Vector2 operator /(float a,   Vector2 b) => new Vector2(  a / b.x,   a / b.y);
        public static Vector2 operator *(Vector2 a, Vector2 b) => new Vector2(a.x * b.x, a.y * b.y);
        public static Vector2 operator *(Vector2 a,   float b) => new Vector2(a.x * b,   a.y * b  );
        public static Vector2 operator *(float a,   Vector2 b) => new Vector2(  a * b.x,   a * b.y);

        public static Vector2 operator %(Vector2 a, Vector2 b) => new Vector2(a.x % b.x, a.y % b.y);
        public static Vector2 operator %(Vector2 a,   float b) => new Vector2(a.x % b,   a.y % b  );
        public static Vector2 operator %(float a,   Vector2 b) => new Vector2(  a % b.x,   a % b.y);
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

        public static Vector3 operator +(Vector3 a, Vector3 b) => new Vector3(a.x + b.x, a.y + b.y, a.z + b.z);
        public static Vector3 operator +(Vector3 a,   float b) => new Vector3(a.x + b,   a.y + b,   a.z + b  );
        public static Vector3 operator +(float a,   Vector3 b) => new Vector3(  a + b.x,   a + b.y,   a + b.z);
        public static Vector3 operator -(Vector3 a, Vector3 b) => new Vector3(a.x - b.x, a.y - b.y, a.z - b.z);
        public static Vector3 operator -(Vector3 a,   float b) => new Vector3(a.x - b,   a.y - b,   a.z - b  );
        public static Vector3 operator -(float a,   Vector3 b) => new Vector3(  a - b.x,   a - b.y,   a - b.z);

        public static Vector3 operator /(Vector3 a, Vector3 b) => new Vector3(a.x / b.x, a.y / b.y, a.z / b.z);
        public static Vector3 operator /(Vector3 a,   float b) => new Vector3(a.x / b,   a.y / b,   a.z / b  );
        public static Vector3 operator /(float a,   Vector3 b) => new Vector3(  a / b.x,   a / b.y,   a / b.z);
        public static Vector3 operator *(Vector3 a, Vector3 b) => new Vector3(a.x * b.x, a.y * b.y, a.z * b.z);
        public static Vector3 operator *(Vector3 a,   float b) => new Vector3(a.x * b,   a.y * b,   a.z * b  );
        public static Vector3 operator *(float a,   Vector3 b) => new Vector3(  a * b.x,   a * b.y,   a * b.z);

        public static Vector3 operator %(Vector3 a, Vector3 b) => new Vector3(a.x % b.x, a.y % b.y, a.z % b.z);
        public static Vector3 operator %(Vector3 a,   float b) => new Vector3(a.x % b,   a.y % b,   a.z % b  );
        public static Vector3 operator %(float a,   Vector3 b) => new Vector3(  a % b.x,   a % b.y,   a % b.z);
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

        public static Vector4 operator +(Vector4 a, Vector4 b) => new Vector4(a.x + b.x, a.y + b.y, a.z + b.z, a.w + b.w);
        public static Vector4 operator +(Vector4 a,   float b) => new Vector4(a.x + b,   a.y + b,   a.z + b,   a.w + b  );
        public static Vector4 operator +(float a,   Vector4 b) => new Vector4(  a + b.x,   a + b.y,   a + b.z,   a + b.w);
        public static Vector4 operator -(Vector4 a, Vector4 b) => new Vector4(a.x - b.x, a.y - b.y, a.z - b.z, a.w - b.w);
        public static Vector4 operator -(Vector4 a,   float b) => new Vector4(a.x - b,   a.y - b,   a.z - b,   a.w - b  );
        public static Vector4 operator -(float a,   Vector4 b) => new Vector4(  a - b.x,   a - b.y,   a - b.z,   a - b.w);

        public static Vector4 operator /(Vector4 a, Vector4 b) => new Vector4(a.x / b.x, a.y / b.y, a.z / b.z, a.w / b.w);
        public static Vector4 operator /(Vector4 a,   float b) => new Vector4(a.x / b,   a.y / b,   a.z / b,   a.w / b  );
        public static Vector4 operator /(float a,   Vector4 b) => new Vector4(  a / b.x,   a / b.y,   a / b.z,   a / b.w);
        public static Vector4 operator *(Vector4 a, Vector4 b) => new Vector4(a.x * b.x, a.y * b.y, a.z * b.z, a.w * b.w);
        public static Vector4 operator *(Vector4 a,   float b) => new Vector4(a.x * b,   a.y * b,   a.z * b,   a.w * b  );
        public static Vector4 operator *(float a,   Vector4 b) => new Vector4(  a * b.x,   a * b.y,   a * b.z,   a * b.w);

        public static Vector4 operator %(Vector4 a, Vector4 b) => new Vector4(a.x % b.x, a.y % b.y, a.z % b.z, a.w % b.w);
        public static Vector4 operator %(Vector4 a,   float b) => new Vector4(a.x % b,   a.y % b,   a.z % b,   a.w % b  );
        public static Vector4 operator %(float a,   Vector4 b) => new Vector4(  a % b.x,   a % b.y,   a % b.z,   a % b.w);
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
