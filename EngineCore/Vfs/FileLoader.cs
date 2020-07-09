using System;
using System.IO;
using System.Runtime.Serialization.Formatters.Binary;

namespace EngineCore.Vfs
{
    public static class FileLoader
    {
        private static readonly string _basePath;

        static FileLoader()
        {
            _basePath = Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "Data");
            
            // TODO: Only needed now for testing builds
            if (!Directory.Exists(_basePath))
            {
                Console.WriteLine("Data directory was missing. Creating...");
                Directory.CreateDirectory(_basePath);
            }
        }
        
        public static T GetData<T>(string filePath) //where T : new()
        {
            var path = Path.Combine(_basePath, filePath);
            using var file = File.OpenRead(path);
            return (T) new BinaryFormatter().Deserialize(file);
        }
        
        // TODO: Test code
        public static void TestPath()
        {
            Console.WriteLine($"Data path: {_basePath}");
        }
    }
}
