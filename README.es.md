# actualizador

Herramienta de mantenimiento y actualización para sistemas Debian, escrita en Rust.

## Descripción

`actualizador` es una utilidad de línea de comandos que automatiza tareas comunes de mantenimiento en sistemas Debian, incluyendo:

- Actualización de paquetes del sistema usando APT
- Limpieza de paquetes no utilizados (`apt autoremove`)
- Actualización y limpieza de Flatpaks
- Actualización de paquetes Snap
- Actualización de toolchains de Rust mediante `rustup`

## Uso

```sh
cargo run --release [--autoremove]
```

- `--autoremove` o `-a`: Ejecuta `apt autoremove` después de actualizar los paquetes del sistema.

### Ejemplo

```sh
cargo run --release -- --autoremove
```

## Requisitos

- Sistema operativo: Debian o derivado
- Rust (para compilar)
- Dependencias:
  - [clap](https://crates.io/crates/clap)
  - [colored](https://crates.io/crates/colored)

## Instalación

1. Clona este repositorio:
   ```sh
   git clone <url-del-repo>
   cd actualizador
   ```
2. Compila el proyecto:
   ```sh
   cargo build --release
   ```
3. Ejecuta el binario generado en `target/release/actualizador`.

## Notas
- El programa detecta automáticamente si tienes instalados `flatpak`, `snap` o `rustup` y ejecuta las acciones correspondientes.
- Se recomienda ejecutarlo con permisos de superusuario para que los comandos de sistema funcionen correctamente.

## Licencia

MIT
