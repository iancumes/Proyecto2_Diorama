# Proyecto 2 - Diorama (Ray Tracing)

Este proyecto es un Ray Tracer desarrollado en Rust que renderiza una escena 3D con materiales realistas, iluminación, sombras, reflejos y refracciones.

## 📋 Descripción

Un motor de Ray Tracing que implementa:

- **Geometrías**: Esferas, cubos, planos
- **Materiales avanzados**: 
  - Texturas (dirt, grass, stone, water, glass, metal)
  - Reflexión y refracción
  - Efecto Fresnel
  - Modelo de iluminación Phong
- **Características visuales**:
  - Skybox procedural (día/noche)
  - Sombras suaves
  - Gamma correction
  - Depth of Field (Profundidad de campo)
  - Post-procesamiento (opcional)
- **Renderizado interactivo**: Visualización en tiempo real con Raylib

## 🛠️ Requisitos

- **Rust** (edición 2024 o superior)
- **Cargo** (gestor de paquetes de Rust)
- **Dependencias del sistema** (para Raylib):
  - Linux: `libx11-dev`, `libxrandr-dev`, `libxi-dev`, `libgl1-mesa-dev`, `libasound2-dev`
  - Windows: No requiere dependencias adicionales
  - macOS: No requiere dependencias adicionales

### Instalar dependencias en Linux (Ubuntu/Debian)

```bash
sudo apt-get update
sudo apt-get install -y libx11-dev libxrandr-dev libxi-dev libgl1-mesa-dev libasound2-dev
```

## 🚀 Compilación y Ejecución

### Modo Debug (más rápido de compilar, más lento de ejecutar)

```bash
cargo build
cargo run
```

### Modo Release (recomendado para mejor rendimiento)

```bash
cargo build --release
cargo run --release
```

El modo release activa optimizaciones agresivas:
- `opt-level = 3`: Máxima optimización
- `lto = true`: Link-Time Optimization
- `codegen-units = 1`: Mejor optimización del código

## 🎮 Controles

Una vez ejecutado el programa, puedes interactuar con la escena:

- **Tecla N**: Alternar entre modo día/noche (cambia el skybox y la iluminación)
- **Tecla P**: Activar/desactivar post-procesamiento
- **Tecla ESC**: Cerrar la aplicación

## 📁 Estructura del Proyecto

```
Proyecto2_Diorama/
├── src/
│   ├── main.rs          # Punto de entrada, ray tracing y render loop
│   ├── vec3.rs          # Operaciones vectoriales 3D
│   ├── ray.rs           # Estructura de rayos
│   ├── camera.rs        # Sistema de cámara
│   ├── geometry.rs      # Primitivas geométricas (esferas, cubos, planos)
│   ├── material.rs      # Definición de materiales
│   ├── texture.rs       # Sistema de texturas y atlas
│   ├── scene.rs         # Configuración de la escena
│   ├── skybox.rs        # Skybox procedural
│   └── transform.rs     # Transformaciones 3D
├── assets/              # Texturas PNG
│   ├── dirt.png
│   ├── grass.png
│   ├── stone.png
│   ├── water.png
│   ├── glass.png
│   ├── metal.png
│   └── metal2.png
├── Cargo.toml           # Configuración del proyecto
└── README.md            # Este archivo
```

## 🎨 Características Técnicas

### Ray Tracing

- **Recursión**: Soporta múltiples rebotes de rayos para reflejos y refracciones
- **Anti-aliasing**: Utiliza un buffer de acumulación para suavizar los bordes
- **Depth of Field**: Simula el desenfoque de la cámara basado en la profundidad

### Iluminación

- **Modelo Phong**: Componentes ambiental, difusa y especular
- **Sombras**: Ray tracing de sombras con detección de oclusión
- **Fresnel**: Efecto realista en superficies reflectantes y transparentes
- **Colores de luz**: Iluminación con color para efectos día/noche

### Materiales

Cada material puede tener:
- Textura base (color o imagen)
- Albedo (factor difuso)
- Especular y shininess
- Reflexividad
- Transparencia e índice de refracción

## 📊 Rendimiento

El proyecto está optimizado para renderizado en tiempo real:
- Resolución por defecto: 800x600
- El modo release es **significativamente más rápido** que debug
- La escena se puede modificar en `src/scene.rs`

## 🔧 Personalización

### Modificar la escena

Edita el archivo `src/scene.rs` para:
- Agregar/quitar objetos
- Cambiar posiciones y tamaños
- Modificar materiales
- Ajustar la iluminación

### Agregar texturas

1. Coloca archivos PNG en la carpeta `assets/`
2. Registra la textura en `src/texture.rs`
3. Asígnala a un material en `src/scene.rs`

## 📝 Notas

- El proyecto utiliza **Raylib** para la ventana y renderizado
- Las texturas se cargan en un atlas para mejor rendimiento
- El skybox es procedural (no requiere imágenes)
- El buffer de profundidad se utiliza para efectos de post-procesamiento

## 🐛 Solución de Problemas

### Error de compilación de Raylib

Si tienes problemas compilando Raylib, asegúrate de tener instaladas todas las dependencias del sistema mencionadas en la sección de requisitos.

### Rendimiento bajo

- Asegúrate de estar ejecutando en modo `--release`
- Reduce la resolución en `src/main.rs` (variables `width` y `height`)
- Reduce la profundidad de recursión del ray tracing

### Texturas no se cargan

Verifica que la carpeta `assets/` existe y contiene los archivos PNG necesarios.

## 👨‍💻 Autor

Proyecto desarrollado como parte del curso de Gráficas por Computadora.

## 📄 Licencia

Este proyecto es de uso educativo.
