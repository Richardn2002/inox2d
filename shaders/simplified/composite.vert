/*
    Copyright © 2020, Inochi2D Project
    Distributed under the 2-Clause BSD License, see LICENSE file.

    Authors: Luna Nielsen and Speykious

    Temporary simplified shaders.
    The goal is to smooth out the transition from simplified shaders to official
    shaders while introducing an MVP.
*/
#version 330
layout(location = 0) in vec2 verts;
layout(location = 1) in vec2 uvs;

out vec2 texUVs;

void main() {
  gl_Position = vec4(verts, 0, 1);
  texUVs = uvs;
}