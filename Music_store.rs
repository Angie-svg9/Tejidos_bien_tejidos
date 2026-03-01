use anchor_lang::prelude::*;

declare_id!("GbYdmq8itoAjpmxWthKC3GeFDpLoWgmTTpcGM4vwVCAX");

#[program]
pub mod tienda_musica {
    use super::*;

    pub fn crear_tienda(context: Context<NuevaTienda>, nombre: String) -> Result<()> {
        let owner_id = context.accounts.owner.key();
        let canciones: Vec<Cancion> = Vec::new();

        context.accounts.tienda.set_inner(Tienda {
            owner: owner_id,
            nombre,
            canciones,
        });

        Ok(())
    }

    pub fn agregar_cancion(
        context: Context<NuevaCancion>, 
        artista: String, 
        nombre: String, 
        album: String, 
        duracion: u16
    ) -> Result<()> {
        let cancion = Cancion {
            artista,
            nombre,
            album,
            duracion,
            disponible: true,
        };

        context.accounts.tienda.canciones.push(cancion);

        Ok(())
    }

    pub fn ver_canciones(context: Context<NuevaCancion>) -> Result<()> {
        let canciones = &context.accounts.tienda.canciones;
        msg!("Lista de canciones: {:#?}", canciones);
        Ok(())
    }

    pub fn eliminar_cancion(context: Context<NuevaCancion>, nombre: String) -> Result<()> {
        let canciones = &mut context.accounts.tienda.canciones;

        if let Some(pos) = canciones.iter().position(|c| c.nombre == nombre) {
            canciones.remove(pos);
            msg!("Canción {} eliminada", nombre);
            return Ok(());
        }

        Err(Errores::CancionNoExiste.into())
    }

    pub fn alternar_estado(context: Context<NuevaCancion>, nombre: String) -> Result<()> {
        let canciones = &mut context.accounts.tienda.canciones;

        if let Some(cancion) = canciones.iter_mut().find(|c| c.nombre == nombre) {
            cancion.disponible = !cancion.disponible;
            msg!("La canción: {}, ahora está disponible: {}", nombre, cancion.disponible);
            return Ok(());
        }

        Err(Errores::CancionNoExiste.into())
    }
}

#[error_code]
pub enum Errores {
    #[msg("Error, Canción no existe")]
    CancionNoExiste,

    #[msg("Error, no eres el propietario de la tienda")]
    NoEresElOwner,
}

#[account]
#[derive(InitSpace)]
pub struct Tienda {
    owner: Pubkey,

    #[max_len(60)]
    nombre: String,

    #[max_len(50)]
    canciones: Vec<Cancion>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Cancion {
    #[max_len(60)]
    artista: String,

    #[max_len(60)]
    nombre: String,

    #[max_len(60)]
    album: String,

    duracion: u16,

    disponible: bool,
}

#[derive(Accounts)]
pub struct NuevaTienda<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Tienda::INIT_SPACE + 8,
        seeds = [b"tienda", owner.key().as_ref()],
        bump
    )]
    pub tienda: Account<'info, Tienda>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NuevaCancion<'info> {
    pub owner: Signer<'info>,
    #[account(mut)]
    pub tienda: Account<'info, Tienda>,
}
