#version 330

#moj_import <minecraft:fog.glsl>
#moj_import <minecraft:sample_lightmap.glsl>
#moj_import <minecraft:dynamictransforms.glsl>
#moj_import <minecraft:projection.glsl>

in vec3 Position;
in vec4 Color;
in vec2 UV0;
in ivec2 UV2;

uniform sampler2D Sampler2;

out float sphericalVertexDistance;
out float cylindricalVertexDistance;
out vec4 vertexColor;
out vec2 texCoord0;

void main() {
    gl_Position = ProjMat * ModelViewMat * vec4(Position, 1.0);
    texCoord0 = UV0;

    // Default Vanilla Text Rendering (Calculates fog/lighting for both world and GUI text)
    sphericalVertexDistance = fog_spherical_distance(Position);
    cylindricalVertexDistance = fog_cylindrical_distance(Position);
    vertexColor = Color * sample_lightmap(Sampler2, UV2);

    // Runtime GUI Detection: Orthographic projections always have ProjMat[3][3] == 1.0
    bool isGUI = ProjMat[3][3] == 1.0;

    if (isGUI) {
        // Right-anchor hack: Sentinel colour #FE01FE
        if (Color.g < 0.02 && abs(Color.r - Color.b) < 0.02 && Color.r > 0.2) {
            const float K = 100.0; // Right margin in GUI px
            gl_Position.x += 1.0 - ProjMat[0][0] * K;

            if (Color.r > 0.5) {
                // Main glyph: Strip the sentinel tint and output true sprite colour
                vertexColor = vec4(1.0, 1.0, 1.0, Color.a);
            } else {
                // Shadow pass: Hide it
                vertexColor.a = 0.0;
            }
        }
    }
}
