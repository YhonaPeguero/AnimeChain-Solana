use anchor_lang::prelude::*;
// ID del programa en Solana. Se llena automáticamente al ejecutar "anchor build"
declare_id!("8gf8aXyegsBcfci44GSnjRjz8krg4w56PrssVvesh6FP");

#[program] // Macro que convierte el código Rust a instrucciones entendibles por Solana
pub mod gestor_animes {
    use super::*; // Importa todos los structs y enums definidos fuera de este módulo

    //////////////////////////// Instrucción: Crear Gestor /////////////////////////////////////
    /*
    Crea una PDA (Program Derived Address) — una cuenta especial en Solana que no necesita
    llave privada para firmar transacciones. Es como una "caja fuerte" que el programa controla.

    Dentro de esta PDA se guardará el struct GestorAnimes, que contendrá la lista de animes.

    La PDA se genera de forma única a partir de:
        * La dirección del wallet del usuario (owner)
        * El ID del programa
        * Un string fijo: "gestor_animes"

    Esto garantiza que cada usuario tenga SU PROPIO gestor de animes único.

    Parámetros de entrada:
        * nombre -> Nombre del gestor (ej: "Mi Colección Favorita") -> String
    */
    pub fn crear_gestor(context: Context<NuevoGestor>, nombre: String) -> Result<()> {
        // Obtenemos la dirección pública (wallet address) de quien llama la instrucción
        let owner_id = context.accounts.owner.key();
        msg!("Gestor creado por: {}", owner_id); // Log de verificación en la transacción

        // Creamos un vector vacío donde se irán guardando los animes
        let animes: Vec<Anime> = Vec::new();

        // Inicializamos el struct GestorAnimes y lo guardamos en la cuenta PDA
        context.accounts.gestor.set_inner(GestorAnimes {
            owner: owner_id, // El dueño del gestor
            nombre,           // El nombre del gestor
            animes,           // Lista de animes (vacía al inicio)
        });

        Ok(()) // Transacción exitosa ✅
    }

