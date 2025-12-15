#version 330 core
out vec4 FragColor;

struct Material {
    vec3 ambient;       // Usado se for cor sólida
    vec3 diffuse;       // Usado se for cor sólida
    sampler2D texture_diffuse; // Usado se for textura
    int use_texture;    // 1 = Sim, 0 = Não
    
    vec3 specular;
    float shininess;
};

struct Light {
    vec3 position;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};

in vec3 FragPos;
in vec3 Normal;
in vec2 TexCoords;

uniform vec3 viewPos;
uniform Material material;
uniform Light light;

void main() {
    vec3 ambientColor;
    vec3 diffuseColor;

    // A GRANDE DECISÃO
    if (material.use_texture == 1) {
        // Modo Textura: A cor ambiente e difusa vêm da imagem
        ambientColor = vec3(texture(material.texture_diffuse, TexCoords));
        diffuseColor = vec3(texture(material.texture_diffuse, TexCoords));
    } else {
        // Modo Cor Sólida: Usa os vetores que definimos
        ambientColor = material.ambient;
        diffuseColor = material.diffuse;
    }

    // 1. Ambient
    vec3 ambient = light.ambient * ambientColor;
  
    // 2. Diffuse 
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(light.position - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = light.diffuse * (diff * diffuseColor);
    
    // 3. Specular (O brilho funciona igual para os dois)
    vec3 viewDir = normalize(viewPos - FragPos);
    vec3 I = -lightDir; 
    vec3 reflectDir = I - 2.0 * dot(norm, I) * norm;  
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specular = light.specular * (spec * material.specular);  
        
    vec3 result = ambient + diffuse + specular;
    FragColor = vec4(result, 1.0);
}
