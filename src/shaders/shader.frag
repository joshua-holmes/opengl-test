#version 330 core

out vec4 final_color;

void main() {
    float lerp_value = gl_FragCoord.y / {WINDOW_HEIGHT}.0f;

    final_color = mix(vec4(1.0f, 1.0f, 1.0f, 1.0f), vec4(0.2f, 0.2f, 0.2f, 1.0f), lerp_value);
}

