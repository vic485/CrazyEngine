//Vertex semantics
in vec3 position;
in vec3 color;
in vec3 normal;
in vec3 uv;

//Shader interface
uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

//Other
out vec3 v_color;
smooth out vec3 v_normal;
out vec3 v_wpos;
out vec3 v_uv;

void main() {
    // simply forward the color
    v_color = color;

    //TODO: Use a proper normal matrix
    mat4 normal_matrix = transpose(inverse(model));
    v_normal = normalize((normal_matrix * vec4(normal, 0.0)).xyz);

    v_uv = uv;
    v_wpos = (model * vec4(position, 1.0)).xyz;

    gl_Position = projection * view * model * vec4(position, 1.0);
}
