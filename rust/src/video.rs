use anyhow::{anyhow, Result};
use opencv::{
    core::{Point2f, Size},
    imgcodecs,
    imgproc,
    prelude::*,
    videoio,
};

pub fn video_main(args: &[String]) -> Result<()> {
    // Espera: programa v <imagem> <saida.mp4> <duracao_segundos>
    if args.len() < 5 {
        return Err(anyhow!(
            "Uso: {} v <imagem> <saida> <duracao_segundos>",
            args[0]
        ));
    }

    let caminho_imagem = &args[2];
    let caminho_video = &args[3];
    let duracao: f64 = args[4].parse()?; // segundos

    let fps = 30.0;
    let total_frames = (fps * duracao) as i32;

    // Carregar imagem
    let img = imgcodecs::imread(caminho_imagem, imgcodecs::IMREAD_COLOR)?;
    if img.empty() {
        return Err(anyhow!("Erro ao carregar imagem {}", caminho_imagem));
    }

    let largura = img.cols();
    let altura = img.rows();

    // Escolher codec (ex.: MJPG ou mp4v)
    // MJPG → .avi
    // MP4V → .mp4
    // XVID → .avi
    let fourcc = videoio::VideoWriter::fourcc('M' , 'J' , 'P' , 'G' )?; 
    // ou: let fourcc = videoio::VideoWriter::fourcc('m' as i8, 'p' as i8, '4' as i8, 'v' as i8)?;

    let mut writer = videoio::VideoWriter::new(
        caminho_video,
        fourcc,
        fps,
        Size::new(largura, altura),
        true,
    )?;

    if !writer.is_opened()? {
        return Err(anyhow!("Erro ao abrir VideoWriter para {}", caminho_video));
    }

    for i in 0..total_frames {
        let angulo = 360.0 * (i as f64) / (total_frames as f64);

        let centro = Point2f::new(largura as f32 / 2.0, altura as f32 / 2.0);
        let m = imgproc::get_rotation_matrix_2d(centro, angulo, 1.0)?;

        let mut frame_rotacionado = Mat::default();
        imgproc::warp_affine(
            &img,
            &mut frame_rotacionado,
            &m,
            Size::new(largura, altura),
            imgproc::INTER_LINEAR,
            opencv::core::BORDER_CONSTANT,
            opencv::core::Scalar::default(),
        )?;

        writer.write(&frame_rotacionado)?;

        print!(
            "\rFrame {}/{} - Ângulo: {:.2}°",
            i + 1,
            total_frames,
            angulo
        );
        use std::io::Write;
        std::io::stdout().flush().ok();
    }

    println!("\nVídeo gerado com sucesso: {}", caminho_video);

    Ok(())
}

