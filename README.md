# 🎌 Gestor de Animes — Solana Program (Anchor)

Un programa on-chain construido con **Anchor Framework** sobre la red de **Solana** que permite a cada usuario gestionar su propia lista de animes de forma segura y descentralizada.

---

## 🧠 ¿Cómo funciona?

Cada usuario que interactúa con el programa obtiene una **PDA (Program Derived Address)** única — una cuenta especial en la blockchain que actúa como su base de datos personal de animes. Esta cuenta es controlada exclusivamente por el programa y solo puede ser modificada por su dueño (owner).

```
Wallet del usuario + Program ID + "gestor_animes" = PDA única por usuario
```

---

## 📦 Estructura de datos

### `GestorAnimes` *(cuenta PDA)*
| Campo | Tipo | Descripción |
|-------|------|-------------|
| `owner` | `Pubkey` | Wallet address del dueño |
| `nombre` | `String` | Nombre del gestor (máx. 60 chars) |
| `animes` | `Vec<Anime>` | Lista de animes (máx. 10) |

### `Anime` *(struct interno)*
| Campo | Tipo | Descripción |
|-------|------|-------------|
| `titulo` | `String` | Título del anime (máx. 100 chars) |
| `episodios` | `u16` | Número total de episodios |
| `imagen` | `String` | URL de la portada (máx. 200 chars) |
| `enlace` | `String` | URL de visualización (máx. 200 chars) |
| `favorito` | `bool` | Marcado como favorito ⭐ o no |

---

## ⚙️ Instrucciones

### 1️⃣ `crear_gestor(nombre)`
Crea la cuenta PDA del usuario en la blockchain. **Debe llamarse primero**, antes de cualquier otra instrucción.

```
nombre: "Mi Colección Favorita"
```

---

### 2️⃣ `agregar_anime(titulo, episodios, imagen, enlace)`
Agrega un anime a la lista. Solo el owner puede ejecutar esta instrucción.

```
titulo:    "Mato Seihei no Slave"
episodios: 12
imagen:    "https://ejemplo.com/imagen.jpg"
enlace:    "https://crunchyroll.com/mato-seihei"
```

---

### 3️⃣ `eliminar_anime(titulo)`
Elimina un anime buscándolo por título. Error si no existe.

```
titulo: "Mato Seihei no Slave"
```

---

### 4️⃣ `ver_animes()`
Imprime en el log de la transacción la lista completa de animes guardados. No recibe parámetros.

---

### 5️⃣ `alternar_favorito(titulo)`
Cambia el estado `favorito` de un anime. Si era `false` → `true` ⭐, y viceversa.

```
titulo: "Fate/Strange Fake"
```

---

### 6️⃣ `actualizar_enlaces(titulo, nuevo_enlace, nueva_imagen)`
Actualiza la URL de visualización y/o la imagen de portada de un anime existente.

```
titulo:       "Dead Account"
nuevo_enlace: "https://nuevo-enlace.com"
nueva_imagen: "https://nueva-imagen.com/portada.jpg"
```

---

## 🔐 Seguridad

Todas las instrucciones de escritura incluyen una validación con `require!` que verifica que **quien firma la transacción sea el mismo owner que creó el gestor**. Si no coincide, la transacción falla con el error `NoEresElOwner`.

---

## ❌ Códigos de Error

| Error | Descripción |
|-------|-------------|
| `NoEresElOwner` | Quien llama la instrucción no es el dueño del gestor |
| `AnimeNoExiste` | El título buscado no existe en la lista |

---

## 🚀 Flujo de uso recomendado

```
1. crear_gestor()      → Inicializa tu PDA en la blockchain
2. agregar_anime()     → Agrega tus animes (hasta 10)
3. ver_animes()        → Verifica el contenido en el log
4. alternar_favorito() → Marca tus favoritos ⭐
5. actualizar_enlaces()→ Corrige URLs si es necesario
6. eliminar_anime()    → Limpia tu lista cuando quieras
```

---

## 🛠️ Stack

- **Blockchain:** [Solana](https://solana.com/)
- **Framework:** [Anchor](https://www.anchor-lang.com/)
- **Lenguaje:** [Rust](https://rust-lang.org/learn/)

---

## 👨‍💻 Autor

Creado por **Yhonatan Peguero**

[![GitHub](https://img.shields.io/badge/GitHub-YhonaPeguero-181717?style=flat&logo=github)](https://github.com/YhonaPeguero)
[![LinkedIn](https://img.shields.io/badge/LinkedIn-yhonatan--peguero-0A66C2?style=flat&logo=linkedin)](https://www.linkedin.com/in/yhonatan-peguero/)
[![Twitter](https://img.shields.io/badge/Twitter-thisnotmeeme-1DA1F2?style=flat&logo=x)](https://x.com/thisnotmeeme)