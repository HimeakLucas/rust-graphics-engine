#version 330 core

out vec4 FragColor;

in vec3 Normal;
in vec3 FragPos;

uniform vec3 lightPos;
uniform vec3 viewPos; //posição da camera
uniform vec3 lightColor;
uniform vec3 objectColor;

void main() {

	//ambient light
	float ambientStrenght = 0.1;
	vec3 ambient = ambientStrenght * lightColor;

	//diffuse light
	vec3 norm = normalize(Normal);
	vec3 lightDir = normalize(lightPos - FragPos);
	float diff = max(dot(norm, lightDir), 0.0); //max garante que não tenhamos valores negativos
	vec3 diffuse = diff * lightColor;

	//Specular light
	float specularStrenght = 1.5;
	vec3 viewDir = normalize(viewPos - FragPos);
	
	vec3 I = lightDir; //Vetor incidente
	vec3 reflectDir = I - 2.0 * dot(norm, I) * norm;
	//vec3 reflectDir = reflect(-light, norm)

	float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32 ); //32 = Shininess)
	vec3 specular = specularStrenght * spec * lightColor;

	vec3 result = (ambient + diffuse + specular) * objectColor;
	FragColor = vec4(result, 1.0);
}
