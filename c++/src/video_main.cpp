#include "video_main.hpp" 

int video_main(int argc, char** argv) {
    if (argc < 5) {
        std::cout << "Uso: ./" << argv[0] << " v  <imagem> <saida.mp4> <duracao_segundos>\n";
        return 1;
    }

    std::string caminhoImagem = argv[2];
    std::string caminhoVideo = argv[3];
    double duracao = std::stod(argv[4]); // em segundos

    int fps = 30;
    int totalFrames = static_cast<int>(fps * duracao);

    // Carregar imagem
    cv::Mat img = cv::imread(caminhoImagem);
    if (img.empty()) {
        std::cout << "Erro ao carregar imagem.\n";
        return 1;
    }

    int largura = img.cols;
    int altura = img.rows;

    // Inicializar o writer
    cv::VideoWriter writer(
        caminhoVideo,
        cv::VideoWriter::fourcc('M','J','P','G'),
        fps,
        cv::Size(largura, altura)
    );

    if (!writer.isOpened()) {
        std::cout << "Erro ao abrir VideoWriter.\n";
        return 1;
    }

    // Gerar os frames
    for (int i = 0; i < totalFrames; i++) {
        double angulo = (360.0 * i) / totalFrames;

        // Matriz de rotação
        cv::Point2f centro(largura / 2.0f, altura / 2.0f);
        cv::Mat M = cv::getRotationMatrix2D(centro, angulo, 1.0);

        cv::Mat frameRotacionado;
        cv::warpAffine(img, frameRotacionado, M, img.size());

        // Escrever no vídeo
        writer.write(frameRotacionado);

        std::cout << "\rFrame " << (i+1) << "/" << totalFrames << " - Ângulo: " << angulo << "°" << std::flush;
    }

    std::cout << "\nVídeo gerado com sucesso: " << caminhoVideo << "\n";

    return 0;
}

