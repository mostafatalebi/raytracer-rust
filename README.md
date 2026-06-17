## 3d RayTracer

[WIP] A 3d Raytracer Render Engine (CPU) that is written entirely in Rust. It is a work in progress,
to explore it, you can see the `/rt/tests/`.

[![sample_scene.jpg](https://i.postimg.cc/CLnp44J1/scene-room-0019.jpg)](https://postimg.cc/D8hNvL9k)


**Features:**
- BVH tree; uses Bounding Volume Hierarchy to gain better performance
- Multi-threaded rendering
- Raytracer Shadows
- Soft shadows (Area light)
- Supported Lights: Point light, ambient light, area light
- Face and vertex normals (vn is used by default for smoother shading)
- Lambert shader
- Blinn-Phong shader
- [WIP] Basic procedural textures (checkered for now)
- Anti Aliasing added (Uniform and Monte-Carlo methods)
- [WIP] Wireframe Shader
- Regional render (rendering a part of the main render frame)
- OBJ file import (needs completion but it is working now for a full obj file)
- [WIP] JSON scene definition 


**Planned Features:**
- Soft Shadows and Refraction
- Background Sky
- Direction light
- Spot Light
- UV Mapping
