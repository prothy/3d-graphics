#version 300 es

#ifdef GL_ES
precision highp float;
#endif

out vec4 uGlobalColor;

void main() {
  uGlobalColor = vec4(1, 0, 0, 1);
}