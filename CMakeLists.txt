cmake_minimum_required(VERSION 3.10)
project(projeto_rotacao)

set(CMAKE_CXX_STANDARD 17)
find_package(OpenCV REQUIRED)

include_directories(${OpenCV_INCLUDE_DIRS})
include_directories(src)

# Biblioteca para funções de rotação
add_library(rotaciona_lib src/rotaciona_main.cpp)

# Biblioteca para funções de vídeo
add_library(video_lib src/video_main.cpp)

# Programa principal (chama as bibliotecas)
add_executable(main src/main.cpp)
target_link_libraries(main rotaciona_lib video_lib ${OpenCV_LIBS})

# Programa que só faz rotação
#add_executable(rotacionar src/rotaciona_main.cpp)
#target_link_libraries(rotacionar rotaciona_lib ${OpenCV_LIBS})

# Programa que gera vídeo
#add_executable(video_rotacao src/video_main.cpp)
#target_link_libraries(video_rotacao video_lib ${OpenCV_LIBS})