    //////////////////////////// Instrucción: Agregar Anime /////////////////////////////////////
    /*
    Agrega un nuevo anime al vector de animes dentro del GestorAnimes.

    El contexto utilizado es ModificarAnime, que da acceso tanto al owner como al gestor.
    Solo el dueño del gestor puede agregar animes (verificación con require!).

    Parámetros de entrada:
        * titulo   -> Nombre del anime (ej: "Naruto") -> String
        * episodios -> Número total de episodios -> u16 (número entre 0 y 65,535)
        * imagen   -> URL de la imagen/portada del anime -> String
        * enlace   -> URL donde se puede ver el anime -> String
    */
    pub fn agregar_anime(
        context: Context<ModificarAnime>,
        titulo: String,
        episodios: u16,
        imagen: String,
        enlace: String,
    ) -> Result<()> {
        // 🔐 Seguridad: verificamos que quien llama sea el dueño del gestor
        // Si no lo es, la transacción falla con el error NoEresElOwner
        require!(
            context.accounts.gestor.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // Construimos el struct Anime con los datos recibidos
        let anime = Anime {
            titulo,
            episodios,
            imagen,
            enlace,
            favorito: false, // Por defecto, el anime no está marcado como favorito
        };

        // Añadimos el anime al vector dentro del gestor
        context.accounts.gestor.animes.push(anime);
        msg!("Anime agregado exitosamente!");

        Ok(()) // Transacción exitosa ✅
    }

    //////////////////////////// Instrucción: Eliminar Anime /////////////////////////////////////
    /*
    Busca un anime por su título y lo elimina del vector.
    Devuelve error si el anime no existe o si el vector está vacío.

    Solo el dueño del gestor puede eliminar animes.

    Parámetros de entrada:
        * titulo -> Título del anime a eliminar -> String
    */
    pub fn eliminar_anime(context: Context<ModificarAnime>, titulo: String) -> Result<()> {
        // 🔐 Verificación de identidad del owner
        require!(
            context.accounts.gestor.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // Obtenemos una referencia mutable al vector de animes para poder modificarlo
        let animes = &mut context.accounts.gestor.animes;

        // Recorremos el vector buscando el anime por su título
        for i in 0..animes.len() {
            if animes[i].titulo == titulo {
                animes.remove(i); // Eliminamos el anime del vector usando su índice
                msg!("Anime '{}' eliminado correctamente!", titulo);
                return Ok(()); // Transacción exitosa ✅
            }
        }

        // Si llegamos aquí, el anime no fue encontrado
        Err(Errores::AnimeNoExiste.into())
    }

    //////////////////////////// Instrucción: Ver Animes /////////////////////////////////////
    /*
    Imprime en el log de la transacción la lista completa de animes guardados.
    Útil para depuración y verificación desde el cliente.

    Solo el dueño puede consultar su gestor.

    Parámetros de entrada:
        Ninguno
    */
    pub fn ver_animes(context: Context<ModificarAnime>) -> Result<()> {
        // 🔐 Verificación de identidad del owner
        require!(
            context.accounts.gestor.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // {:#?} muestra el contenido completo del vector de forma legible en el log
        // Requiere que el struct Anime tenga el atributo #[derive(Debug)]
        msg!(
            "Lista completa de animes: {:#?}",
            context.accounts.gestor.animes
        );

        Ok(()) // Transacción exitosa ✅
    }

    //////////////////////////// Instrucción: Alternar Favorito /////////////////////////////////////
    /*
    Cambia el estado del campo "favorito" de un anime:
        - Si era false → pasa a true  (marcado como favorito ⭐)
        - Si era true  → pasa a false (quitado de favoritos)

    Solo el dueño puede marcar/desmarcar favoritos.

    Parámetros de entrada:
        * titulo -> Título del anime cuyo estado se quiere alternar -> String
    */
    pub fn alternar_favorito(context: Context<ModificarAnime>, titulo: String) -> Result<()> {
        // 🔐 Verificación de identidad del owner
        require!(
            context.accounts.gestor.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // Referencia mutable al vector de animes
        let animes = &mut context.accounts.gestor.animes;

        // Buscamos el anime por título y alternamos su estado "favorito"
        for i in 0..animes.len() {
            if animes[i].titulo == titulo {
                let nuevo_estado = !animes[i].favorito; // Invertimos el valor booleano
                animes[i].favorito = nuevo_estado;
                msg!(
                    "El anime '{}' ahora tiene favorito = {}",
                    titulo,
                    nuevo_estado
                );
                return Ok(()); // Transacción exitosa ✅
            }
        }

        // Anime no encontrado en el vector
        Err(Errores::AnimeNoExiste.into())
    }

    //////////////////////////// Instrucción: Actualizar Enlace /////////////////////////////////////
    /*
    Permite actualizar la URL de visualización o la imagen de portada de un anime existente.
    Útil si el enlace cambia o si queremos corregir la imagen.

    Solo el dueño puede hacer esta actualización.

    Parámetros de entrada:
        * titulo      -> Título del anime a actualizar -> String
        * nuevo_enlace -> Nueva URL de visualización -> String
        * nueva_imagen -> Nueva URL de imagen/portada -> String
    */
    pub fn actualizar_enlaces(
        context: Context<ModificarAnime>,
        titulo: String,
        nuevo_enlace: String,
        nueva_imagen: String,
    ) -> Result<()> {
        // 🔐 Verificación de identidad del owner
        require!(
            context.accounts.gestor.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // Referencia mutable al vector de animes
        let animes = &mut context.accounts.gestor.animes;

        // Buscamos el anime y actualizamos sus URLs
        for i in 0..animes.len() {
            if animes[i].titulo == titulo {
                animes[i].enlace = nuevo_enlace.clone(); // Actualizamos el enlace de visualización
                animes[i].imagen = nueva_imagen.clone(); // Actualizamos la imagen de portada
                msg!("Anime '{}' actualizado con nuevos enlaces!", titulo);
                return Ok(()); // Transacción exitosa ✅
            }
        }

        // Anime no encontrado
        Err(Errores::AnimeNoExiste.into())
    }
}

//////////////////////////// Códigos de Error /////////////////////////////////////
/*
Enum con todos los posibles errores del programa.
Cada variante lleva un mensaje descriptivo que se muestra cuando ocurre el error.

Estructura:
    #[msg("Mensaje de error visible al usuario")]
    NombreDelError,
*/
#[error_code]
pub enum Errores {
    #[msg("Error: no eres el propietario del gestor que deseas modificar")]
    NoEresElOwner,

    #[msg("Error: el anime que buscas no existe en tu gestor")]
    AnimeNoExiste,
}

//////////////////////////// Struct: GestorAnimes /////////////////////////////////////
/*
Cuenta principal que se almacena en la blockchain como PDA.
Contiene todos los datos del gestor de animes de un usuario.

Atributos importantes:
    - #[account]     → Le indica a Anchor que este struct ES una cuenta en Solana
    - #[derive(InitSpace)] → Calcula automáticamente el espacio necesario en la blockchain
*/
#[account]
#[derive(InitSpace)]
pub struct GestorAnimes {
    /// Dirección pública del dueño del gestor (32 bytes, formato de llave pública de Solana)
    owner: Pubkey,

    /// Nombre personalizado del gestor (máximo 60 caracteres)
    #[max_len(60)]
    nombre: String,

    /// Lista de animes guardados (máximo 10 animes en el gestor)
    #[max_len(10)]
    animes: Vec<Anime>,
}

//////////////////////////// Struct: Anime /////////////////////////////////////
/*
Struct secundario (NO es una cuenta, es un dato dentro de GestorAnimes).
Representa un anime individual con todos sus campos.

Atributos derive necesarios:
    - AnchorSerialize   → Para poder guardar el struct en la cuenta de Solana
    - AnchorDeserialize → Para poder leer el struct desde la cuenta de Solana
    - Clone             → Para copiar sus valores cuando sea necesario
    - InitSpace         → Para que Anchor calcule cuánto espacio ocupa en la blockchain
    - PartialEq         → Para comparar structs con "=="
    - Debug             → Para poder mostrarlo en logs con "{:?}" o "{:#?}"
*/
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Anime {
    /// Título del anime (ej: "Attack on Titan") — máximo 100 caracteres
    #[max_len(100)]
    titulo: String,

    /// Número total de episodios del anime (u16 = número entre 0 y 65,535)
    episodios: u16,

    /// URL de la imagen o portada del anime (ej: "https://img.example.com/aot.jpg") — máximo 200 caracteres
    #[max_len(200)]
    imagen: String,

    /// URL donde se puede ver el anime (ej: "https://crunchyroll.com/aot") — máximo 200 caracteres
    #[max_len(200)]
    enlace: String,

    /// Indica si el anime está marcado como favorito (true = favorito ⭐, false = normal)
    favorito: bool,
}

//////////////////////////// Contexto: NuevoGestor /////////////////////////////////////
/*
Define las cuentas necesarias para la instrucción "crear_gestor".
Anchor valida automáticamente que estas cuentas existan y sean correctas.
*/
#[derive(Accounts)]
pub struct NuevoGestor<'info> {
    /// El owner es quien firma y paga la transacción (mut porque su balance de SOL cambia)
    #[account(mut)]
    pub owner: Signer<'info>,

    /// La PDA que se creará para almacenar el GestorAnimes
    #[account(
        init,       // Indica que esta cuenta se creará nueva al llamar la instrucción
        payer = owner, // El owner paga el costo de creación (rent en SOL)
        space = GestorAnimes::INIT_SPACE + 8, // Espacio calculado + 8 bytes del discriminador de Anchor
        seeds = [b"gestor_animes", owner.key().as_ref()], // Semillas únicas: string fijo + wallet del owner
        bump        // Anchor busca automáticamente el "bump" que hace válida la PDA
    )]
    pub gestor: Account<'info, GestorAnimes>,

    /// Programa del sistema de Solana, necesario para crear nuevas cuentas
    pub system_program: Program<'info, System>,
}

//////////////////////////// Contexto: ModificarAnime /////////////////////////////////////
/*
Define las cuentas necesarias para todas las instrucciones que leen o modifican animes:
agregar_anime, eliminar_anime, ver_animes, alternar_favorito, actualizar_enlaces.

No necesita "init" porque el gestor ya existe, solo lo lee o modifica.
*/
#[derive(Accounts)]
pub struct ModificarAnime<'info> {
    /// El owner firma la transacción para verificar su identidad
    pub owner: Signer<'info>,

    /// El gestor de animes existente — se marca "mut" porque sus datos pueden cambiar
    #[account(mut)]
    pub gestor: Account<'info, GestorAnimes>,
}