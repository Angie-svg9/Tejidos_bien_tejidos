// --- CONFIGURACIÓN MANUAL: CAMBIA LOS DATOS AQUÍ ---
const ACCION = "AGREGAR"; // Opciones: "AGREGAR", "VER", "ELIMINAR"

const DATOS_CANCION = {
  artista: "Olivia Rodrigo",
  nombre: "Vampire",
  album: "GUTS",
  duracion: 339
};

const NOMBRE_PARA_ELIMINAR = "canción"; // Nombre exacto para borrar
// ---------------------------------------------------

const [tiendaPda] = web3.PublicKey.findProgramAddressSync(
  [Buffer.from("tienda"), pg.wallet.publicKey.toBuffer()],
  pg.program.programId
);

async function ejecutarCliente() {
  console.log("--- GESTOR DE TIENDA MUSICAL (Modo Config) ---");

  try {
    const tienda = await pg.program.account.tienda.fetch(tiendaPda);
    console.log(`ℹ️ Tienda: "${tienda.nombre}"`);

    if (ACCION === "AGREGAR") {
      console.log(`Enviando: ${DATOS_CANCION.nombre}...`);
      const tx = await pg.program.methods
        .agregarCancion(
          DATOS_CANCION.artista,
          DATOS_CANCION.nombre,
          DATOS_CANCION.album,
          DATOS_CANCION.duracion
        )
        .accounts({
          owner: pg.wallet.publicKey,
          tienda: tiendaPda,
        }).rpc();
      console.log(`✅ ¡Agregada! TX: ${tx}`);

    } else if (ACCION === "VER") {
      const data = await pg.program.account.tienda.fetch(tiendaPda);
      console.log("\n🎵 --- LISTA DE CANCIONES ---");
      data.canciones.forEach((c, i) => {
        console.log(`${i + 1}. ${c.nombre} - ${c.artista} (${c.album})`);
      });

    } else if (ACCION === "ELIMINAR") {
      console.log(`Eliminando: ${NOMBRE_PARA_ELIMINAR}...`);
      const tx = await pg.program.methods
        .eliminarCancion(NOMBRE_PARA_ELIMINAR)
        .accounts({
          owner: pg.wallet.publicKey,
          tienda: tiendaPda,
        }).rpc();
      console.log(`✅ Eliminada. TX: ${tx}`);
    }

  } catch (e) {
    console.error("❌ Error:", e.message);
    console.log("Tip: Si la tienda no existe, asegúrate de haber corrido el test primero.");
  }
}

await ejecutarCliente();
