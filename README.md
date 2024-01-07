# metallum_reviews_scan

Busca términos entre todas las reviews de Metal Archives
## Instalación y ejecución
El programa no tiene dependencias externas, solamente es necesario descargar el binario y ejecutarlo. **Importante**: no olvides descargar la base de datos. Ese archivo ZIP debe descomprimirse en el mismo directorio que el ejecutable. De esta forma, *metallum_reviews_scan* y *metallum_reviews.sqlite3* deben estar en la misma carpeta.

El programa se ejecuta como una aplicación web en el puerto 8080. En el caso de que que se este usando ese puerto se puede asignar otro a través del argumento --port

```bash
./metallum_reviews_scan --port 8081
```
### Versión de la base de datos
Versión v.0.1 (datos actualizados hasta el 01/04/2024)

[Descarga desde Google Drive](https://drive.google.com/file/d/1GBecghRmjjuDKLlPasX38ctnZr3V0XuD/view?usp=sharing)

### Repositorio
[GitHub](https://github.com/antikorps/metallum_reviews_scan)