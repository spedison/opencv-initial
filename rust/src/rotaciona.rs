use opencv::core::Vector;

use anyhow::{anyhow, Result};

use opencv::{
    core::{Point2f, Size},
    imgcodecs,
    imgproc,
    prelude::*,
};

pub fn rotaciona_main(args: &[String]) -> Result<()> {
    // Espera: programa r <imagem> <angulo>
    if args.len() < 4 {
        return Err(anyhow!(
            "Uso: {} r <imagem> <angulo>",
            args[0]
        ));
    }

    let caminho = &args[2];
    let angulo: f64 = args[3].parse()?;

    // 1) Carregar imagem
    let img = imgcodecs::imread(caminho, imgcodecs::IMREAD_COLOR)?;
    if img.empty() {
        return Err(anyhow!("Erro ao carregar imagem {}", caminho));
    }

    let largura = img.cols();
    let altura = img.rows();

    // 2) Calcular centro
    let centro = Point2f::new(largura as f32 / 2.0, altura as f32 / 2.0);

    // 3) Matriz de rotação
    let m = imgproc::get_rotation_matrix_2d(centro, angulo, 1.0)?;

    // 4) Aplicar rotação
    let mut rotacionada = Mat::default();
    imgproc::warp_affine(
        &img,
        &mut rotacionada,
        &m,
        Size::new(largura, altura),
        imgproc::INTER_LINEAR,
        opencv::core::BORDER_CONSTANT,
        opencv::core::Scalar::default(),
    )?;

    // 5) Salvar resultado
    let saida = "imagem_rotacionada.png";
    imgcodecs::imwrite(saida, &rotacionada, &Vector::<i32>::new())?;

    println!("Imagem rotacionada gerada: {}", saida);

    Ok(())
}

