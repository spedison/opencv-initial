#include "rotaciona_main.hpp" 


int rotaciona_main(int argc, char** argv) {
    if (argc < 3) {
        std::cout << "Uso: ./" << argv[0] << " r <imagem> <angulo>\n";
        return 1;
    }

    std::string caminho = argv[2];
    double angulo = std::stod(argv[3]);

    // 1) Carregar imagem
    cv::Mat img = cv::imread(caminho);
    if (img.empty()) {
        std::cout << "Erro ao carregar imagem.\n";
        return 1;
    }

    // 2) Calcular centro
    cv::Point2f centro(img.cols / 2.0f, img.rows / 2.0f);

    // 3) Obter matriz de rotação
    cv::Mat M = cv::getRotationMatrix2D(centro, angulo, 1.0);

    // 4) Aplicar rotação
    cv::Mat rotacionada;
    cv::warpAffine(img, rotacionada, M, img.size());

    // 5) Salvar resultado
    cv::imwrite("imagem_rotacionada.png", rotacionada);

    std::cout << "Imagem rotacionada gerada: imagem_rotacionada.png\n";

    return 0;
}

