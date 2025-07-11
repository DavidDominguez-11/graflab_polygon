# Lab 1: Filling any polygon

## David Dominguez 23712

## Descripción
Este proyecto implementa algoritmos para el trazado y relleno de polígonos en Rust, utilizando un framebuffer simple y la librería Raylib para la generación de imágenes. El sistema soporta:

- Trazado de líneas con el algoritmo de Bresenham
- Relleno de polígonos simples con el algoritmo scanline
- Relleno de polígonos con agujeros
- Exportación de resultados a archivos BMP

## Características Principales

1. **Algoritmos Implementados**:
   - Algoritmo de Bresenham para trazado de líneas
   - Scanline con parity rule para relleno de polígonos
   - Soporte para polígonos complejos y con agujeros

2. **Formatos Soportados**:
   - Exportación a BMP (24-bit)
   - Resolución configurable (1000x1000 píxeles por defecto)

3. **Personalización**:
   - Colores personalizables para fondo y polígonos
   - Múltiples polígonos en una sola imagen

## Requisitos del Sistema

- Rust
- Cargo
- Librerías del sistema para construcción (build-essential/cmake)
- Raylib

## Instalación

1. Clona el repositorio:
   ```bash
   git clone https://github.com/tu-usuario/graflab_polygon.git
   cd graflab_polygon/polygons_lab
   ```

2. Construye el proyecto:
   ```bash
   cargo build
   ```

## Uso

El programa genera automáticamente 4 imágenes de ejemplo:

```bash
cargo run
```

### Archivos Generados:
1. `poligono1.bmp` - Polígono amarillo (10 lados)
2. `poligono2.bmp` - Polígono azul (4 lados)
3. `poligono3.bmp` - Polígono rojo (triángulo)
4. `poligono4-5.bmp` - Polígono verde con agujero

### Personalización
Modifica `main.rs` para:
- Cambiar colores (constantes `Color::`)
- Añadir nuevos polígonos (arrays de vértices)
- Ajustar tamaño del framebuffer

## Estructura del Código

```
src/
├── framebuffer.rs   # Manejo del buffer de imagen
├── line.rs          # Algoritmo de Bresenham
├── main.rs          # Ejemplos y lógica principal
└── polygon.rs       # Algoritmos de relleno (scanline)
```

