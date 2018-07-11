// Copyright 2018 Joshua Minter
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#version 150
in vec2 point;
in vec2 texcoord;
in vec4 color;

out vec2 frag_texcoord;
out vec4 frag_color;

void main() {
	frag_texcoord = texcoord;
	frag_color = color;
	gl_Position = point;
}