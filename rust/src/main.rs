mod rotaciona;
mod video;

use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso: {} <v|r> ...", args[0]);
        std::process::exit(1);
    }

    let modo = args[1].chars().next().unwrap_or(' ');

    match modo {
        'v' | 'V' => video::video_main(&args)?,
        'r' | 'R' => rotaciona::rotaciona_main(&args)?,
        _ => {
            eprintln!("Primeiro parâmetro deve ser 'v' (vídeo) ou 'r' (rotação).");
            std::process::exit(1);
        }
    }

    Ok(())
}
