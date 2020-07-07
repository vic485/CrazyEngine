in vec3 v_color;
smooth in vec3 v_normal;
in vec3 v_wpos;
in vec3 v_uv;

out vec3 frag_color;

const float PI = 3.14159265359; //From google

//Testing values
const vec3 lightDir     = vec3(-1.0,-1.0,-1.0);
const vec3 lightColor   = vec3( 1.0, 1.0, 1.0);
const vec3 diffuseColor = vec3( 1.0, 1.0, 1.0);

uniform vec3 viewDirection;

struct Camera {
    vec3 position;
    vec3 settings;
};

struct Material {
    sampler2D main;

    vec4 albedo;
    float metalness;
    float roughness;
};

struct DirectionalLight {
    vec3 lightDir;
    float intensity; //TODO: Figure out how to convert between real-life units and this imaginary unit the lighting model uses.
    vec3 lightColor;
};

struct PointLight {
    vec3 lightPos;
    float intensity; //TODO: See DirectionalLight's todo
    vec3 lightColor;
};

uniform Camera camera;

uniform DirectionalLight lights_dir[32];
uniform int lights_dir_count;

uniform PointLight lights_point[128];
uniform int lights_point_count;

uniform Material material;

////////////////////////////////////////////////////////////////////////////////
//BRDF
////////////////////////////////////////////////////////////////////////////////
//NOTES
//Wi = normalize(lightDir);
//cosTheta = max(dot(normal, Wi), 0.0);
//attenuation = intensity of light at location (basically falloff)
//radiance = lightColor * attenuation * cosTheta;

//Derived from: https://learnopengl.com/PBR/Theory
//a is roughness * roughness
float DistributionGGX(vec3 N, vec3 H, float a) {
    float a2 = a * a;
    float NdotH = max(dot(N, H), 0.0);
    float NdotH2 = NdotH * NdotH;

    float nom = a2;
    float denom = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;

    return nom / denom;
}

float GeometrySchlickGGX(float NdotV, float k) {
    float nom = NdotV;
    float denom = NdotV * (1.0 - k) + k;

    return nom / denom;
}

//k is a remapping of roughness based on whether we're using the geometry function for either direct lighting or IBL lighting
float GeometrySmith(vec3 N, vec3 V, vec3 L, float k) {
    float NdotV = max(dot(N, V), 0.0);
    float NdotL = max(dot(N, L), 0.0);
    float ggx1 = GeometrySchlickGGX(NdotV, k);
    float ggx2 = GeometrySchlickGGX(NdotL, k);

    return ggx1 * ggx2;
}

vec3 fresnelSchlick(float cosTheta, vec3 F0) {
    return max(F0 + (1.0 - F0) * pow(1.0 - cosTheta, 5.0), 0.0);
}

vec3 LightingModel(Material material, vec3 N, vec3 V) {
    vec3 Lo = vec3(0.0);

    vec3 base_color = material.albedo.rgb * texture2D(material.main, v_uv.xy * vec2(1.0, -1.0)).rgb;

    float roughness_sqr = material.roughness * material.roughness;

    //Do lighting stuff
    //Directional lights
    for (int i = 0; i < 32; i++) {
        if (i >= lights_dir_count) break;

        vec3 L = normalize(-lights_dir[i].lightDir);
        vec3 H = normalize(V + L);

        // float attenuation = 1.0; //No falloff
        vec3 radiance = lightColor * lights_dir[i].intensity;

        vec3 F0 = vec3(0.04);
        F0 = mix(F0, base_color, material.metalness);
        vec3 F = fresnelSchlick(clamp(dot(H, V), 0.0, 1.0), F0); //It seems for IBL, we would need to replace dot(H, V) with dot(N, V)

        float NDF = DistributionGGX(N, H, roughness_sqr);

        float r = material.roughness + 1.0;
        float k = (r * r) / 8.0;
        float G = GeometrySmith(N, V, L, k);

        vec3 numerator = NDF * G * F;
        float denominator = 4.0 * max(dot(N, V), 0.0) * max(dot(N, L), 0.0);
        vec3 specular = numerator / max(denominator, 0.001);

        vec3 kS = F;
        vec3 kD = vec3(1.0) - kS;
        kD *= 1.0 - material.metalness;

        float NdotL = max(dot(N, L), 0.0);
        Lo += (kD * base_color / PI + specular) * radiance * NdotL;
    }

    //Point lights
    for (int i = 0; i < 128; i++) {
        if (i >= lights_point_count) break;

        // vec3 L = normalize(-lights_dir[i].lightDir);
        float dist = distance(lights_point[i].lightPos, v_wpos);
        vec3 L = normalize(lights_point[i].lightPos - v_wpos);
        vec3 H = normalize(V + L);

        float attenuation = 1.0 / (dist * dist); //No falloff
        vec3 radiance = lightColor * attenuation * lights_point[i].intensity;

        vec3 F0 = vec3(0.04); //Adds some ambient occlusion
        F0 = mix(F0, base_color, material.metalness);
        vec3 F = fresnelSchlick(clamp(dot(H, V), 0.0, 1.0), F0); //It seems for IBL, we would need to replace dot(H, V) with dot(N, V)

        float NDF = DistributionGGX(N, H, roughness_sqr);

        float r = material.roughness + 1.0;
        float k = (r * r) / 8.0;
        float G = GeometrySmith(N, V, L, k);

        vec3 numerator = NDF * G * F;
        float denominator = 4.0 * max(dot(N, V), 0.0) * max(dot(N, L), 0.0);
        vec3 specular = numerator / max(denominator, 0.001);

        vec3 kS = F;
        vec3 kD = vec3(1.0) - kS;
        kD *= 1.0 - material.metalness;

        float NdotL = max(dot(N, L), 0.0);
        Lo += (kD * base_color / PI + specular) * radiance * NdotL;
    }

    return Lo;
}

//From: https://placeholderart.wordpress.com/2014/11/21/implementing-a-physically-based-camera-manual-exposure/
float getSaturationBasedExposure(float aperture, float shutterSpeed, float iso) {
    float l_max = (7800.0 / 65.0) * sqrt(aperture) / (iso * shutterSpeed);
    return 1.0 / l_max;
}

void main() {
    // vec2 f_uv = vec2(v_uv.x, -v_uv.y);
    // frag_color = v_color * texture2D(main_texture, f_uv).rgb;

    vec3 N = normalize(v_normal);
    vec3 V = -normalize(v_wpos - camera.position);

    vec3 color = LightingModel(material, N, V);

    float aperture = camera.settings.x;
    float shutterSpeed = camera.settings.y;
    float iso = camera.settings.z;
    color *= getSaturationBasedExposure(aperture, shutterSpeed, iso);

    //We assume that all our textures are read in linear space (as OpenGL supposedly automatically converts gamma-space images to linear-space)
    //Therefore, after applying all our lighting effects, we should gamma correct our final result.
    frag_color = pow(color, vec3(0.4545));
}
