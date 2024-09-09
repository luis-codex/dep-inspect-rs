# pack-analyzer.npmjs

`pack-analyzer.npmjs` es una herramienta para analizar dependencias de paquetes en proyectos de Node.js. Permite obtener información sobre las dependencias de un archivo local, un repositorio remoto de GitHub o paquetes en línea.

## preview photo ./captures/preview.png

![preview](/.captures/capture.png)

## Uso

```sh
./program [opciones]
```

### Opciones

-   `-r, --remote [URL]` Repositorio de GitHub
-   `-f, --file [PATH]` Archivo local
-   `-p, --package [PAQUETES]` Paquete(s) en línea

### Ejemplos

```sh
./program -r usuario/repo/main/package.json
./program -f /ruta/al/package.json
./program -p react express lodash framer-motion motion
```

## Instalación

Para compilar e instalar el proyecto, asegúrate de tener Rust y Cargo instalados en tu sistema. Luego, clona el repositorio y compila el proyecto.

## Construcción

### Construir para Linux

Esto construirá para la arquitectura x86_64.

```sh
cargo build --release --target x86_64-unknown-linux-gnu
```

### Construir para Windows x86_64

Esto construirá para la arquitectura x86_64.

```sh
cargo build --release --target x86_64-pc-windows-gnu
```

### Construir para MacOS

Esto construirá para ambas arquitecturas x86_64 y aarch64.

```sh
cargo build --release --target x86_64-apple-darwin && cargo build --release --target aarch64-apple-darwin
```

## Dependencias

El proyecto utiliza las siguientes dependencias:

-   [`colored`](command:_github.copilot.openSymbolFromReferences?%5B%22%22%2C%5B%7B%22uri%22%3A%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2Fhome%2Fluis%2Fcoding%2Frust%2Fdep-inspect-rs%2Fsrc%2Futils%2Fmod.rs%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%22pos%22%3A%7B%22line%22%3A1%2C%22character%22%3A4%7D%7D%2C%7B%22uri%22%3A%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2Fhome%2Fluis%2Fcoding%2Frust%2Fdep-inspect-rs%2FCargo.toml%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%22pos%22%3A%7B%22line%22%3A6%2C%22character%22%3A0%7D%7D%5D%2C%220e8176d5-e6b2-454e-b5a9-c4c5127df528%22%5D "Go to definition") para colorear la salida en la terminal.
-   [`serde_json`](command:_github.copilot.openSymbolFromReferences?%5B%22%22%2C%5B%7B%22uri%22%3A%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2Fhome%2Fluis%2Fcoding%2Frust%2Fdep-inspect-rs%2Fsrc%2Fcontrollers%2Ffile_remote.rs%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%22pos%22%3A%7B%22line%22%3A39%2C%22character%22%3A32%7D%7D%2C%7B%22uri%22%3A%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2Fhome%2Fluis%2Fcoding%2Frust%2Fdep-inspect-rs%2Fsrc%2Fcontrollers%2Ffile_package.rs%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%22pos%22%3A%7B%22line%22%3A7%2C%22character%22%3A14%7D%7D%2C%7B%22uri%22%3A%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2Fhome%2Fluis%2Fcoding%2Frust%2Fdep-inspect-rs%2FCargo.toml%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%22pos%22%3A%7B%22line%22%3A7%2C%22character%22%3A0%7D%7D%5D%2C%220e8176d5-e6b2-454e-b5a9-c4c5127df528%22%5D "Go to definition") para manejar JSON.
-   [`reqwest`](command:_github.copilot.openSymbolFromReferences?%5B%22%22%2C%5B%7B%22uri%22%3A%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2Fhome%2Fluis%2Fcoding%2Frust%2Fdep-inspect-rs%2FCargo.toml%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%22pos%22%3A%7B%22line%22%3A8%2C%22character%22%3A0%7D%7D%5D%2C%220e8176d5-e6b2-454e-b5a9-c4c5127df528%22%5D "Go to definition") para realizar solicitudes HTTP.
-   [`tokio`](command:_github.copilot.openSymbolFromReferences?%5B%22%22%2C%5B%7B%22uri%22%3A%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2Fhome%2Fluis%2Fcoding%2Frust%2Fdep-inspect-rs%2Fsrc%2Fcontrollers%2Ffile_remote.rs%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%22pos%22%3A%7B%22line%22%3A29%2C%22character%22%3A37%7D%7D%2C%7B%22uri%22%3A%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2Fhome%2Fluis%2Fcoding%2Frust%2Fdep-inspect-rs%2FCargo.toml%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%22pos%22%3A%7B%22line%22%3A10%2C%22character%22%3A0%7D%7D%5D%2C%220e8176d5-e6b2-454e-b5a9-c4c5127df528%22%5D "Go to definition") para manejar tareas asíncronas.
-   [`futures`](command:_github.copilot.openSymbolFromReferences?%5B%22%22%2C%5B%7B%22uri%22%3A%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2Fhome%2Fluis%2Fcoding%2Frust%2Fdep-inspect-rs%2FCargo.toml%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%22pos%22%3A%7B%22line%22%3A11%2C%22character%22%3A0%7D%7D%5D%2C%220e8176d5-e6b2-454e-b5a9-c4c5127df528%22%5D "Go to definition") para manejar futuros.
-   [`clap`](command:_github.copilot.openSymbolFromReferences?%5B%22%22%2C%5B%7B%22uri%22%3A%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2Fhome%2Fluis%2Fcoding%2Frust%2Fdep-inspect-rs%2FCargo.toml%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%2C%22pos%22%3A%7B%22line%22%3A12%2C%22character%22%3A0%7D%7D%5D%2C%220e8176d5-e6b2-454e-b5a9-c4c5127df528%22%5D "Go to definition") para manejar argumentos de línea de comandos.

## Contribuir

Si deseas contribuir a este proyecto, por favor sigue los siguientes pasos:

1. Haz un fork del repositorio.
2. Crea una nueva rama (`git checkout -b feature/nueva-funcionalidad`).
3. Realiza tus cambios y haz commit (`git commit -am 'Añadir nueva funcionalidad'`).
4. Sube tus cambios a tu fork (`git push origin feature/nueva-funcionalidad`).
5. Abre un Pull Request.

## Licencia

Este proyecto está licenciado bajo la Licencia MIT. Para más detalles, consulta el archivo `LICENSE`.

## Contacto

Para cualquier pregunta o sugerencia, por favor abre un issue en el repositorio o contacta al autor a través de su perfil de GitHub.
