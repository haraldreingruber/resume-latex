# Harald Reingruber

**Medical 3D Visualization Expert · Rust & WebAssembly**

Scharnstein, Upper Austria, Austria · [GitHub](https://github.com/haraldreingruber-dedalus) · [LinkedIn](https://www.linkedin.com/in/haraldreingruber)

## Summary

Specialist in medical 3D visualization, real-time graphics, and augmented reality, with 15+ years across medical imaging, AR/VR SDKs, and web-based 3D streaming. Currently leading a Rust/WebAssembly modernization of a legacy C++ visualization engine. Pragmatic about trade-offs, balancing product delivery with long-term quality and maintainability through clean code, TDD, and continuous improvement.

## Experience

| Period | Role | Company |
| --- | --- | --- |
| 11/2020 – present | Medical 3D Visualization Expert (Rust/TypeScript) | Dedalus HealthCare (remote) |
| 11/2019 – 05/2020 | Pair & Mob-Programming Tour (apprenticeship) | Self-employed (remote, worldwide) |
| 09/2016 – 10/2019 | Software Engineer,  Unity 3D/Node.js/React/TypeScript | three10, Vienna |
| 04/2015 – 08/2016 | Software Engineer, Augmented Reality (C++/Objective-C/JavaScript) | ViewAR, Vienna |
| 10/2013 – 04/2015 | 3D Volume Rendering Engineer (C++) | Agfa HealthCare, Vienna |
| 09/2010 – 09/2013 | Software Engineer, 3D Visualization (Java) | Agfa HealthCare, Vienna |

**Highlights**

- **Dedalus:** Migrated the medical 3D visualization module from C++/DirectX 9 to Rust + WebAssembly + WebGL/OpenGL and TypeScript. Repackaged it as a framework-agnostic Web Component that ships in two host products (Angular and React). Built a side-by-side comparison system to validate performance and visuals during the migration, and added component-level E2E tests with Storybook and Playwright.
- **Mob-programming tour:** Practiced TDD and pair/mob programming with 9 teams in 4 countries. Co-organized the Mob-Programming on Open Source Software meetup in Austria.
- **three10:** One of two principal engineers in a team of 6. Built a JavaScript library that streams server-side Unity 3D graphics to the browser over a low-latency video codec. Owned the Docker/Node.js/React backend architecture and secure file transfer.
- **ViewAR:** Extended the C++ AR core and integrated third-party AR/VR SDKs. Generated watertight meshes and volume estimates from depth-sensor scans for a Lufthansa Cargo pallet-volume system. Cross-compiled the core to WebGL with Emscripten/WebAssembly.
- **Agfa HealthCare:** Main engineer for the GPU/CPU volume rendering engine, including SIMD-optimized CPU rendering. Implemented progressive loading of volume image data (patent granted) and bi-cubic filtering for WebGL resampling. Replaced per-platform build scripts with CMake.

## Selected projects & research

- **Master's thesis, Austrian Institute of Technology (2009–2010):** *An Asynchronous Data Interface for Event-based Stereo Vision Systems*. Improved 3D reconstruction accuracy by adjusting pixel retention based on event density.
- **Final-year project, Computer Vision Center Barcelona (2009):** *Intestinal Content Detection in Capsule Endoscopy using Robust Features*. Built an SVM detector that reached 71% accuracy.

## Skills

- **Languages:** Rust, TypeScript/JavaScript, C++, C#, Java
- **Graphics & 3D:** WebGL/OpenGL, WebAssembly/Emscripten, Unity 3D
- **Imaging & vision:** OpenCV, SIMD (SSE/AVX/Neon), image processing
- **Web & tooling:** Node.js, Web Components, Playwright, Storybook, Docker
- **Practices:** Unit testing & TDD, Agile (Scrum/Kanban), continuous improvement, clean code, AI-driven development (Copilot, Claude Code, OpenSpec)

## Education

- **MSc Computer Science, Visual Computing:** Vienna University of Technology (2007–2011)
- **Exchange program:** Universitat Autònoma de Barcelona (2009)
- **BSc Computer Science, Medical Software Engineering:** Upper Austria University of Applied Sciences, Hagenberg (2005–2007)

## Languages

German (native) · English (fluent) · Spanish (conversational)

---

The full resume is written in LaTeX ([main.tex](main.tex), AltaCV template) and builds with `pdflatex` (see [build.sh](build.sh)).
